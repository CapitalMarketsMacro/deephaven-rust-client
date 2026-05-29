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

    /// A space-mapper invariant was violated (duplicate insert, missing keys,
    /// or a shift that changed the set size).
    #[error("space mapper: {0}")]
    SpaceMapper(String),

    /// The endpoint string could not be parsed into a valid URI.
    #[cfg(feature = "client")]
    #[error("invalid endpoint URI: {0}")]
    InvalidUri(String),

    /// A header name or value could not be represented as gRPC metadata.
    #[cfg(feature = "client")]
    #[error("invalid request metadata: {0}")]
    Metadata(String),

    /// The server's response did not include an `authorization` token header.
    #[cfg(feature = "client")]
    #[error("server response did not contain an authorization token")]
    MissingAuthToken,

    /// gRPC transport/connection failure.
    #[cfg(feature = "client")]
    #[error("transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    /// A gRPC call returned a non-OK status.
    #[cfg(feature = "client")]
    #[error("grpc status: {0}")]
    Status(#[from] tonic::Status),

    /// The server reported a failure resolving/creating a table.
    #[cfg(feature = "client")]
    #[error("server error: {0}")]
    Server(String),

    /// Failure decoding Arrow Flight data into record batches.
    #[cfg(feature = "client")]
    #[error("arrow decode error: {0}")]
    Arrow(String),
}

/// Crate-wide result alias.
pub type Result<T> = std::result::Result<T, Error>;
