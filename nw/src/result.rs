/// Result type used throughout `workflow-nw`, with [`crate::error::Error`] as the error type.
pub type Result<T> = std::result::Result<T, crate::error::Error>;
