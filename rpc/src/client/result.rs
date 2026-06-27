//!
//! Client [`Result`] enum encapsulating client [`Error`]
//!
use super::error::Error;
/// Result type returned by client operations, parameterized over the client [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
