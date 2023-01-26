//!
//! [`Result`] enum encapsulating server [`Error`] enum.
//!
use super::error::Error;
pub type Result<T> = std::result::Result<T, Error>;
