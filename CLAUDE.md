# CLAUDE.md — deephaven-rs

Project instructions for Claude Code. Read this fully before writing code.

## Mission

Build a **clean-room Rust client for Deephaven Community Core** over Arrow
Flight + the Barrage flatbuffer extension. There is no official Rust client, so
we port Deephaven's own clean-room C# client (`Dh_NetClient`) to Rust. We are
**translating a reference implementation, not reverse-engineering**.

Two capability tiers, build in this order:
1. **Snapshot client** — authenticate, get a table ticket, `DoGet`, decode Arrow. (Easy; do first.)
2. **Ticking client** — `DoExchange` + Barrage delta decode + local table state. (The real work.)

## Current state

- `src/rowset.rs` — DONE. The Barrage RowSet "external compressed delta" codec,
  a faithful port of `RowSequenceDecoder.cs`, with passing unit tests. **This is
  the source of truth for row-key decoding. Do not rewrite it; build on it.**
- Everything else is unbuilt. `Cargo.toml` currently has zero deps so the codec
  tests run standalone.

## Ground rules (non-negotiable)

1. **Port, don't invent.** Every protocol module must correspond to a specific
   file in the C# reference client. At the top of each new module, add a comment
   naming the reference file you ported (e.g. `// Port of BarrageProcessor.cs`).
   If you can't find the reference behavior, STOP and say so — do not guess wire
   formats.
2. **Keep `cargo test` green at every commit.** Never break existing tests.
3. **Approved dependencies only:** `tonic`, `prost`, `tonic-build`, `arrow`,
   `arrow-flight`, `planus`, `tokio`, `futures`, `bytes`, `thiserror`. Adding
   anything else requires asking the user first.
4. **No `unwrap()`/`expect()` in library paths** except in tests. Use a
   `thiserror` error enum (`src/error.rs`).
5. **Work one phase at a time.** Do not start a phase until the previous phase's
   acceptance criteria pass. Show the verification output before moving on.
6. **Faithful order matters.** The Barrage update application order
   (removes → shifts → adds → modifies) is load-bearing. Never reorder it.

## One-time setup (run these first)

```bash
# 1. Reference client + protos — clone shallow next to this repo, do NOT vendor
#    the whole thing into git; we only read from it.
git clone --filter=blob:none --depth 1 \
  https://github.com/deephaven/deephaven-core.git ../deephaven-core

# 2. Barrage flatbuffer schema lives in a separate repo.
git clone --depth 1 https://github.com/deephaven/barrage.git ../barrage
#    The schema is the .fbs under ../barrage (e.g. format/schema). If you cannot
#    locate it, the C# generated classes in
#    ../deephaven-core/csharp/client/Dh_NetClient/flatbuf/*.cs encode the exact
#    field order and are an acceptable schema reference.

# 3. Local Deephaven server for integration tests (PSK auth).
docker run --rm --name deephaven -p 10000:10000 -v "$(pwd)/data:/data" \
  --env START_OPTS="-Dauthentication.psk=DeephavenRustDev123" \
  ghcr.io/deephaven/server:latest
#    IDE at http://localhost:10000/ide/  (token: DeephavenRustDev123)
#    In the IDE console, create a ticking test table:
#       from deephaven import time_table
#       t = time_table("PT1S").update(["X = i", "Y = i*i"])
```

### Protos to vendor into `proto/` (copy from the reference clone)

From `../deephaven-core/proto/proto-backplane-grpc/src/main/proto/deephaven_core/proto/`:
`config.proto`, `session.proto`, `table.proto`, `console.proto`, `ticket.proto`,
`application.proto`, `object.proto`, plus any they `import`. **Arrow's
`Flight.proto` is already provided by the `arrow-flight` crate — do not vendor it.**

## Build configuration

`build.rs` must:
- run `tonic_build` over the vendored `proto/*.proto` (client-side only:
  `.build_server(false)`),
- run `planus` codegen over the Barrage `.fbs` into `src/barrage_generated.rs`
  (pure-Rust; no `flatc` binary dependency).

Gate codegen so `cargo test` for `rowset` still works if protos are absent —
i.e. only invoke proto/fbs codegen when the files exist, and `cfg`-gate the
networked modules behind a default feature `client` so the codec can always be
tested in isolation.

## Protocol cheat-sheet (verified facts)

**Auth handshake (unary, not a streaming login).**
Open the gRPC channel, then call `ConfigService.GetConfigurationConstants` with
an `authorization` *request-header*:
- Anonymous: `Anonymous`
- PSK: `io.deephaven.authentication.psk.PskAuthenticationHandler DeephavenRustDev123`
- Basic: `Basic <base64(user:pass)>`

The **session token comes back in the response headers** under `authorization`.
Use that token as the `authorization` header on all later calls. Refresh on a
keepalive timer before it expires. (Ref: `Server.cs`.)

**Get a ticket.** Use `ConsoleService` (start session, run script, fetch the
exported variable) or `TableService` batch ops. Result is a `Ticket` (opaque
bytes). (Ref: `Server.cs`, generated services.)

**Subscribe (DoExchange).** (Ref: `SubscriptionThread.cs`, `BarrageProcessor.cs`.)
- `FlightDescriptor::command(b"dphn")`, open `DoExchange` with auth metadata.
- Write ONE priming message: a throwaway 1-column Arrow `RecordBatch` whose
  `app_metadata` = the subscription request bytes below. The batch data is
  ignored; the metadata is what matters.

**Subscription request = nested flatbuffers.**
Inner `BarrageSubscriptionRequest { ticket, columns?, viewport?, subscription_options }`
wrapped in `BarrageMessageWrapper`:
- `magic = 0x6E687064`  (ASCII `"dphn"`)
- `msg_type = BarrageSubscriptionRequest`
- `msg_payload = <inner request bytes>`
- omit `columns` → all columns; omit `viewport` → entire table.

**Decode each update.** First `FlightData` of an update carries `app_metadata`
= `BarrageMessageWrapper` → `BarrageUpdateMetadata`. Validate `magic`, then
decode row-key blobs with `rowset::read_external_compressed_delta`:
- `removed_rows` → 1 RowSequence
- `shift_data`   → **3** RowSequences read from the SAME buffer: start, end, dest
- `added_rows`   → 1 RowSequence
- each `mod_column_nodes[i].modified_rows` → 1 RowSequence

`num_add_batches` / `num_mod_batches` say how many following RecordBatches carry
add / mod cell data.

**Apply in EXACTLY this order:** `1. removes  2. shifts  3. adds  4. modifies`.

## Phased plan with acceptance gates

### Phase 0 — scaffold
- Add `src/error.rs` (thiserror enum), wire `lib.rs` modules, add the `client`
  feature, configure `build.rs` (codegen gated as above).
- **Accept:** `cargo test --no-default-features` runs the `rowset` tests green.

### Phase 1 — gRPC + auth
- `src/auth.rs`: open channel, `GetConfigurationConstants` handshake, extract
  session token from response headers, keepalive refresh task. (Port `Server.cs`.)
- **Accept:** an example (`examples/connect.rs`) connects to the local docker
  server with PSK and prints a config constant. Run it; show output.

### Phase 2 — snapshot (the easy win)
- `src/flight.rs`: resolve a ticket for a named global table, `DoGet`, collect
  Arrow `RecordBatch`es, print schema + row count.
- **Accept:** `examples/snapshot.rs -- TableName` prints the schema and N rows
  of a static table you created in the IDE. Show output.

### Phase 3 — Barrage codegen + request build
- `src/barrage.rs`: planus-generated types; `create_subscription_request(ticket)`
  building the nested wrapper with the correct magic/type. (Port `BarrageProcessor.cs::CreateSubscriptionRequest`.)
- **Accept:** unit test asserts the produced bytes parse back to a
  `BarrageMessageWrapper` with `magic == 0x6E687064` and the right msg type, and
  the inner ticket round-trips.

### Phase 4 — table state
- `src/space_mapper.rs` (port `SpaceMapper.cs`): key-space ↔ position-space map.
- `src/table_state.rs` (port `TableState.cs`): add/erase keys, `apply_shifts`,
  snapshot. Uses `rowset::RowSequence`.
- **Accept:** unit tests porting the C# ticking tests for shifts/erase. Show green.

### Phase 5 — subscription loop
- `src/subscribe.rs` (port `SubscriptionThread.cs` + `BarrageProcessor` state
  machine): prime DoExchange, read stream, decode metadata, apply
  removes→shifts→adds→modifies, surface a stream of typed updates to the caller.
- **Accept:** `examples/subscribe.rs -- TimeTableName` connects to the local
  ticking `time_table` and prints coherent row counts that grow once per second
  with no panics for ≥30s. Show a sample of the output.

## Testing strategy

- **Unit:** pure-logic modules (`rowset`, `space_mapper`, `table_state`,
  `barrage` request build) — no network. Port the corresponding C# tests where
  they exist (`Dh_NetClientTests/`).
- **Integration:** `tests/` and `examples/` against the local docker server.
  Read host/PSK from env: `DH_HOST=localhost:10000`, `DH_PSK=DeephavenRustDev123`.
  Skip (don't fail) integration tests when `DH_HOST` is unset.

## Pitfalls (from the reference, learned the hard way)

- RowSet blobs are little-endian; a buffered value flushes as a singleton on the
  next positive value or as an interval *end* on the next negative value. This is
  already handled in `rowset.rs` — match its semantics elsewhere.
- `shift_data` is three RowSequences in one buffer; read sequentially from the
  same cursor, do not re-slice.
- Reordering the four update steps silently corrupts state hours later. Don't.
- The priming DoExchange message's RecordBatch payload is irrelevant; only its
  `app_metadata` matters. Don't waste time on it.
- Don't vendor Arrow's `Flight.proto`; `arrow-flight` owns it.

## First prompt to give Claude Code

> Read CLAUDE.md. Run the one-time setup (clone references, start the Deephaven
> docker server, vendor the protos). Then do Phase 0 only and show me the green
> `cargo test --no-default-features` output before proposing Phase 1. Do not skip
> ahead.
