use crate::client::error::Error;

/// Result type alias for client-side WebSocket operations.
pub type Result<T> = std::result::Result<T, Error>;
