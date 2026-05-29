//! Port of the snapshot path in `Dh_NetClient/TableHandleManager.cs`
//! (`FetchTable`, `MakeScopeReference`) and `TableHandle.cs`
//! (`GetFlightStream`, `ToArrowTableAsync`).
//!
//! Resolving a named global table is two steps:
//!   1. `TableService.FetchTable` with a "scope reference" ticket (`s/<name>`)
//!      as the source, exporting it to a fresh client ticket.
//!   2. Flight `DoGet` on that export ticket, decoding the Arrow stream.

use arrow::datatypes::{Schema, SchemaRef};
use arrow::record_batch::RecordBatch;
use arrow_flight::flight_service_client::FlightServiceClient;
use arrow_flight::utils::flight_data_to_batches;
use arrow_flight::Ticket as FlightTicket;
use futures::StreamExt;
use std::sync::Arc;

use crate::auth::Server;
use crate::error::{Error, Result};
use crate::proto::grpc::table_service_client::TableServiceClient;
use crate::proto::grpc::{table_reference, FetchTableRequest, TableReference, Ticket};

/// A point-in-time snapshot of a table: its Arrow schema plus the record
/// batches returned by `DoGet`.
pub struct Snapshot {
    pub schema: SchemaRef,
    pub batches: Vec<RecordBatch>,
}

impl Snapshot {
    /// Total number of rows across all batches.
    pub fn num_rows(&self) -> usize {
        self.batches.iter().map(|b| b.num_rows()).sum()
    }
}

/// Transform a table name into a Deephaven scope-reference ticket (`s/<name>`).
/// Port of `TableHandleManager.MakeScopeReference`.
fn scope_reference(table_name: &str) -> Vec<u8> {
    format!("s/{table_name}").into_bytes()
}

/// Resolve a named global table to a freshly exported ticket. Port of
/// `TableHandleManager.FetchTable`. Returns the export ticket bytes.
pub async fn fetch_table(server: &Server, table_name: &str) -> Result<Vec<u8>> {
    let export = server.new_export_ticket();

    let request = FetchTableRequest {
        source_id: Some(TableReference {
            r#ref: Some(table_reference::Ref::Ticket(Ticket {
                ticket: scope_reference(table_name),
            })),
        }),
        result_id: Some(Ticket { ticket: export.clone() }),
    };

    let mut table = TableServiceClient::new(server.channel());
    let response = table
        .fetch_table(server.authorize(request)?)
        .await?
        .into_inner();

    if !response.success && !response.error_info.is_empty() {
        return Err(Error::Server(response.error_info));
    }

    Ok(export)
}

/// `DoGet` the given ticket and collect the Arrow record batches. Port of
/// `TableHandle.GetFlightStream` + `ToArrowTableAsync`.
pub async fn do_get(server: &Server, ticket: Vec<u8>) -> Result<Snapshot> {
    let mut flight = FlightServiceClient::new(server.channel());
    let request = server.authorize(FlightTicket { ticket: ticket.into() })?;

    let mut stream = flight.do_get(request).await?.into_inner();
    let mut data = Vec::new();
    while let Some(message) = stream.next().await {
        data.push(message?);
    }

    let batches =
        flight_data_to_batches(&data).map_err(|e| Error::Arrow(e.to_string()))?;
    let schema = batches
        .first()
        .map(|b| b.schema())
        .unwrap_or_else(|| Arc::new(Schema::empty()));

    Ok(Snapshot { schema, batches })
}

/// Convenience: resolve a named table and snapshot it. Combines
/// [`fetch_table`] and [`do_get`].
pub async fn snapshot(server: &Server, table_name: &str) -> Result<Snapshot> {
    let ticket = fetch_table(server, table_name).await?;
    do_get(server, ticket).await
}
