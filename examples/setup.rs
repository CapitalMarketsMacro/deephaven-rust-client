//! Create the demo tables used by the snapshot/subscribe examples, via a
//! Python console session. Run once after the server starts.
//!
//!   static_demo  — a 10-row static table (X, Y, Z)
//!   ticking_demo — a time_table that grows once per second (Timestamp, X, Y)
//!
//! Env: DH_HOST (default localhost:10000), DH_PSK (default DeephavenRustDev123).

use deephaven_rs::auth::{ClientOptions, Server};
use deephaven_rs::console::Console;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::var("DH_HOST").unwrap_or_else(|_| "localhost:10000".to_string());
    let psk = std::env::var("DH_PSK").unwrap_or_else(|_| "DeephavenRustDev123".to_string());

    let server = Server::connect(&host, ClientOptions::psk(&psk)).await?;
    let console = Console::start(&server, "python").await?;

    console
        .run_script(
            &server,
            "from deephaven import empty_table, time_table\n\
             static_demo = empty_table(10).update([\"X = i\", \"Y = i * i\", \"Z = (double) i / 2\"])\n\
             ticking_demo = time_table(\"PT1S\").update([\"X = i\", \"Y = i * i\"])\n",
        )
        .await?;

    println!("Created tables: static_demo (10 rows), ticking_demo (ticking).");
    Ok(())
}
