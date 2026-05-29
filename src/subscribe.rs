//! Port of `Dh_NetClient/subscription/SubscriptionThread.cs` plus the metadata
//! decode of the `BarrageProcessor` state machine (`AwaitingMetadata`).
//!
//! Opens a `DoExchange`, primes it with the subscription request as
//! `app_metadata`, then reads the response stream. Each `FlightData` carrying
//! `app_metadata` begins a Barrage update: we decode `BarrageUpdateMetadata`,
//! decode the row-key blobs with [`crate::rowset`], and apply them to a
//! [`TableState`] in the load-bearing order **removes → shifts → adds →
//! modifies**.
//!
//! Cell-data movement is deferred (see [`crate::table_state`]); the add/modify
//! RecordBatch messages on the stream are skipped. What we surface is coherent
//! key-space/row-count updates.

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
use crate::table_state::TableState;

/// The DoExchange command discriminator: ASCII "dphn".
const COMMAND: &[u8] = b"dphn";

/// One decoded Barrage update, applied to the local [`TableState`].
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

/// A live subscription: the DoExchange response stream plus the evolving table
/// state. Drop it to end the subscription.
pub struct Subscription {
    response: tonic::Streaming<FlightData>,
    table_state: TableState,
}

/// Subscribe to a named table: resolve its ticket, then open the subscription.
pub async fn subscribe(server: &Server, table_name: &str) -> Result<Subscription> {
    let ticket = flight::fetch_table(server, table_name).await?;
    subscribe_to_ticket(server, ticket).await
}

/// Subscribe to an already-resolved ticket. Port of `SubscriptionThread.Start`
/// + `UpdateProcessor.RunForeverHelper` (priming).
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

    // Keep the request stream open after priming (pending never completes) so
    // the server keeps streaming updates.
    let outbound = stream::once(async move { priming }).chain(stream::pending::<FlightData>());

    let mut flight = FlightServiceClient::new(server.channel());
    let response = flight.do_exchange(server.authorize(outbound)?).await?.into_inner();

    Ok(Subscription { response, table_state: TableState::new() })
}

impl Subscription {
    /// Current local row count.
    pub fn num_rows(&self) -> u64 {
        self.table_state.num_rows()
    }

    /// Read the stream until the next update arrives (skipping the schema and
    /// data-only RecordBatch messages, which have no `app_metadata`). Returns
    /// `None` when the stream ends.
    pub async fn next_update(&mut self) -> Result<Option<TickingUpdate>> {
        while let Some(item) = self.response.next().await {
            let flight_data = item?;
            if flight_data.app_metadata.is_empty() {
                continue;
            }
            let update = self.apply_metadata(&flight_data.app_metadata)?;
            return Ok(Some(update));
        }
        Ok(None)
    }

    /// Decode `BarrageUpdateMetadata` and apply it to the table state in the
    /// fixed order. Port of `AwaitingMetadata.ProcessNextChunk` (metadata path).
    fn apply_metadata(&mut self, metadata: &[u8]) -> Result<TickingUpdate> {
        let wrapper = fb::BarrageMessageWrapperRef::read_as_root(metadata)
            .map_err(|e| Error::Barrage(format!("wrapper parse: {e}")))?;

        let magic = wrapper.magic().map_err(|e| Error::Barrage(e.to_string()))?;
        if magic != DEEPHAVEN_MAGIC {
            return Err(Error::Barrage(format!(
                "expected magic {DEEPHAVEN_MAGIC:#x}, got {magic:#x}"
            )));
        }
        let msg_type = wrapper.msg_type().map_err(|e| Error::Barrage(e.to_string()))?;
        if msg_type != fb::BarrageMessageType::BarrageUpdateMetadata {
            return Err(Error::Barrage(format!(
                "expected BarrageUpdateMetadata, got {msg_type:?}"
            )));
        }

        let payload = wrapper
            .msg_payload()
            .map_err(|e| Error::Barrage(e.to_string()))?
            .ok_or_else(|| Error::Barrage("update metadata payload missing".into()))?;
        let payload = i8_to_u8(payload);
        let bmd = fb::BarrageUpdateMetadataRef::read_as_root(&payload)
            .map_err(|e| Error::Barrage(format!("metadata parse: {e}")))?;

        // Decode the row-key blobs.
        let removed_rows = decode_seq(bmd.removed_rows().map_err(barrage_err)?)?;
        let added_rows = decode_seq(bmd.added_rows().map_err(barrage_err)?)?;
        let (shift_start, shift_end, shift_dest) =
            decode_shifts(bmd.shift_data().map_err(barrage_err)?)?;

        let mut modified_rows = Vec::new();
        if let Some(nodes) = bmd.mod_column_nodes().map_err(barrage_err)? {
            for node in nodes {
                let node = node.map_err(barrage_err)?;
                modified_rows.push(decode_seq(node.modified_rows().map_err(barrage_err)?)?);
            }
        }

        let is_snapshot = bmd.is_snapshot().map_err(barrage_err)?;
        let table_size = bmd.table_size().map_err(barrage_err)?;

        // Apply in EXACTLY this order: removes -> shifts -> adds -> modifies.
        let removed = if removed_rows.is_empty() {
            RowSequence::create_empty()
        } else {
            self.table_state.erase(&removed_rows)?
        };
        self.table_state
            .apply_shifts(&shift_start, &shift_end, &shift_dest)?;
        let added = self.table_state.add_keys(&added_rows)?;

        // Modifies change cell values, not the key space; converting to index
        // space validates the keys exist. (No cell data tracked yet.)
        let mut modified_per_column = Vec::with_capacity(modified_rows.len());
        for m in &modified_rows {
            modified_per_column.push(self.table_state.convert_keys_to_indices(m)?);
        }

        Ok(TickingUpdate {
            is_snapshot,
            removed,
            added,
            modified_per_column,
            num_rows: self.table_state.num_rows(),
            table_size,
        })
    }
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
