/// Convenience `Result` alias whose error type is this crate's [`Error`](crate::error::Error).
pub type Result<T> = std::result::Result<T, crate::error::Error>;
