//!
//! [`Result`] enum encapsulating server [`Error`] enum.
//!
use super::error::Error;
/// Result type returning the server module's [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
