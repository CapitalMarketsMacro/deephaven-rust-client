//! deephaven-rs — a clean-room Rust client for Deephaven Community Core over
//! Arrow Flight + the Barrage flatbuffer extension.
//!
//! Ported from Deephaven's own C# client (`Dh_NetClient`). See `CLAUDE.md` for
//! the phased plan and the reference-file mapping.
//!
//! The pure-logic RowSet codec ([`rowset`]) and error type ([`error`]) are
//! always available. Networked modules live behind the `client` feature.

pub mod error;
pub mod rowset;

// Networked modules are introduced phase by phase behind the `client` feature.
// (auth, flight, barrage, space_mapper, table_state, subscribe)
