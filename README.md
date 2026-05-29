# deephaven-rs

A clean-room Rust client for [Deephaven Community Core](https://deephaven.io)
over Arrow Flight + the Barrage flatbuffer extension. There is no official Rust
client, so this is a faithful port of Deephaven's own C# client
(`Dh_NetClient`) — each module names the reference file it ports.

Two capability tiers:

1. **Snapshot** — authenticate, resolve a table ticket, `DoGet`, decode Arrow.
2. **Ticking** — `DoExchange` + Barrage delta decode + local table state, applying
   updates in the load-bearing order **removes → shifts → adds → modifies**.

## Status

| Area | State |
|---|---|
| RowSet delta codec (`rowset`) | ✅ |
| gRPC channel + PSK/anonymous auth (`auth`) | ✅ |
| Snapshot via `FetchTable` + `DoGet` (`flight`) | ✅ |
| Barrage subscription request (`barrage`) | ✅ |
| Key space ↔ position mapping (`space_mapper`) | ✅ |
| Table state + Arrow cell-data movement (`table_state`) | ✅ |
| Subscription loop (`subscribe`) | ✅ |

Not yet implemented: Deephaven null-sentinel → Arrow-validity normalization;
Basic auth (Anonymous + PSK are supported).

## Build prerequisites

- A `protoc` binary (for `tonic-build`). Point cargo at it via `PROTOC`, or put
  it on `PATH`. (`.cargo/config.toml` in this repo sets a local Windows path —
  adjust it for your machine.)
- To regenerate the Barrage flatbuffers: `cargo install planus-cli`, then
  `planus rust -o src/barrage_generated.rs path/to/Barrage.fbs` and strip the
  emitted `::serde::` derives.

```bash
cargo build
cargo test                      # full suite
cargo test --no-default-features # pure-logic codec + space_mapper, no network
```

## Examples

All examples take connection details and require a running Deephaven server.

```bash
# Generic ticking viewer: stream any table from any server.
cargo run --example ticking -- http://your-server:10000 your_table your_psk

# Snapshot a static table.
cargo run --example snapshot -- your_table

# Auth handshake only.
cargo run --example connect
```

`ticking` reads the PSK from the 3rd argument or `DH_PSK` (anonymous if absent),
and `DH_SECONDS` controls how long to run (`0` = until Ctrl-C). `connect`/
`snapshot` read `DH_HOST` (default `localhost:10000`) and `DH_PSK`.

## License

Apache-2.0. Protocol definitions under `proto/` and the Barrage schema are
Deephaven's, used under their Apache-2.0 license.
