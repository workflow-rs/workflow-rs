/// Result type alias whose error variant is the crate's [`Error`](crate::error::Error).
pub type Result<T> = std::result::Result<T, crate::error::Error>;
