//!
//! Client [`Result`] enum encapsulating client [`Error`]
//!
use super::error::Error;
pub type Result<T> = std::result::Result<T, Error>;
