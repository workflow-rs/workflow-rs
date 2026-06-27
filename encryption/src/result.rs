/// Result type alias using this crate's [`Error`](crate::error::Error).
pub type Result<T> = std::result::Result<T, crate::error::Error>;
