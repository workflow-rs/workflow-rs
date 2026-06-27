//! [`Result`] enum encapsulating the node [`Error`](crate::error::Error) enum

/// Result type used throughout this crate, with the error fixed to the node
/// [`Error`](crate::error::Error) enum.
pub type Result<T> = std::result::Result<T, crate::error::Error>;
