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

    // tonic-build/prost-build invoke `protoc`, discovered via the PROTOC env var
    // or `protoc` on PATH. Emit a clear hint before the (less obvious) failure
    // if neither is available.
    if !protoc_available() {
        println!(
            "cargo:warning=protoc not found. Install Protocol Buffers and ensure \
             `protoc` is on PATH, or set the PROTOC env var to its full path. \
             See https://grpc.io/docs/protoc-installation/"
        );
    }

    tonic_build::configure()
        .build_server(false)
        .compile_protos(&full, &[proto_root])?;

    Ok(())
}

/// Whether a `protoc` binary can be located the way prost-build will look for it:
/// the `PROTOC` env var, or `protoc` on `PATH`.
fn protoc_available() -> bool {
    if std::env::var_os("PROTOC").is_some() {
        return true;
    }
    let exe = if cfg!(windows) { "protoc.exe" } else { "protoc" };
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join(exe).is_file()))
        .unwrap_or(false)
}
