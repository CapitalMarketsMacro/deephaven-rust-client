//! Build script: generate client-side gRPC stubs from the vendored Deephaven
//! protos. Barrage flatbuffer codegen is added in Phase 3.
//!
//! Codegen is gated two ways so the pure-logic codec (`src/rowset.rs`) keeps
//! building in isolation:
//!   * only when the `client` feature is enabled (CARGO_FEATURE_CLIENT), and
//!   * only when the proto files are actually present on disk.

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Skip everything unless the networked `client` feature is active.
    if std::env::var_os("CARGO_FEATURE_CLIENT").is_none() {
        return Ok(());
    }

    let proto_root = Path::new("proto");
    // Client-side only; service order mirrors the vendored set. Console is in
    // the `script.grpc` package, the rest in `grpc`.
    let protos = [
        "deephaven_core/proto/ticket.proto",
        "deephaven_core/proto/config.proto",
        "deephaven_core/proto/application.proto",
        "deephaven_core/proto/object.proto",
        "deephaven_core/proto/session.proto",
        "deephaven_core/proto/table.proto",
        "deephaven_core/proto/console.proto",
    ];

    let full: Vec<_> = protos.iter().map(|p| proto_root.join(p)).collect();

    // If the protos haven't been vendored yet, do nothing (codec stays standalone).
    if !full.iter().all(|p| p.exists()) {
        return Ok(());
    }

    for p in &full {
        println!("cargo:rerun-if-changed={}", p.display());
    }
    println!("cargo:rerun-if-changed=proto");

    tonic_build::configure()
        .build_server(false)
        .compile_protos(&full, &[proto_root])?;

    Ok(())
}
