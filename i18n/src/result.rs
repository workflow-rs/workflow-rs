/// Result type returning the crate's [`Error`](crate::error::Error) on failure.
pub type Result<T> = std::result::Result<T, crate::error::Error>;
