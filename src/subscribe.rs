//! Port of `Dh_NetClient/subscription/SubscriptionThread.cs` and the
//! `BarrageProcessor` chunk-processing state machine.
//!
//! Opens a `DoExchange`, primes it with the subscription request as
//! `app_metadata`, then reads the response stream. The stream begins with an
//! Arrow schema (list-wrapped, since `columns_as_list` is set), followed by
//! update messages. A message carrying `app_metadata` begins a Barrage update
//! (`BarrageUpdateMetadata`) and ALSO carries the first add-data record batch;
//! subsequent messages carry more add/modify record batches.
//!
//! We decode the row-key blobs with [`crate::rowset`] and apply the update to a
//! [`TableState`] in the load-bearing order **removes → shifts → adds →
//! modifies**, splicing the Arrow cell data in as the record batches arrive.

use arrow::array::ArrayRef;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use arrow_flight::decode::{DecodedPayload, FlightDataDecoder};
use arrow_flight::error::FlightError;
use arrow_flight::flight_descriptor::DescriptorType;
use arrow_flight::flight_service_client::FlightServiceClient;
use arrow_flight::{FlightData, FlightDescriptor};
use futures::{stream, StreamExt};
use planus::ReadAsRoot;

use crate::auth::Server;
use crate::barrage::{self, DEEPHAVEN_MAGIC};
use crate::barrage_generated::io::deephaven::barrage::flatbuf as fb;
use crate::error::{Error, Result};
use crate::flight;
use crate::rowset::{read_external_compressed_delta, DataInput, RowSequence};
use crate::table_state::{unwrap_columns, unwrap_schema, TableState};

/// The DoExchange command discriminator: ASCII "dphn".
const COMMAND: &[u8] = b"dphn";

/// One decoded Barrage update, after application to the local [`TableState`].
pub struct TickingUpdate {
    /// True for the initial snapshot update.
    pub is_snapshot: bool,
    /// Positions (index space) that were removed, before removal.
    pub removed: RowSequence,
    /// Positions (index space) that were added, after insertion.
    pub added: RowSequence,
    /// Per-column modified positions (index space).
    pub modified_per_column: Vec<RowSequence>,
    /// Local row count after applying this update.
    pub num_rows: u64,
    /// Server-reported table size (cross-check for `num_rows`).
    pub table_size: i64,
}

/// In-progress state for the update currently being assembled from chunks.
struct Pending {
    is_snapshot: bool,
    table_size: i64,
    removed_index: RowSequence,
    added_index: RowSequence,
    /// Added positions not yet filled with cell data.
    added_remaining: RowSequence,
    /// Decoded per-column modified rows (key space), pending conversion.
    modifies_key_space: Vec<RowSequence>,
    /// Per-column modified positions (index space), once the adds are done.
    modified_index: Vec<RowSequence>,
    /// Per-column modified positions not yet filled with cell data.
    modified_remaining: Vec<RowSequence>,
    /// False while consuming add batches, true once consuming modify batches.
    in_modifies: bool,
    modifies_initialized: bool,
}

/// A live subscription: the decoded DoExchange stream plus the evolving table
/// state. Drop it to end the subscription.
pub struct Subscription {
    decoder: FlightDataDecoder,
    table_state: Option<TableState>,
    pending: Option<Pending>,
}

/// Subscribe to a named table: resolve its ticket, then open the subscription.
pub async fn subscribe(server: &Server, table_name: &str) -> Result<Subscription> {
    let ticket = flight::fetch_table(server, table_name).await?;
    subscribe_to_ticket(server, ticket).await
}

/// Subscribe to an already-resolved ticket. Port of `SubscriptionThread.Start`
/// + the priming in `UpdateProcessor.RunForeverHelper`.
pub async fn subscribe_to_ticket(server: &Server, ticket: Vec<u8>) -> Result<Subscription> {
    let sub_req = barrage::create_subscription_request(&ticket);

    // The one priming message: descriptor `dphn`, subscription request in
    // app_metadata. The data payload is irrelevant.
    let priming = FlightData {
        flight_descriptor: Some(FlightDescriptor {
            r#type: DescriptorType::Cmd as i32,
            cmd: COMMAND.to_vec().into(),
            path: Vec::new(),
        }),
        app_metadata: sub_req.into(),
        data_header: Default::default(),
        data_body: Default::default(),
    };

    // Keep the request stream open after priming so the server keeps streaming.
    let outbound = stream::once(async move { priming }).chain(stream::pending::<FlightData>());

    let mut flight = FlightServiceClient::new(server.channel());
    let response = flight.do_exchange(server.authorize(outbound)?).await?.into_inner();

    // Adapt the tonic stream into the FlightError-typed stream the decoder wants.
    let decoder = FlightDataDecoder::new(response.map(|r| r.map_err(FlightError::from)));

    Ok(Subscription { decoder, table_state: None, pending: None })
}

impl Subscription {
    /// Current local row count.
    pub fn num_rows(&self) -> u64 {
        self.table_state.as_ref().map(|t| t.num_rows()).unwrap_or(0)
    }

    /// A snapshot of the current table values as an Arrow `RecordBatch`.
    pub fn snapshot(&self) -> Result<RecordBatch> {
        self.table_state
            .as_ref()
            .ok_or_else(|| Error::Barrage("no schema received yet".into()))?
            .snapshot()
    }

    /// Read the stream until a complete update has been assembled, applying it
    /// to the table state. Returns `None` when the stream ends. Drives the
    /// `BarrageProcessor` state machine across record-batch chunks.
    pub async fn next_update(&mut self) -> Result<Option<TickingUpdate>> {
        loop {
            let item = match self.decoder.next().await {
                None => return Ok(None),
                Some(r) => r.map_err(flight_err)?,
            };

            match item.payload {
                DecodedPayload::Schema(wire_schema) => {
                    self.init_table_state(&wire_schema);
                    continue;
                }
                DecodedPayload::None => {
                    // Could still carry update metadata with no record batch.
                    if item.inner.app_metadata.is_empty() {
                        continue;
                    }
                    self.begin_update(&item.inner.app_metadata)?;
                    if let Some(update) = self.feed_chunk(None)? {
                        return Ok(Some(update));
                    }
                }
                DecodedPayload::RecordBatch(batch) => {
                    if !item.inner.app_metadata.is_empty() {
                        self.begin_update(&item.inner.app_metadata)?;
                    }
                    if let Some(update) = self.feed_chunk(Some(batch))? {
                        return Ok(Some(update));
                    }
                }
            }
        }
    }

    fn init_table_state(&mut self, wire_schema: &SchemaRef) {
        let schema = unwrap_schema(wire_schema);
        self.table_state = Some(TableState::with_schema(schema));
    }

    /// Decode `BarrageUpdateMetadata`, apply removes + shifts + add_keys, and
    /// stage the pending add/modify work. Port of the metadata path in
    /// `AwaitingMetadata.ProcessNextChunk`.
    fn begin_update(&mut self, metadata: &[u8]) -> Result<()> {
        let table_state = self
            .table_state
            .as_mut()
            .ok_or_else(|| Error::Barrage("update metadata arrived before schema".into()))?;

        let wrapper = fb::BarrageMessageWrapperRef::read_as_root(metadata)
            .map_err(|e| Error::Barrage(format!("wrapper parse: {e}")))?;
        let magic = wrapper.magic().map_err(barrage_err)?;
        if magic != DEEPHAVEN_MAGIC {
            return Err(Error::Barrage(format!(
                "expected magic {DEEPHAVEN_MAGIC:#x}, got {magic:#x}"
            )));
        }
        if wrapper.msg_type().map_err(barrage_err)? != fb::BarrageMessageType::BarrageUpdateMetadata {
            return Err(Error::Barrage("expected BarrageUpdateMetadata".into()));
        }

        let payload = wrapper
            .msg_payload()
            .map_err(barrage_err)?
            .ok_or_else(|| Error::Barrage("update metadata payload missing".into()))?;
        let payload = i8_to_u8(payload);
        let bmd = fb::BarrageUpdateMetadataRef::read_as_root(&payload)
            .map_err(|e| Error::Barrage(format!("metadata parse: {e}")))?;

        let removed_rows = decode_seq(bmd.removed_rows().map_err(barrage_err)?)?;
        let added_rows = decode_seq(bmd.added_rows().map_err(barrage_err)?)?;
        let (shift_start, shift_end, shift_dest) =
            decode_shifts(bmd.shift_data().map_err(barrage_err)?)?;

        let mut modifies_key_space = Vec::new();
        if let Some(nodes) = bmd.mod_column_nodes().map_err(barrage_err)? {
            for node in nodes {
                let node = node.map_err(barrage_err)?;
                modifies_key_space.push(decode_seq(node.modified_rows().map_err(barrage_err)?)?);
            }
        }

        let is_snapshot = bmd.is_snapshot().map_err(barrage_err)?;
        let table_size = bmd.table_size().map_err(barrage_err)?;

        // 1. removes, 2. shifts, 3. add_keys (cell data spliced in feed_chunk).
        let removed_index = if removed_rows.is_empty() {
            RowSequence::create_empty()
        } else {
            table_state.erase(&removed_rows)?
        };
        table_state.apply_shifts(&shift_start, &shift_end, &shift_dest)?;
        let added_index = table_state.add_keys(&added_rows)?;

        self.pending = Some(Pending {
            is_snapshot,
            table_size,
            removed_index,
            added_remaining: added_index.clone(),
            added_index,
            modifies_key_space,
            modified_index: Vec::new(),
            modified_remaining: Vec::new(),
            in_modifies: false,
            modifies_initialized: false,
        });
        Ok(())
    }

    /// Feed a record batch (or `None`) into the current update, consuming it as
    /// add data then modify data. Returns the completed [`TickingUpdate`] when
    /// the update is fully assembled. Ports `AwaitingAdds`/`AwaitingModifies`/
    /// `BuildingResult`.
    fn feed_chunk(&mut self, batch: Option<RecordBatch>) -> Result<Option<TickingUpdate>> {
        let table_state = self
            .table_state
            .as_mut()
            .ok_or_else(|| Error::Barrage("record batch before schema".into()))?;
        let pending = self
            .pending
            .as_mut()
            .ok_or_else(|| Error::Barrage("record batch with no update in progress".into()))?;

        // Per-column chunk arrays and a shrinking available-count/offset cursor.
        let (sources, mut avail) = match &batch {
            Some(rb) => unwrap_columns(rb)?,
            None => (Vec::new(), Vec::new()),
        };
        let mut offset = vec![0usize; avail.len()];

        // --- Adds phase ---
        if !pending.in_modifies {
            if pending.added_remaining.is_empty() {
                pending.in_modifies = true;
            } else {
                if avail.is_empty() {
                    return Ok(None); // need an add-data batch
                }
                let chunk = avail[0];
                if !avail.iter().all(|&a| a == chunk) {
                    return Err(Error::Barrage(format!(
                        "add chunk columns have inconsistent sizes: {avail:?}"
                    )));
                }
                if chunk == 0 {
                    return Ok(None); // empty batch, need more
                }
                if pending.added_remaining.count() < chunk as u64 {
                    return Err(Error::Barrage(format!(
                        "excess add data: have {} remaining, batch carries {chunk}",
                        pending.added_remaining.count()
                    )));
                }
                let rows = pending.added_remaining.take(chunk as u64);
                let chunk_sources: Vec<ArrayRef> =
                    sources.iter().map(|s| s.slice(0, chunk)).collect();
                table_state.add_data(&chunk_sources, &rows)?;
                pending.added_remaining = pending.added_remaining.drop(chunk as u64);
                for a in avail.iter_mut() {
                    *a = 0;
                }
                if !pending.added_remaining.is_empty() {
                    return Ok(None); // more add data to come
                }
                pending.in_modifies = true;
                // This batch was an add batch (now consumed); modifies, if any,
                // arrive in later batches.
            }
        }

        // --- Modifies phase ---
        if !pending.modifies_initialized {
            pending.modifies_initialized = true;
            let mut modified_index = Vec::with_capacity(pending.modifies_key_space.len());
            for keys in &pending.modifies_key_space {
                modified_index.push(table_state.convert_keys_to_indices(keys)?);
            }
            pending.modified_remaining = modified_index.clone();
            pending.modified_index = modified_index;
        }

        if pending.modified_remaining.iter().all(|r| r.is_empty()) {
            return Ok(Some(build_update(pending, table_state.num_rows())));
        }

        // Consume whatever modify data this batch carries (per column).
        if !avail.is_empty() && avail.iter().any(|&a| a > 0) {
            for i in 0..pending.modified_remaining.len() {
                let a = *avail.get(i).unwrap_or(&0);
                if a == 0 {
                    continue;
                }
                let remaining = pending.modified_remaining[i].count();
                if a as u64 > remaining {
                    return Err(Error::Barrage(format!(
                        "col {i}: modify batch carries {a} but only {remaining} remaining"
                    )));
                }
                let rows = pending.modified_remaining[i].take(a as u64);
                let source = sources[i].slice(offset[i], a);
                table_state.modify_data(i, &source, &rows)?;
                pending.modified_remaining[i] = pending.modified_remaining[i].drop(a as u64);
                offset[i] += a;
                avail[i] = 0;
            }
        }

        if pending.modified_remaining.iter().all(|r| r.is_empty()) {
            return Ok(Some(build_update(pending, table_state.num_rows())));
        }

        Ok(None) // need more modify data
    }
}

/// Build the result update and consume the pending state.
fn build_update(pending: &mut Pending, num_rows: u64) -> TickingUpdate {
    TickingUpdate {
        is_snapshot: pending.is_snapshot,
        removed: std::mem::replace(&mut pending.removed_index, RowSequence::create_empty()),
        added: std::mem::replace(&mut pending.added_index, RowSequence::create_empty()),
        modified_per_column: std::mem::take(&mut pending.modified_index),
        num_rows,
        table_size: pending.table_size,
    }
}

fn flight_err(e: FlightError) -> Error {
    Error::Arrow(e.to_string())
}

fn barrage_err(e: planus::Error) -> Error {
    Error::Barrage(e.to_string())
}

/// flatbuffers `[byte]` is signed; reinterpret as unsigned for the codec.
fn i8_to_u8(bytes: &[i8]) -> Vec<u8> {
    bytes.iter().map(|&b| b as u8).collect()
}

/// Decode a single RowSequence from an optional blob (absent/empty => empty).
fn decode_seq(bytes: Option<&[i8]>) -> Result<RowSequence> {
    match bytes {
        None => Ok(RowSequence::create_empty()),
        Some(b) => {
            let buf = i8_to_u8(b);
            let mut input = DataInput::new(&buf);
            read_external_compressed_delta(&mut input)
        }
    }
}

/// Decode the THREE RowSequences packed into `shift_data` from one cursor
/// (start, end, dest) — read sequentially, never re-sliced.
fn decode_shifts(bytes: Option<&[i8]>) -> Result<(RowSequence, RowSequence, RowSequence)> {
    match bytes {
        None => Ok((
            RowSequence::create_empty(),
            RowSequence::create_empty(),
            RowSequence::create_empty(),
        )),
        Some(b) => {
            let buf = i8_to_u8(b);
            let mut input = DataInput::new(&buf);
            let start = read_external_compressed_delta(&mut input)?;
            let end = read_external_compressed_delta(&mut input)?;
            let dest = read_external_compressed_delta(&mut input)?;
            Ok((start, end, dest))
        }
    }
}
