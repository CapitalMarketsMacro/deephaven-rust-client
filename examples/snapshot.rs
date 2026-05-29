//! Phase 2 acceptance: snapshot a named static table and print its schema and
//! row count.
//!
//! Usage: cargo run --example snapshot -- <TableName>
//! Env: DH_HOST (default localhost:10000), DH_PSK (default DeephavenRustDev123).

use deephaven_rs::auth::{ClientOptions, Server};
use deephaven_rs::flight;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table_name = std::env::args().nth(1).ok_or(
        "usage: cargo run --example snapshot -- <TableName>",
    )?;
    let host = std::env::var("DH_HOST").unwrap_or_else(|_| "localhost:10000".to_string());
    let psk = std::env::var("DH_PSK").unwrap_or_else(|_| "DeephavenRustDev123".to_string());

    println!("Connecting to {host}...");
    let server = Server::connect(&host, ClientOptions::psk(&psk)).await?;

    println!("Fetching snapshot of table '{table_name}'...");
    let snapshot = flight::snapshot(&server, &table_name).await?;

    println!("\nSchema ({} columns):", snapshot.schema.fields().len());
    for field in snapshot.schema.fields() {
        println!("  {:<20} {}", field.name(), field.data_type());
    }
    println!(
        "\n{} row(s) across {} record batch(es).",
        snapshot.num_rows(),
        snapshot.batches.len()
    );

    Ok(())
}
