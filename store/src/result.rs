//! [`Result`] type alias
/// Result type alias used throughout this crate, defaulting the error type
/// to the crate's [`Error`](crate::error::Error).
pub type Result<T> = std::result::Result<T, crate::error::Error>;
