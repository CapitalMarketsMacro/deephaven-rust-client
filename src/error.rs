//! Library error type. One `thiserror` enum for all fallible library paths
//! (CLAUDE.md ground rule: no `unwrap()`/`expect()` outside tests).
//!
//! Network/protocol variants are added behind `#[cfg(feature = "client")]` as
//! later phases introduce them; the codec variants below are always present so
//! `src/rowset.rs` builds with `--no-default-features`.

use thiserror::Error;

/// All errors produced by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A RowSet delta buffer ended before a value it promised could be read.
    #[error("rowset decode: unexpected end of buffer")]
    UnexpectedEof,

    /// A RowSet delta buffer contained an unrecognized command/value tag.
    #[error("rowset decode: bad command byte {0:#04x}")]
    BadCommand(u8),
}

/// Crate-wide result alias.
pub type Result<T> = std::result::Result<T, Error>;
