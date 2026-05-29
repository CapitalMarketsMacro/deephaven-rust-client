//! Port of `Dh_NetClient/ticking/SpaceMapper.cs` and
//! `ticking/ShiftProcessor.cs`.
//!
//! A `SpaceMapper` maintains the set of occupied row keys (key space) and maps
//! them to dense positions (position/index space) via order-statistic rank
//! queries. The C# implementation uses a C5 `TreeSet` with O(log n) indexing;
//! we use a `BTreeSet` and count with `range(..).count()`. Same semantics.

use std::collections::BTreeSet;

use crate::error::{Error, Result};
use crate::rowset::{Interval, RowSequence, RowSequenceBuilder};

/// Maps row keys (key space) to dense positions (index space).
#[derive(Debug, Default)]
pub struct SpaceMapper {
    set: BTreeSet<u64>,
}

impl SpaceMapper {
    pub fn new() -> SpaceMapper {
        SpaceMapper { set: BTreeSet::new() }
    }

    /// Number of keys currently mapped.
    pub fn cardinality(&self) -> u64 {
        self.set.len() as u64
    }

    /// The zero-based rank of `value`: the number of keys strictly less than it.
    /// Port of `ZeroBasedRank` (C5 `LastIndexOf`, present or not).
    pub fn zero_based_rank(&self, value: u64) -> u64 {
        self.set.range(..value).count() as u64
    }

    /// Add `keys` (key space) and return their positions (index space) after
    /// insertion. Keys must not already be present. Port of `AddKeys`.
    pub fn add_keys(&mut self, keys: &RowSequence) -> Result<RowSequence> {
        let mut builder = RowSequenceBuilder::new();
        for interval in keys.intervals() {
            let index_range = self.add_range(interval)?;
            builder.add_interval(index_range);
        }
        Ok(builder.build())
    }

    /// Insert a contiguous key interval (must be absent) and return its
    /// index-space range. Port of `AddRange`.
    fn add_range(&mut self, interval: Interval) -> Result<Interval> {
        let initial = self.set.len();
        let size = interval.count();
        for key in interval.begin..interval.end {
            self.set.insert(key);
        }
        let change = (self.set.len() - initial) as u64;
        if change != size {
            return Err(Error::SpaceMapper(format!(
                "range [{}, {}) has size {size} but set cardinality changed by {change}; \
                 duplicates were inserted",
                interval.begin, interval.end
            )));
        }
        // The inserted keys are all >= begin, so the count of keys strictly
        // less than begin equals begin's position in the set.
        let index = self.zero_based_rank(interval.begin);
        Ok(Interval::of_start_and_size(index, size))
    }

    /// Remove keys in `interval` (some/all may be absent). Returns the rank of
    /// `interval.begin` before removal. Port of `EraseRange`.
    pub fn erase_range(&mut self, interval: Interval) -> u64 {
        let result = self.zero_based_rank(interval.begin);
        let to_remove: Vec<u64> =
            self.set.range(interval.begin..interval.end).copied().collect();
        for key in to_remove {
            self.set.remove(&key);
        }
        result
    }

    /// Move the existing keys in `range` so they start at `dest_key`, offsetting
    /// each by `dest_key - range.begin`. The caller guarantees the target region
    /// is empty. Port of `ApplyShift`.
    pub fn apply_shift(&mut self, range: Interval, dest_key: u64) -> Result<()> {
        // Build the shifted ranges from the keys we actually hold in `range`.
        let subset: Vec<u64> = self.set.range(range.begin..range.end).copied().collect();
        let mut new_ranges: Vec<Interval> = Vec::new();
        for item in subset {
            let new_key = dest_key + (item - range.begin);
            match new_ranges.last_mut() {
                Some(last) if last.end == new_key => last.end += 1,
                _ => new_ranges.push(Interval::of_singleton(new_key)),
            }
        }

        // Shifts never change the set size.
        let original = self.set.len();
        let to_remove: Vec<u64> = self.set.range(range.begin..range.end).copied().collect();
        for key in to_remove {
            self.set.remove(&key);
        }
        for new_range in new_ranges {
            self.add_range(new_range)?;
        }
        if self.set.len() != original {
            return Err(Error::SpaceMapper(format!(
                "shift changed set size from {original} to {}",
                self.set.len()
            )));
        }
        Ok(())
    }

    /// Look up `keys` (key space) and return their positions (index space).
    /// Each key interval must be fully present. Port of `ConvertKeysToIndices`.
    pub fn convert_keys_to_indices(&self, keys: &RowSequence) -> Result<RowSequence> {
        if keys.is_empty() {
            return Ok(RowSequence::create_empty());
        }
        let mut builder = RowSequenceBuilder::new();
        for interval in keys.intervals() {
            let size = interval.count();
            let present = self.set.range(interval.begin..interval.end).count() as u64;
            if present != size {
                return Err(Error::SpaceMapper(format!(
                    "of the {size} keys in [{}, {}), the set only contains {present}",
                    interval.begin, interval.end
                )));
            }
            let next_rank = self.zero_based_rank(interval.begin);
            builder.add_interval(Interval::of_start_and_size(next_rank, size));
        }
        Ok(builder.build())
    }
}

/// Decompose the "transposed" shift representation (first/last/dest RowSequences)
/// into `(closed-range-as-half-open, dest_key)` pairs, ordered so applying them
/// sequentially never collides: negative (downward) shifts in forward order,
/// then positive (upward) shifts in reverse order. Port of
/// `ShiftProcessor.AnalyzeShiftData`.
pub fn analyze_shift_data(
    first_index: &RowSequence,
    last_index: &RowSequence,
    dest_index: &RowSequence,
) -> Result<Vec<(Interval, u64)>> {
    let firsts = first_index.elements();
    let lasts = last_index.elements();
    let dests = dest_index.elements();
    if firsts.len() != lasts.len() || firsts.len() != dests.len() {
        return Err(Error::SpaceMapper(format!(
            "shift sequences not of same size: first={}, last={}, dest={}",
            firsts.len(),
            lasts.len(),
            dests.len()
        )));
    }

    let mut out: Vec<(Interval, u64)> = Vec::new();
    let mut positive_shifts: Vec<(Interval, u64)> = Vec::new();
    for i in 0..firsts.len() {
        // `last` is a closed bound; convert to half-open.
        let interval = Interval::of(firsts[i], lasts[i] + 1);
        if dests[i] >= firsts[i] {
            positive_shifts.push((interval, dests[i]));
        } else {
            out.push((interval, dests[i]));
        }
    }
    for shift in positive_shifts.into_iter().rev() {
        out.push(shift);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a RowSequence of singleton keys.
    fn keys(values: &[u64]) -> RowSequence {
        let mut b = RowSequenceBuilder::new();
        for &v in values {
            b.add(v);
        }
        b.build()
    }

    #[test]
    fn add_keys_doc_example() {
        // Port of the AddKeys doc example in SpaceMapper.cs.
        let mut sm = SpaceMapper::new();
        // Precondition: mapper holds [100, 300].
        sm.add_keys(&keys(&[100, 300])).unwrap();

        // AddKeys([1, 2, 200, 201, 400, 401]) -> positions [0,1,3,4,6,7].
        let positions = sm.add_keys(&keys(&[1, 2, 200, 201, 400, 401])).unwrap();
        assert_eq!(positions.elements(), vec![0, 1, 3, 4, 6, 7]);
        assert_eq!(sm.cardinality(), 8);
    }

    #[test]
    fn convert_keys_to_indices_maps_positions() {
        let mut sm = SpaceMapper::new();
        sm.add_keys(&keys(&[1, 2, 100, 200, 201, 300, 400, 401])).unwrap();
        let idx = sm.convert_keys_to_indices(&keys(&[200, 201, 300])).unwrap();
        assert_eq!(idx.elements(), vec![3, 4, 5]);
    }

    #[test]
    fn convert_missing_keys_errors() {
        let mut sm = SpaceMapper::new();
        sm.add_keys(&keys(&[1, 2, 3])).unwrap();
        assert!(matches!(
            sm.convert_keys_to_indices(&keys(&[2, 5])),
            Err(Error::SpaceMapper(_))
        ));
    }

    #[test]
    fn add_duplicate_errors() {
        let mut sm = SpaceMapper::new();
        sm.add_keys(&keys(&[10, 11])).unwrap();
        assert!(matches!(sm.add_keys(&keys(&[11, 12])), Err(Error::SpaceMapper(_))));
    }

    #[test]
    fn erase_range_returns_rank_and_removes() {
        let mut sm = SpaceMapper::new();
        sm.add_keys(&keys(&[1, 2, 100, 200])).unwrap();
        // Rank of 100 is 2 (two keys below it). Erasing removes it.
        let rank = sm.erase_range(Interval::of_singleton(100));
        assert_eq!(rank, 2);
        assert_eq!(sm.cardinality(), 3);
        assert!(matches!(
            sm.convert_keys_to_indices(&keys(&[100])),
            Err(Error::SpaceMapper(_))
        ));
    }

    #[test]
    fn apply_positive_shift_moves_keys_up() {
        let mut sm = SpaceMapper::new();
        sm.add_keys(&keys(&[10, 11, 12, 20, 21])).unwrap();
        // Move closed range [20, 21] to start at 30.
        sm.apply_shift(Interval::of(20, 22), 30).unwrap();
        assert_eq!(sm.cardinality(), 5);
        assert_eq!(sm.zero_based_rank(30), 3);
        // 20, 21 are gone; 30, 31 present.
        assert!(matches!(
            sm.convert_keys_to_indices(&keys(&[20])),
            Err(Error::SpaceMapper(_))
        ));
        assert_eq!(sm.convert_keys_to_indices(&keys(&[30, 31])).unwrap().elements(), vec![3, 4]);
    }

    #[test]
    fn apply_negative_shift_moves_keys_down() {
        let mut sm = SpaceMapper::new();
        sm.add_keys(&keys(&[10, 11, 20, 21])).unwrap();
        // Move closed range [20, 21] down to start at 5.
        sm.apply_shift(Interval::of(20, 22), 5).unwrap();
        assert_eq!(sm.cardinality(), 4);
        assert_eq!(sm.zero_based_rank(5), 0);
        assert_eq!(sm.convert_keys_to_indices(&keys(&[5, 6, 10, 11])).unwrap().elements(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn analyze_shift_data_orders_negative_then_positive_reversed() {
        // first/last/dest are each sorted ascending with positional pairing (as
        // the server sends them). Pairs: 10->5 (down), 30->40 (up), 70->90 (up).
        let first = keys(&[10, 30, 70]);
        let last = keys(&[10, 30, 70]);
        let dest = keys(&[5, 40, 90]);
        let shifts = analyze_shift_data(&first, &last, &dest).unwrap();
        // Negative shift (10->5) yielded first, then the two positive shifts in
        // reverse order: (70->90) then (30->40).
        assert_eq!(
            shifts,
            vec![
                (Interval::of(10, 11), 5),
                (Interval::of(70, 71), 90),
                (Interval::of(30, 31), 40),
            ]
        );
    }

    #[test]
    fn analyze_shift_data_size_mismatch_errors() {
        let first = keys(&[10, 20]);
        let last = keys(&[10]);
        let dest = keys(&[5, 6]);
        assert!(matches!(
            analyze_shift_data(&first, &last, &dest),
            Err(Error::SpaceMapper(_))
        ));
    }
}
