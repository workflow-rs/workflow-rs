//!
//! Result enum encapsulating [`super::error::Error`]
//! enum common to client and server
//!

/// Result type returning the crate's general-purpose [`Error`](super::error::Error).
pub type Result<T> = std::result::Result<T, super::error::Error>;

// use super::messages::borsh::ServerError;
/// Result type returning a [`ServerError`](super::error::ServerError), used for
/// errors that originate on and are reported by the RPC server.
pub type ServerResult<T> = std::result::Result<T, super::error::ServerError>;
