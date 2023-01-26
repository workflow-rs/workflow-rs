//! Result module encapsulating [`Result`] enum used
//! by the [`WebSocketServer`](super::WebSocketServer)
use super::error::Error;

/// [`Result`] encapsulating [`Error`] produced
/// by the [`WebSocketServer`](super::WebSocketServer)
pub type Result<T> = std::result::Result<T, Error>;
