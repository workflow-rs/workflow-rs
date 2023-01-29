//! [`enum@Error`] declarations for the [`server`](super) module

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] workflow_websocket::server::error::Error),

    #[error(transparent)]
    ChannelSendError(#[from] tokio::sync::mpsc::error::SendError<tungstenite::Message>),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),

    #[error(transparent)]
    RpcError(#[from] crate::error::Error),

    #[error("SerdeJSON error: {0}")]
    SerdeJSON(#[from] serde_json::Error),
}

// impl<T> From<SendError<T>> for Error {
//     fn from(_: SendError<T>) -> Self {
//         Error::ChannelSendError
//     }
// }
