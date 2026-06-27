//! [`enum@Error`] declarations for the [`server`](super) module

use thiserror::Error;
pub use workflow_websocket::server::Error as WebSocketError;

#[derive(Debug, Error)]
/// Errors produced by the RPC server.
pub enum Error {
    /// Error originating from the underlying WebSocket server.
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] workflow_websocket::server::error::Error),

    /// Failure to send a message over an internal channel (receiver dropped).
    #[error(transparent)]
    ChannelSendError(#[from] tokio::sync::mpsc::error::SendError<tungstenite::Message>),

    /// Underlying I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Error originating from a spawned background task.
    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),

    /// General RPC error propagated from the crate's core error type.
    #[error(transparent)]
    RpcError(#[from] crate::error::Error),

    /// JSON serialization or deserialization error.
    #[error("SerdeJSON error: {0}")]
    SerdeJSON(#[from] serde_json::Error),
}
