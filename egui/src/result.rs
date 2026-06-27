/// Convenience result type used throughout the crate, with the error fixed to
/// the crate's own [`Error`](crate::error::Error).
pub type Result<T> = std::result::Result<T, crate::error::Error>;
