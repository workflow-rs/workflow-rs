//!
//! [`Result`] encapsulating internal terminal [`Error`](super::error::Error)
//! and [`CliResult`] encapsulating a String error (for output in the terminal)
//!

/// Result type for internal terminal operations, carrying the crate
/// [`Error`](crate::error::Error) on failure.
pub type Result<T> = std::result::Result<T, crate::error::Error>;
/// Result type for CLI command handlers, carrying a `String` error message
/// to be displayed in the terminal on failure.
pub type CliResult<T> = std::result::Result<T, String>;
