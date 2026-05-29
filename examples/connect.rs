//! Phase 1 acceptance: connect to the local Deephaven server with PSK auth and
//! print a configuration constant.
//!
//! Env: DH_HOST (default localhost:10000), DH_PSK (default DeephavenRustDev123).

use deephaven_rs::auth::{ClientOptions, Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::var("DH_HOST").unwrap_or_else(|_| "localhost:10000".to_string());
    let psk = std::env::var("DH_PSK").unwrap_or_else(|_| "DeephavenRustDev123".to_string());

    println!("Connecting to {host} (PSK auth)...");
    let server = Server::connect(&host, ClientOptions::psk(&psk)).await?;
    println!("Connected. Session token acquired.");

    let constants = server.configuration_constants().await?;
    println!("Server returned {} configuration constants.", constants.len());

    // Print a stable, illustrative constant.
    if let Some(v) = constants.get("http.session.durationMs") {
        println!("  http.session.durationMs = {v}");
    } else if let Some((k, v)) = constants.iter().next() {
        println!("  {k} = {v}");
    }

    Ok(())
}
