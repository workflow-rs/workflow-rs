//!
//! [`Result`] encapsulating internal terminal [`Error`](super::error::Error)
//! and [`CliResult`] encapsulating a String error (for output in the terminal)
//!

pub type Result<T> = std::result::Result<T, crate::error::Error>;
pub type CliResult<T> = std::result::Result<T, String>;
