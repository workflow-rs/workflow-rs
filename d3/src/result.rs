/// Result type alias for this crate, using [`crate::error::Error`] as the error type.
pub type Result<T> = std::result::Result<T, crate::error::Error>;
