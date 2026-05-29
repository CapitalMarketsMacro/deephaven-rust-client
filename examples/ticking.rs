//! Generic ticking-table viewer: connect to any Deephaven server and stream a
//! table, printing each Barrage update and the current last row.
//!
//! Usage:
//!   cargo run --example ticking -- <server-url> <table-name> [psk]
//!
//! Examples:
//!   cargo run --example ticking -- http://your-server:10000 your_table your_psk
//!   cargo run --example ticking -- localhost:10000 ticking_demo DeephavenRustDev123
//!
//! Auth: pass a PSK as the third argument (or set DH_PSK); otherwise anonymous.
//! Duration: set DH_SECONDS (default 60; 0 = run until the stream ends / Ctrl-C).

use std::time::{Duration, Instant};

use arrow::record_batch::RecordBatch;
use arrow::util::display::{ArrayFormatter, FormatOptions};
use deephaven_rs::auth::{ClientOptions, Server};
use deephaven_rs::subscribe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let url = args.next().ok_or(
        "usage: cargo run --example ticking -- <server-url> <table-name> [psk]",
    )?;
    let table = args.next().ok_or(
        "usage: cargo run --example ticking -- <server-url> <table-name> [psk]",
    )?;
    let psk = args.next().or_else(|| std::env::var("DH_PSK").ok());

    let seconds: u64 = std::env::var("DH_SECONDS").ok().and_then(|s| s.parse().ok()).unwrap_or(60);

    let options = match &psk {
        Some(p) => {
            println!("Connecting to {url} (PSK auth)...");
            ClientOptions::psk(p)
        }
        None => {
            println!("Connecting to {url} (anonymous auth)...");
            ClientOptions::anonymous()
        }
    };

    let server = Server::connect(&url, options).await?;
    println!("Connected. Subscribing to '{table}'...\n");

    let mut subscription = subscribe::subscribe(&server, &table).await?;

    let deadline = (seconds > 0).then(|| Instant::now() + Duration::from_secs(seconds));
    let mut schema_printed = false;
    let mut update_count = 0u64;

    loop {
        if let Some(d) = deadline {
            if Instant::now() >= d {
                break;
            }
        }

        match tokio::time::timeout(Duration::from_secs(15), subscription.next_update()).await {
            Ok(Ok(Some(update))) => {
                update_count += 1;

                // Print the schema once, on the first (snapshot) update.
                if !schema_printed {
                    if let Ok(batch) = subscription.snapshot() {
                        print_schema(&batch);
                    }
                    schema_printed = true;
                }

                let kind = if update.is_snapshot { "snapshot" } else { "delta" };
                let mods: u64 = update.modified_per_column.iter().map(|m| m.count()).sum();
                println!(
                    "#{update_count:<4} [{kind:>8}] rows={:<6} (+{} -{} ~{})  server={}",
                    update.num_rows,
                    update.added.count(),
                    update.removed.count(),
                    mods,
                    update.table_size,
                );
                if let Ok(batch) = subscription.snapshot() {
                    if let Some(row) = format_row(&batch, batch.num_rows().wrapping_sub(1)) {
                        println!("      last row: {row}");
                    }
                }
            }
            Ok(Ok(None)) => {
                println!("stream ended");
                break;
            }
            Ok(Err(e)) => {
                eprintln!("error: {e}");
                break;
            }
            Err(_) => {
                println!("(no update in 15s)");
            }
        }
    }

    println!("\nDone. {update_count} update(s); final row count = {}.", subscription.num_rows());
    Ok(())
}

fn print_schema(batch: &RecordBatch) {
    println!("Schema ({} columns):", batch.num_columns());
    for field in batch.schema().fields() {
        println!("  {:<24} {}", field.name(), field.data_type());
    }
    println!();
}

/// Render row `row` as `col=value` pairs.
fn format_row(batch: &RecordBatch, row: usize) -> Option<String> {
    if batch.num_rows() == 0 || row >= batch.num_rows() {
        return None;
    }
    let opts = FormatOptions::default();
    let cells: Vec<String> = batch
        .schema()
        .fields()
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let value = ArrayFormatter::try_new(batch.column(i), &opts)
                .map(|fmt| fmt.value(row).to_string())
                .unwrap_or_else(|_| "?".to_string());
            format!("{}={}", f.name(), value)
        })
        .collect();
    Some(cells.join(", "))
}
