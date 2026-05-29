//! Port of `Dh_NetClient/ticking/RowSequenceDecoder.cs` (plus the supporting
//! `container/RowSequence.cs` and `container/Interval.cs`).
//!
//! This is the Barrage RowSet "external compressed delta" codec — the source of
//! truth for row-key decoding. Everything in the ticking path builds on it, so
//! the semantics here mirror the C# reference exactly:
//!
//! * Values are little-endian and signed.
//! * A buffered ("pending") value is flushed as a *singleton* when the next
//!   value is positive, or as the *end* of an interval when the next value is
//!   negative.
//! * `shift_data` is three RowSequences read sequentially from ONE buffer using
//!   the same cursor — see [`DataInput`] reuse in the subscription path.

use crate::error::{Error, Result};

/// A half-open interval `[begin, end)` of row keys.
///
/// Port of `Interval.cs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub begin: u64,
    pub end: u64,
}

impl Interval {
    pub const EMPTY: Interval = Interval { begin: 0, end: 0 };

    pub fn of(begin: u64, end: u64) -> Interval {
        Interval { begin, end }
    }

    pub fn of_singleton(value: u64) -> Interval {
        Interval { begin: value, end: value + 1 }
    }

    pub fn count(&self) -> u64 {
        self.end - self.begin
    }

    pub fn is_empty(&self) -> bool {
        self.begin == self.end
    }
}

/// An ordered, de-duplicated set of row keys stored as merged half-open
/// intervals. Port of `RowSequence` / `BasicRowSequence` in `RowSequence.cs`.
///
/// Like the C# `BasicRowSequence`, the interval array is shared (`Arc`) so that
/// `take`/`drop` produce cheap subsequences without copying.
#[derive(Debug, Clone)]
pub struct RowSequence {
    intervals: std::sync::Arc<[Interval]>,
    start_index: usize,
    start_offset: u64,
    count: u64,
}

impl RowSequence {
    /// An empty sequence.
    pub fn create_empty() -> RowSequence {
        RowSequence {
            intervals: std::sync::Arc::from(Vec::new()),
            start_index: 0,
            start_offset: 0,
            count: 0,
        }
    }

    /// Build from already-consolidated intervals (used internally by the
    /// builder; callers normally go through [`RowSequenceBuilder`]).
    fn from_consolidated(intervals: Vec<Interval>) -> RowSequence {
        let count = intervals.iter().map(|i| i.count()).sum();
        RowSequence {
            intervals: std::sync::Arc::from(intervals),
            start_index: 0,
            start_offset: 0,
            count,
        }
    }

    /// Total number of row keys in the sequence.
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// The intervals making up exactly this (sub)sequence, in order.
    ///
    /// Mirrors `BasicRowSequence.Intervals` (via the `SplitHelper` walk): the
    /// first and last interval may be partial slices of the shared array.
    pub fn intervals(&self) -> Vec<Interval> {
        let mut out = Vec::new();
        let mut idx = self.start_index;
        let mut off = self.start_offset;
        let mut remaining = self.count;
        while remaining != 0 {
            let cur = self.intervals[idx];
            let entry_count = cur.count();
            if off == entry_count {
                idx += 1;
                off = 0;
                continue;
            }
            let entry_remaining = entry_count - off;
            let amount = entry_remaining.min(remaining);
            let begin = cur.begin + off;
            let end = begin + amount;
            off += amount;
            remaining -= amount;
            out.push(Interval::of(begin, end));
        }
        out
    }

    /// All row keys, expanded. Convenience for tests and small sequences.
    pub fn elements(&self) -> Vec<u64> {
        let mut out = Vec::with_capacity(self.count as usize);
        for interval in self.intervals() {
            for k in interval.begin..interval.end {
                out.push(k);
            }
        }
        out
    }

    /// The first `size` keys (clamped). Port of `RowSequence.Take`.
    pub fn take(&self, size: u64) -> RowSequence {
        RowSequence {
            intervals: self.intervals.clone(),
            start_index: self.start_index,
            start_offset: self.start_offset,
            count: size.min(self.count),
        }
    }

    /// All but the first `size` keys (clamped). Port of `RowSequence.Drop`.
    pub fn drop(&self, size: u64) -> RowSequence {
        let mut idx = self.start_index;
        let mut off = self.start_offset;
        let mut remaining = size.min(self.count);
        let final_size = self.count - remaining;
        while remaining != 0 {
            let cur = self.intervals[idx];
            let entry_count = cur.count();
            if off == entry_count {
                idx += 1;
                off = 0;
                continue;
            }
            let entry_remaining = entry_count - off;
            let amount = entry_remaining.min(remaining);
            off += amount;
            remaining -= amount;
        }
        RowSequence {
            intervals: self.intervals.clone(),
            start_index: idx,
            start_offset: off,
            count: final_size,
        }
    }
}

/// Accumulates intervals/keys and consolidates them on `build`.
///
/// Port of `RowSequenceBuilder` in `RowSequence.cs`.
#[derive(Debug, Default)]
pub struct RowSequenceBuilder {
    intervals: Vec<Interval>,
}

impl RowSequenceBuilder {
    pub fn new() -> RowSequenceBuilder {
        RowSequenceBuilder { intervals: Vec::new() }
    }

    /// Add an interval. Need not be disjoint with existing intervals. Empty
    /// intervals are ignored.
    pub fn add_interval(&mut self, interval: Interval) {
        if interval.is_empty() {
            return;
        }
        self.intervals.push(interval);
    }

    /// Add a single key.
    pub fn add(&mut self, key: u64) {
        self.add_interval(Interval::of_singleton(key));
    }

    /// Sort, merge abutting/overlapping intervals, and produce the sequence.
    pub fn build(mut self) -> RowSequence {
        self.intervals.sort_by(|a, b| a.begin.cmp(&b.begin));

        let mut consolidated: Vec<Interval> = Vec::new();
        let mut it = self.intervals.into_iter();
        if let Some(mut last) = it.next() {
            for this in it {
                if last.end >= this.begin {
                    // abut or overlap
                    last.end = last.end.max(this.end);
                    continue;
                }
                consolidated.push(last);
                last = this;
            }
            consolidated.push(last);
        }

        RowSequence::from_consolidated(consolidated)
    }
}

// ---------------------------------------------------------------------------
// Wire-format command/value tags. Port of the `Constants` class.
// ---------------------------------------------------------------------------

mod constants {
    pub const SHORT_VALUE: u8 = 1;
    pub const INT_VALUE: u8 = 2;
    pub const LONG_VALUE: u8 = 3;
    pub const BYTE_VALUE: u8 = 4;

    pub const VALUE_MASK: u8 = 7;

    pub const OFFSET: u8 = 8;
    pub const SHORT_ARRAY: u8 = 16;
    pub const BYTE_ARRAY: u8 = 24;
    pub const END: u8 = 32;
    pub const CMD_MASK: u8 = 0x78;
}

/// A little-endian cursor over a delta buffer. Port of `DataInput`.
///
/// Crucially, this holds a persistent offset so a single instance can decode
/// several RowSequences sequentially from the same buffer (e.g. the three
/// sequences packed into `shift_data`). Do NOT re-slice the buffer between
/// reads — advance one cursor.
pub struct DataInput<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> DataInput<'a> {
    pub fn new(data: &'a [u8]) -> DataInput<'a> {
        DataInput { data, offset: 0 }
    }

    /// Bytes consumed so far (useful when chaining reads off one buffer).
    pub fn position(&self) -> usize {
        self.offset
    }

    fn read_i8(&mut self) -> Result<i8> {
        let b = *self.data.get(self.offset).ok_or(Error::UnexpectedEof)?;
        self.offset += 1;
        Ok(b as i8)
    }

    fn read_i16(&mut self) -> Result<i16> {
        let end = self.offset + 2;
        let bytes = self.data.get(self.offset..end).ok_or(Error::UnexpectedEof)?;
        let v = i16::from_le_bytes([bytes[0], bytes[1]]);
        self.offset = end;
        Ok(v)
    }

    fn read_i32(&mut self) -> Result<i32> {
        let end = self.offset + 4;
        let bytes = self.data.get(self.offset..end).ok_or(Error::UnexpectedEof)?;
        let v = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        self.offset = end;
        Ok(v)
    }

    fn read_i64(&mut self) -> Result<i64> {
        let end = self.offset + 8;
        let bytes = self.data.get(self.offset..end).ok_or(Error::UnexpectedEof)?;
        let mut arr = [0u8; 8];
        arr.copy_from_slice(bytes);
        let v = i64::from_le_bytes(arr);
        self.offset = end;
        Ok(v)
    }

    /// Read a value whose width is encoded in the low bits of `command`.
    fn read_value(&mut self, command: u8) -> Result<i64> {
        match command & constants::VALUE_MASK {
            constants::LONG_VALUE => self.read_i64(),
            constants::INT_VALUE => Ok(self.read_i32()? as i64),
            constants::SHORT_VALUE => Ok(self.read_i16()? as i64),
            constants::BYTE_VALUE => Ok(self.read_i8()? as i64),
            _ => Err(Error::BadCommand(command)),
        }
    }
}

/// Decode one RowSequence from the cursor's current position. Port of
/// `RowSequenceDecoder.ReadExternalCompressedDelta`.
///
/// Advances the cursor past the `END` command so the same [`DataInput`] can be
/// reused for the next sequence (as `shift_data` requires).
pub fn read_external_compressed_delta(input: &mut DataInput) -> Result<RowSequence> {
    let mut builder = RowSequenceBuilder::new();
    let mut offset: i64 = 0;

    // `pending == -1` is the "nothing buffered" sentinel, exactly as in C#.
    let mut pending: i64 = -1;

    // Flush logic shared by every decoded delta value.
    fn consume(builder: &mut RowSequenceBuilder, pending: &mut i64, v: i64) {
        let s = *pending;
        if s == -1 {
            *pending = v;
        } else if v < 0 {
            let begin = s as u64;
            let end = (-v) as u64 + 1;
            builder.add_interval(Interval::of(begin, end));
            *pending = -1;
        } else {
            builder.add(s as u64);
            *pending = v;
        }
    }

    loop {
        let command = input.read_i8()? as u8;
        match command & constants::CMD_MASK {
            constants::OFFSET => {
                let value = input.read_value(command)?;
                let actual = offset + value.abs();
                consume(&mut builder, &mut pending, if value < 0 { -actual } else { actual });
                offset = actual;
            }
            constants::SHORT_ARRAY => {
                let count = input.read_value(command)?;
                for _ in 0..count {
                    let sv = input.read_i16()? as i64;
                    let actual = offset + sv.abs();
                    consume(&mut builder, &mut pending, if sv < 0 { -actual } else { actual });
                    offset = actual;
                }
            }
            constants::BYTE_ARRAY => {
                let count = input.read_value(command)?;
                for _ in 0..count {
                    let bv = input.read_i8()? as i64;
                    let actual = offset + bv.abs();
                    consume(&mut builder, &mut pending, if bv < 0 { -actual } else { actual });
                    offset = actual;
                }
            }
            constants::END => {
                if pending >= 0 {
                    builder.add(pending as u64);
                }
                return Ok(builder.build());
            }
            _ => return Err(Error::BadCommand(command)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Builder / RowSequence basics ------------------------------------

    #[test]
    fn builder_merges_abutting_and_overlapping() {
        let mut b = RowSequenceBuilder::new();
        // [10,14), singleton 100, [5000,5003) -- from the RowSequence.cs doc example.
        b.add_interval(Interval::of(10, 14));
        b.add(100);
        b.add_interval(Interval::of(5000, 5003));
        // Add an overlapping/abutting interval to force a merge.
        b.add_interval(Interval::of(12, 16)); // overlaps [10,14) -> [10,16)
        b.add_interval(Interval::of(16, 20)); // abuts [10,16) -> [10,20)
        let rs = b.build();
        assert_eq!(
            rs.intervals(),
            vec![Interval::of(10, 20), Interval::of(100, 101), Interval::of(5000, 5003)]
        );
        assert_eq!(rs.count(), 10 + 1 + 3);
    }

    #[test]
    fn take_and_drop_walk_intervals() {
        let mut b = RowSequenceBuilder::new();
        b.add_interval(Interval::of(10, 14)); // 10,11,12,13
        b.add_interval(Interval::of(100, 102)); // 100,101
        let rs = b.build();
        assert_eq!(rs.elements(), vec![10, 11, 12, 13, 100, 101]);

        // take spans the first interval boundary
        assert_eq!(rs.take(5).elements(), vec![10, 11, 12, 13, 100]);
        assert_eq!(rs.take(0).elements(), Vec::<u64>::new());

        // drop into the middle of the first interval
        let dropped = rs.drop(2);
        assert_eq!(dropped.elements(), vec![12, 13, 100, 101]);
        assert_eq!(dropped.count(), 4);

        // take after drop
        assert_eq!(rs.drop(2).take(3).elements(), vec![12, 13, 100]);
    }

    // ---- Codec round-trips, hand-encoding the wire format ----------------
    //
    // The server is the encoder; there is no C# encoder to port. So we build
    // buffers by hand following the documented format and assert the decode.

    /// One OFFSET command carrying a byte-width value.
    fn offset_byte(v: i8) -> Vec<u8> {
        vec![constants::OFFSET | constants::BYTE_VALUE, v as u8]
    }

    const END: u8 = constants::END | constants::BYTE_VALUE;

    #[test]
    fn single_key() {
        // pending=5, then END flushes it as a singleton.
        let mut buf = offset_byte(5);
        buf.push(END);
        let mut input = DataInput::new(&buf);
        let rs = read_external_compressed_delta(&mut input).unwrap();
        assert_eq!(rs.elements(), vec![5]);
    }

    #[test]
    fn two_singletons_via_positive_then_positive() {
        // 5 (buffered), +3 -> actual 8 (positive flushes 5 as singleton, buffers 8),
        // END flushes 8.
        let mut buf = offset_byte(5);
        buf.extend(offset_byte(3));
        buf.push(END);
        let mut input = DataInput::new(&buf);
        let rs = read_external_compressed_delta(&mut input).unwrap();
        assert_eq!(rs.elements(), vec![5, 8]);
    }

    #[test]
    fn interval_via_negative_end() {
        // begin=5 (buffered). Next delta is negative: -3 -> actual offset 5+3=8,
        // sign negative => consume(-8) => interval [5, 8+1) = [5,9).
        let mut buf = offset_byte(5);
        buf.extend(offset_byte(-3));
        buf.push(END);
        let mut input = DataInput::new(&buf);
        let rs = read_external_compressed_delta(&mut input).unwrap();
        assert_eq!(rs.intervals(), vec![Interval::of(5, 9)]);
        assert_eq!(rs.elements(), vec![5, 6, 7, 8]);
    }

    #[test]
    fn byte_array_run_of_deltas() {
        // BYTE_ARRAY with count=3, deltas +5,+1,+1 -> keys 5,6,7 (each positive
        // flushes the previous), END flushes the last. Merges to one interval.
        let mut buf = vec![constants::BYTE_ARRAY | constants::BYTE_VALUE, 3u8, 5u8, 1u8, 1u8];
        buf.push(END);
        let mut input = DataInput::new(&buf);
        let rs = read_external_compressed_delta(&mut input).unwrap();
        assert_eq!(rs.elements(), vec![5, 6, 7]);
        assert_eq!(rs.intervals(), vec![Interval::of(5, 8)]);
    }

    #[test]
    fn short_array_and_int_value_widths() {
        // SHORT_ARRAY count=2 with two i16 deltas: +1000 then -0.
        // 1000 buffered; second delta 0 -> actual 1000, sign(0) not <0 so
        // positive: flush 1000 as singleton, buffer 1000 again; END flushes 1000.
        let mut buf = vec![constants::SHORT_ARRAY | constants::BYTE_VALUE, 2u8];
        buf.extend_from_slice(&1000i16.to_le_bytes());
        buf.extend_from_slice(&0i16.to_le_bytes());
        buf.push(END);
        let mut input = DataInput::new(&buf);
        let rs = read_external_compressed_delta(&mut input).unwrap();
        assert_eq!(rs.elements(), vec![1000]);

        // OFFSET with INT value width.
        let mut buf2 = vec![constants::OFFSET | constants::INT_VALUE];
        buf2.extend_from_slice(&70000i32.to_le_bytes());
        buf2.push(END);
        let mut input2 = DataInput::new(&buf2);
        let rs2 = read_external_compressed_delta(&mut input2).unwrap();
        assert_eq!(rs2.elements(), vec![70000]);
    }

    #[test]
    fn mixed_singletons_and_interval() {
        // 5 buffered; -3 -> interval [5,9); then +100 from offset 8 -> actual 108
        // buffered; END flushes 108.
        let mut buf = offset_byte(5);
        buf.extend(offset_byte(-3)); // interval [5,9), offset now 8
        buf.extend(offset_byte(100)); // actual 108 buffered
        buf.push(END);
        let mut input = DataInput::new(&buf);
        let rs = read_external_compressed_delta(&mut input).unwrap();
        assert_eq!(rs.intervals(), vec![Interval::of(5, 9), Interval::of(108, 109)]);
    }

    #[test]
    fn three_sequences_share_one_cursor() {
        // shift_data semantics: three sequences packed back-to-back, decoded
        // from the SAME DataInput cursor without re-slicing.
        let mut buf = offset_byte(5);
        buf.push(END); // seq1: [5]
        buf.extend(offset_byte(10));
        buf.extend(offset_byte(-2)); // 10 then -2 -> actual 12 -> interval [10,13)
        buf.push(END); // seq2
        buf.extend(offset_byte(100));
        buf.push(END); // seq3: [100]

        let mut input = DataInput::new(&buf);
        let s1 = read_external_compressed_delta(&mut input).unwrap();
        let s2 = read_external_compressed_delta(&mut input).unwrap();
        let s3 = read_external_compressed_delta(&mut input).unwrap();
        assert_eq!(s1.elements(), vec![5]);
        assert_eq!(s2.intervals(), vec![Interval::of(10, 13)]);
        assert_eq!(s3.elements(), vec![100]);
        assert_eq!(input.position(), buf.len());
    }

    #[test]
    fn empty_sequence() {
        let buf = vec![END];
        let mut input = DataInput::new(&buf);
        let rs = read_external_compressed_delta(&mut input).unwrap();
        assert!(rs.is_empty());
        assert_eq!(rs.elements(), Vec::<u64>::new());
    }

    #[test]
    fn bad_command_errors() {
        // A command with an unknown CMD_MASK pattern (0x40) is rejected.
        let buf = vec![0x40u8, END];
        let mut input = DataInput::new(&buf);
        assert!(matches!(
            read_external_compressed_delta(&mut input),
            Err(Error::BadCommand(_))
        ));
    }

    #[test]
    fn truncated_buffer_errors() {
        // OFFSET promising an i32 value but the buffer ends early.
        let buf = vec![constants::OFFSET | constants::INT_VALUE, 0x01, 0x02];
        let mut input = DataInput::new(&buf);
        assert!(matches!(
            read_external_compressed_delta(&mut input),
            Err(Error::UnexpectedEof)
        ));
    }
}
