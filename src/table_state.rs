//! Port of `Dh_NetClient/ticking/TableState.cs`.
//!
//! `TableState` owns the live key space (via [`SpaceMapper`]) and the current
//! row count, and applies Barrage updates to it. The C# class additionally
//! moves Arrow cell data (`AddData`/`ModifyData` over `ColumnSource`s); that
//! column-value machinery is deferred — what we track here (key set + row
//! count, and the key-space↔index-space conversions) is what the subscription
//! loop needs to report coherent, growing row counts.

use crate::error::Result;
use crate::rowset::RowSequence;
use crate::space_mapper::{analyze_shift_data, SpaceMapper};

/// The evolving state of a subscribed table's row key space.
#[derive(Debug, Default)]
pub struct TableState {
    space_mapper: SpaceMapper,
}

impl TableState {
    pub fn new() -> TableState {
        TableState { space_mapper: SpaceMapper::new() }
    }

    /// Current number of rows (cardinality of the key space).
    pub fn num_rows(&self) -> u64 {
        self.space_mapper.cardinality()
    }

    /// Add `keys` (key space); returns their positions in index space. Port of
    /// `TableState.AddKeys` (the `AddData` cell-copy step is deferred).
    pub fn add_keys(&mut self, keys: &RowSequence) -> Result<RowSequence> {
        self.space_mapper.add_keys(keys)
    }

    /// Convert key-space keys to index-space positions. Port of
    /// `ConvertKeysToIndices`.
    pub fn convert_keys_to_indices(&self, keys: &RowSequence) -> Result<RowSequence> {
        self.space_mapper.convert_keys_to_indices(keys)
    }

    /// Erase `keys` (key space); returns the erased positions in index space.
    /// Port of `TableState.Erase` (without the cell-copy step): convert first
    /// (while the keys still exist), then remove from the mapper.
    pub fn erase(&mut self, keys: &RowSequence) -> Result<RowSequence> {
        let result = self.space_mapper.convert_keys_to_indices(keys)?;
        for interval in keys.intervals() {
            self.space_mapper.erase_range(interval);
        }
        Ok(result)
    }

    /// Apply the transposed shift data. Port of `TableState.ApplyShifts`.
    pub fn apply_shifts(
        &mut self,
        first_index: &RowSequence,
        last_index: &RowSequence,
        dest_index: &RowSequence,
    ) -> Result<()> {
        for (range, dest_key) in analyze_shift_data(first_index, last_index, dest_index)? {
            self.space_mapper.apply_shift(range, dest_key)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rowset::RowSequenceBuilder;

    fn keys(values: &[u64]) -> RowSequence {
        let mut b = RowSequenceBuilder::new();
        for &v in values {
            b.add(v);
        }
        b.build()
    }

    #[test]
    fn add_then_erase_tracks_row_count_and_positions() {
        let mut state = TableState::new();

        let added = state.add_keys(&keys(&[5, 6, 7, 100])).unwrap();
        assert_eq!(added.elements(), vec![0, 1, 2, 3]);
        assert_eq!(state.num_rows(), 4);

        // Erase key 6 -> it's at index 1.
        let erased = state.erase(&keys(&[6])).unwrap();
        assert_eq!(erased.elements(), vec![1]);
        assert_eq!(state.num_rows(), 3);

        // Remaining keys [5,7,100] map to dense positions [0,1,2].
        assert_eq!(
            state.convert_keys_to_indices(&keys(&[5, 7, 100])).unwrap().elements(),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn apply_shifts_preserves_count_and_relocates_keys() {
        let mut state = TableState::new();
        state.add_keys(&keys(&[10, 11, 12, 20, 21])).unwrap();

        // Shift the closed range [20,21] up to start at 30.
        state
            .apply_shifts(&keys(&[20]), &keys(&[21]), &keys(&[30]))
            .unwrap();

        assert_eq!(state.num_rows(), 5);
        assert_eq!(
            state.convert_keys_to_indices(&keys(&[10, 11, 12, 30, 31])).unwrap().elements(),
            vec![0, 1, 2, 3, 4]
        );
    }
}
