//!
//! [`enum@Error`] enum declaration for server-side WebSocket errors.
//!
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

/// Errors produced by the [`WebSocketServer`](super::WebSocketServer).
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Other(String),

    #[error("{0}")]
    Listen(String),

    /// Indicates that no messages have been received
    /// within the specified timeout period
    #[error("Connection timeout")]
    ConnectionTimeout,

    /// Indicates that the data received is not a
    /// valid handshake message
    #[error("Malformed handshake message")]
    MalformedHandshake,

    /// Indicates that the data received is not a
    /// valid or acceptable message
    #[error("Malformed handshake message")]
    MalformedMessage,

    /// Indicates handler negotiation failure
    /// This error code is reserved for structs
    /// implementing WebSocket handler.
    #[error("Negotiation failure")]
    NegotiationFailure,

    /// Indicates handler negotiation failure
    /// with a specific reason
    /// This error code is reserved for structs
    /// implementing WebSocket handler.
    #[error("Negotiation failure: {0}")]
    NegotiationFailureWithReason(String),

    /// Error sending response via the
    /// tokio mspc response channel
    #[error("Response channel send error {0:?}")]
    ResponseChannelError(#[from] SendError<tungstenite::Message>),

    /// WebSocket error produced by the underlying
    /// Tungstenite WebSocket crate
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tungstenite::Error),

    /// Connection terminated absormally
    #[error("Connection closed abnormally")]
    AbnormalClose,

    /// Server closed connection
    #[error("Server closed connection")]
    ServerClose,

    #[error("Error signaling listener shutdown: {0}")]
    Stop(String),
    #[error("Error signaling listener shutdown: {0}")]
    Done(String),
    #[error("Error waiting for listener shutdown: {0}")]
    Join(String),
    // #[error(transparent)]
    // ChannelSendError(#[from] tokio::sync::mpsc::error::SendError<tungstenite::Message>),
}
