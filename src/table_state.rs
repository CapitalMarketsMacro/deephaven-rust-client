//! Port of `Dh_NetClient/ticking/TableState.cs`.
//!
//! `TableState` owns the live key space (via [`SpaceMapper`]) **and** the dense
//! index-space column data. Barrage updates are applied in the load-bearing
//! order removes → shifts → adds → modifies:
//!
//! * removes  — erase column cells + keys at the removed positions
//! * shifts   — SpaceMapper only (dense data doesn't reorder, so cells stay put)
//! * adds     — `add_keys` grows the key space; `add_data` splices in new cells
//! * modifies — `modify_data` overwrites cells at the modified positions
//!
//! The C# implementation copies via typed `ColumnSource`/`Chunk` machinery with
//! Deephaven null-sentinel handling. We instead move Arrow arrays directly with
//! the type-generic `concat`/`slice` kernels. (Deephaven null *sentinels* are
//! carried through as-is rather than normalized to Arrow validity bits — a
//! minor presentation nuance, not a data-movement one.)

use arrow::array::{new_empty_array, Array, ArrayRef, ListArray};
use arrow::compute::concat;
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

use crate::error::{Error, Result};
use crate::rowset::RowSequence;
use crate::space_mapper::{analyze_shift_data, SpaceMapper};

/// The evolving state of a subscribed table: key space + dense column data.
#[derive(Debug)]
pub struct TableState {
    space_mapper: SpaceMapper,
    /// The table's real (unwrapped) schema. `None` for key-space-only use.
    schema: Option<SchemaRef>,
    /// Dense column data, one array per field; each has length == row count.
    /// Empty when there is no schema.
    columns: Vec<ArrayRef>,
}

impl Default for TableState {
    fn default() -> Self {
        TableState::new()
    }
}

impl TableState {
    /// Key-space-only state (no column data). Used for pure-logic tests.
    pub fn new() -> TableState {
        TableState { space_mapper: SpaceMapper::new(), schema: None, columns: Vec::new() }
    }

    /// State backed by `schema`, with empty column data. The schema must be the
    /// table's real (unwrapped) schema — see [`unwrap_schema`].
    pub fn with_schema(schema: SchemaRef) -> TableState {
        let columns = schema
            .fields()
            .iter()
            .map(|f| new_empty_array(f.data_type()))
            .collect();
        TableState { space_mapper: SpaceMapper::new(), schema: Some(schema), columns }
    }

    /// Current number of rows (cardinality of the key space).
    pub fn num_rows(&self) -> u64 {
        self.space_mapper.cardinality()
    }

    /// The table's real schema, if any.
    pub fn schema(&self) -> Option<&SchemaRef> {
        self.schema.as_ref()
    }

    /// Add `keys` (key space); returns their positions in index space. Grows the
    /// key space only — call [`add_data`](Self::add_data) to splice in the cells.
    /// Port of `TableState.AddKeys`.
    pub fn add_keys(&mut self, keys: &RowSequence) -> Result<RowSequence> {
        self.space_mapper.add_keys(keys)
    }

    /// Convert key-space keys to index-space positions. Port of
    /// `ConvertKeysToIndices`.
    pub fn convert_keys_to_indices(&self, keys: &RowSequence) -> Result<RowSequence> {
        self.space_mapper.convert_keys_to_indices(keys)
    }

    /// Erase `keys` (key space); removes the corresponding cells (if column data
    /// is present) and keys, and returns the erased index-space positions. Port
    /// of `TableState.Erase`. Convert first (keys still present), then remove.
    pub fn erase(&mut self, keys: &RowSequence) -> Result<RowSequence> {
        let result = self.space_mapper.convert_keys_to_indices(keys)?;
        for i in 0..self.columns.len() {
            self.columns[i] = splice_erase(&self.columns[i], &result)?;
        }
        for interval in keys.intervals() {
            self.space_mapper.erase_range(interval);
        }
        Ok(result)
    }

    /// Apply the transposed shift data (SpaceMapper only). Port of
    /// `TableState.ApplyShifts`.
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

    /// Splice the new-row cells from `sources` (one chunk per column) into the
    /// columns at the index-space positions `rows_to_add`. Port of
    /// `TableState.AddData`. The keys must already have been added via
    /// [`add_keys`](Self::add_keys).
    pub fn add_data(&mut self, sources: &[ArrayRef], rows_to_add: &RowSequence) -> Result<()> {
        self.require_schema()?;
        if sources.len() != self.columns.len() {
            return Err(Error::Arrow(format!(
                "add_data: {} source columns but table has {}",
                sources.len(),
                self.columns.len()
            )));
        }
        for i in 0..self.columns.len() {
            self.columns[i] = splice_add(&self.columns[i], &sources[i], rows_to_add)?;
        }
        Ok(())
    }

    /// Overwrite the cells of column `col` at the index-space positions
    /// `rows_to_modify` with values from `source`. Port of
    /// `TableState.ModifyData`.
    pub fn modify_data(
        &mut self,
        col: usize,
        source: &ArrayRef,
        rows_to_modify: &RowSequence,
    ) -> Result<()> {
        self.require_schema()?;
        let column = self
            .columns
            .get(col)
            .ok_or_else(|| Error::Arrow(format!("modify_data: column {col} out of range")))?;
        self.columns[col] = splice_modify(column, source, rows_to_modify)?;
        Ok(())
    }

    /// A snapshot of the current table state as an Arrow `RecordBatch`. Port of
    /// `TableState.Snapshot`.
    pub fn snapshot(&self) -> Result<RecordBatch> {
        let schema = self
            .schema
            .clone()
            .ok_or_else(|| Error::Arrow("snapshot: table state has no schema".into()))?;
        RecordBatch::try_new(schema, self.columns.clone()).map_err(|e| Error::Arrow(e.to_string()))
    }

    fn require_schema(&self) -> Result<()> {
        if self.schema.is_none() {
            return Err(Error::Arrow("operation requires a schema-backed TableState".into()));
        }
        Ok(())
    }
}

/// Derive a table's real schema from the list-wrapped wire schema sent over
/// DoExchange when `columns_as_list` is set: each `List<T>` field becomes a `T`
/// field.
pub fn unwrap_schema(wire: &Schema) -> SchemaRef {
    let fields: Vec<Field> = wire
        .fields()
        .iter()
        .map(|f| match f.data_type() {
            DataType::List(child) | DataType::LargeList(child) => {
                Field::new(f.name(), child.data_type().clone(), true)
            }
            other => Field::new(f.name(), other.clone(), f.is_nullable()),
        })
        .collect();
    Arc::new(Schema::new(fields))
}

/// Unwrap a record batch's `columns_as_list` columns into per-column value
/// chunks and their sizes. Port of `ArrowColumnSource.CreateFromListArray`.
pub fn unwrap_columns(batch: &RecordBatch) -> Result<(Vec<ArrayRef>, Vec<usize>)> {
    let mut sources = Vec::with_capacity(batch.num_columns());
    let mut sizes = Vec::with_capacity(batch.num_columns());
    for column in batch.columns() {
        if let Some(list) = column.as_any().downcast_ref::<ListArray>() {
            // columns_as_list wraps each column in a single-element list.
            let values = if list.is_empty() {
                new_empty_array(&list.value_type())
            } else {
                list.value(0)
            };
            sizes.push(values.len());
            sources.push(values);
        } else {
            // Not list-wrapped: use the column directly.
            sizes.push(column.len());
            sources.push(column.clone());
        }
    }
    Ok((sources, sizes))
}

// --- Arrow splice helpers (type-generic via concat/slice) -----------------

fn concat_parts(parts: &[ArrayRef], empty_type: &DataType) -> Result<ArrayRef> {
    if parts.is_empty() {
        return Ok(new_empty_array(empty_type));
    }
    let refs: Vec<&dyn Array> = parts.iter().map(|a| a.as_ref()).collect();
    concat(&refs).map_err(|e| Error::Arrow(e.to_string()))
}

/// Build a new column inserting `new`'s cells at the index-space `rows`,
/// interleaving with `orig`. Mirrors the `TableState.AddData` walk.
fn splice_add(orig: &ArrayRef, new: &ArrayRef, rows: &RowSequence) -> Result<ArrayRef> {
    let orig_len = orig.len();
    let mut parts: Vec<ArrayRef> = Vec::new();
    let mut orig_index = 0usize;
    let mut new_index = 0usize;
    let mut dest_index = 0usize;
    for iv in rows.intervals() {
        let begin = iv.begin as usize;
        let end = iv.end as usize;
        if dest_index < begin {
            let n = begin - dest_index;
            parts.push(orig.slice(orig_index, n));
            orig_index += n;
            dest_index += n;
        }
        let n = end - begin;
        parts.push(new.slice(new_index, n));
        new_index += n;
        dest_index += n;
    }
    if orig_index < orig_len {
        parts.push(orig.slice(orig_index, orig_len - orig_index));
    }
    concat_parts(&parts, orig.data_type())
}

/// Build a new column dropping the cells at the index-space `removed` positions.
/// Mirrors the `TableState.Erase` walk.
fn splice_erase(src: &ArrayRef, removed: &RowSequence) -> Result<ArrayRef> {
    let len = src.len();
    let mut parts: Vec<ArrayRef> = Vec::new();
    let mut src_index = 0usize;
    for iv in removed.intervals() {
        let begin = iv.begin as usize;
        let end = iv.end as usize;
        if src_index < begin {
            parts.push(src.slice(src_index, begin - src_index));
        }
        src_index = end;
    }
    if src_index < len {
        parts.push(src.slice(src_index, len - src_index));
    }
    concat_parts(&parts, src.data_type())
}

/// Build a new column replacing the cells at the index-space `rows` with `new`'s
/// cells. Mirrors the `TableState.ModifyData` walk (same length as `orig`).
fn splice_modify(orig: &ArrayRef, new: &ArrayRef, rows: &RowSequence) -> Result<ArrayRef> {
    let len = orig.len();
    let mut parts: Vec<ArrayRef> = Vec::new();
    let mut src_index = 0usize;
    let mut new_index = 0usize;
    let mut dest_index = 0usize;
    for iv in rows.intervals() {
        let begin = iv.begin as usize;
        let end = iv.end as usize;
        if dest_index < begin {
            let n = begin - dest_index;
            parts.push(orig.slice(src_index, n));
            src_index += n;
            dest_index += n;
        }
        let n = end - begin;
        parts.push(new.slice(new_index, n));
        new_index += n;
        src_index += n; // skip the overwritten orig cells
        dest_index += n;
    }
    if src_index < len {
        parts.push(orig.slice(src_index, len - src_index));
    }
    concat_parts(&parts, orig.data_type())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rowset::RowSequenceBuilder;
    use arrow::array::Int32Array;

    fn keys(values: &[u64]) -> RowSequence {
        let mut b = RowSequenceBuilder::new();
        for &v in values {
            b.add(v);
        }
        b.build()
    }

    // ---- Key-space-only behavior (no schema) -----------------------------

    #[test]
    fn add_then_erase_tracks_row_count_and_positions() {
        let mut state = TableState::new();
        let added = state.add_keys(&keys(&[5, 6, 7, 100])).unwrap();
        assert_eq!(added.elements(), vec![0, 1, 2, 3]);
        assert_eq!(state.num_rows(), 4);

        let erased = state.erase(&keys(&[6])).unwrap();
        assert_eq!(erased.elements(), vec![1]);
        assert_eq!(state.num_rows(), 3);
        assert_eq!(
            state.convert_keys_to_indices(&keys(&[5, 7, 100])).unwrap().elements(),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn apply_shifts_preserves_count_and_relocates_keys() {
        let mut state = TableState::new();
        state.add_keys(&keys(&[10, 11, 12, 20, 21])).unwrap();
        state.apply_shifts(&keys(&[20]), &keys(&[21]), &keys(&[30])).unwrap();
        assert_eq!(state.num_rows(), 5);
        assert_eq!(
            state.convert_keys_to_indices(&keys(&[10, 11, 12, 30, 31])).unwrap().elements(),
            vec![0, 1, 2, 3, 4]
        );
    }

    // ---- Column data movement (with schema) ------------------------------

    fn int_state() -> TableState {
        let schema = Arc::new(Schema::new(vec![Field::new("X", DataType::Int32, true)]));
        TableState::with_schema(schema)
    }

    fn col(state: &TableState) -> Vec<i32> {
        let batch = state.snapshot().unwrap();
        batch
            .column(0)
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect()
    }

    fn arr(values: &[i32]) -> ArrayRef {
        Arc::new(Int32Array::from(values.to_vec()))
    }

    #[test]
    fn add_data_inserts_at_positions() {
        // Existing [C,D,G,H] (10,20,60,70) at positions 0..4, then add
        // [A,B,E,F] (1,2,40,50) at index positions 0,1,4,5 -> A,B,C,D,E,F,G,H.
        let mut state = int_state();
        // seed existing rows
        let added = state.add_keys(&keys(&[10, 20, 60, 70])).unwrap();
        state.add_data(&[arr(&[10, 20, 60, 70])], &added).unwrap();
        assert_eq!(col(&state), vec![10, 20, 60, 70]);

        // add new keys; their index positions become 0,1,4,5
        let added2 = state.add_keys(&keys(&[1, 2, 40, 50])).unwrap();
        assert_eq!(added2.elements(), vec![0, 1, 4, 5]);
        state.add_data(&[arr(&[1, 2, 40, 50])], &added2).unwrap();
        assert_eq!(col(&state), vec![1, 2, 10, 20, 40, 50, 60, 70]);
        assert_eq!(state.num_rows(), 8);
    }

    #[test]
    fn erase_removes_cells() {
        let mut state = int_state();
        let added = state.add_keys(&keys(&[1, 2, 3, 4])).unwrap();
        state.add_data(&[arr(&[10, 20, 30, 40])], &added).unwrap();
        // erase keys 2 and 3 (index positions 1,2)
        state.erase(&keys(&[2, 3])).unwrap();
        assert_eq!(col(&state), vec![10, 40]);
        assert_eq!(state.num_rows(), 2);
    }

    #[test]
    fn modify_data_overwrites_cells() {
        let mut state = int_state();
        let added = state.add_keys(&keys(&[1, 2, 3, 4])).unwrap();
        state.add_data(&[arr(&[10, 20, 30, 40])], &added).unwrap();
        // modify keys 2 and 4 (index positions 1,3) to 99, 88
        let mod_idx = state.convert_keys_to_indices(&keys(&[2, 4])).unwrap();
        state.modify_data(0, &arr(&[99, 88]), &mod_idx).unwrap();
        assert_eq!(col(&state), vec![10, 99, 30, 88]);
        assert_eq!(state.num_rows(), 4);
    }
}
