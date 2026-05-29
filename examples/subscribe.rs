//! Phase 5 acceptance: subscribe to a ticking table and print row counts that
//! grow once per second for >= 30s, with no panics.
//!
//! Usage: cargo run --example subscribe -- <TableName>   (default: ticking_demo)
//! Env: DH_HOST (default localhost:10000), DH_PSK (default DeephavenRustDev123).

use std::time::{Duration, Instant};

use deephaven_rs::auth::{ClientOptions, Server};
use deephaven_rs::subscribe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table_name = std::env::args().nth(1).unwrap_or_else(|| "ticking_demo".to_string());
    let host = std::env::var("DH_HOST").unwrap_or_else(|_| "localhost:10000".to_string());
    let psk = std::env::var("DH_PSK").unwrap_or_else(|_| "DeephavenRustDev123".to_string());

    println!("Connecting to {host}...");
    let server = Server::connect(&host, ClientOptions::psk(&psk)).await?;

    println!("Subscribing to '{table_name}' (will run ~35s)...\n");
    let mut subscription = subscribe::subscribe(&server, &table_name).await?;

    let deadline = Instant::now() + Duration::from_secs(35);
    let mut update_count = 0u64;

    while Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_secs(10), subscription.next_update()).await {
            Ok(Ok(Some(update))) => {
                update_count += 1;
                let kind = if update.is_snapshot { "snapshot" } else { "delta" };
                let tail = subscription.snapshot().ok().map(|b| format_last_row(&b)).unwrap_or_default();
                println!(
                    "update #{update_count:<3} [{kind:>8}]  rows={:<5} (+{} -{})  server_size={}  last_row=[{tail}]",
                    update.num_rows,
                    update.added.count(),
                    update.removed.count(),
                    update.table_size,
                );
            }
            Ok(Ok(None)) => {
                println!("stream ended");
                break;
            }
            Ok(Err(e)) => {
                println!("error: {e}");
                break;
            }
            Err(_) => {
                println!("(no update in 10s)");
            }
        }
    }

    println!("\nDone. Received {update_count} update(s); final row count = {}.", subscription.num_rows());
    Ok(())
}

/// Render the last row of a record batch as `col=value` pairs, to show that
/// real cell data is flowing into the local table state.
fn format_last_row(batch: &arrow::record_batch::RecordBatch) -> String {
    use arrow::util::display::{ArrayFormatter, FormatOptions};
    if batch.num_rows() == 0 {
        return String::new();
    }
    let row = batch.num_rows() - 1;
    let opts = FormatOptions::default();
    batch
        .schema()
        .fields()
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let cell = ArrayFormatter::try_new(batch.column(i), &opts)
                .map(|fmt| fmt.value(row).to_string())
                .unwrap_or_else(|_| "?".to_string());
            format!("{}={}", f.name(), cell)
        })
        .collect::<Vec<_>>()
        .join(", ")
}
