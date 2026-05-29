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
#[cfg(feature = "client")]
pub mod proto;

#[cfg(feature = "client")]
pub mod auth;

#[cfg(feature = "client")]
pub mod flight;

#[cfg(feature = "client")]
pub mod console;

/// planus-generated Barrage flatbuffer types. To regenerate:
/// `planus rust -o src/barrage_generated.rs ../barrage/format/Barrage.fbs`
/// then strip the unconditional serde derives the CLI emits (serde is not a
/// dependency): `sed -i '/::serde::Serialize,/d; /::serde::Deserialize,/d'`.
#[cfg(feature = "client")]
#[allow(warnings, clippy::all)]
pub mod barrage_generated;

#[cfg(feature = "client")]
pub mod barrage;
