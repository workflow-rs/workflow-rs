//!
//! Common [`enum@Error`] definitions used by both [`super::client`] and [`super::server`] modules.
//!

use borsh::{BorshDeserialize, BorshSerialize};
use serde::*;
use std::sync::PoisonError;
use thiserror::Error;
use workflow_core::channel::{RecvError, SendError, TrySendError};

/// Errors shared by the wRPC client and server message-handling layers.
#[derive(Error, Debug)]
pub enum Error {
    /// Received message is smaller than the minimum header size
    #[error("Invalid header size")]
    HeaderSize,

    /// An underlying I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// An error originating from the underlying async task subsystem.
    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),

    /// The requested message encoding is unknown or unsupported.
    #[error("invalid encoding {0}")]
    Encoding(String),
}

///
/// [`ServerError`] enum is used by both Server and Client and
/// represents errors returned by server-side handlers. This enum
/// is also serialized and transported to the client when using
/// the `Borsh` protocol (as such, this mostly contains pure enum
/// values).
///
#[derive(
    Error, Debug, Clone, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
pub enum ServerError {
    /// The connection has been closed.
    #[error("connection is closed")]
    Close,
    /// The RPC call did not complete within the allotted time.
    #[error("RPC call timed out")]
    Timeout,
    /// No data was available where data was expected.
    #[error("no data")]
    NoData,
    /// No handler was registered for the requested RPC method.
    #[error("RPC method not found")]
    NotFound,
    /// A lock was poisoned by a panic in another thread.
    #[error("resource lock error")]
    PoisonError,
    /// The request was expected to use the Borsh protocol but did not.
    #[error("not a borsh request")]
    NonBorshRequest,
    /// The request was expected to use the serde/JSON protocol but did not.
    #[error("not a serde request")]
    NonSerdeRequest,
    /// The request could not be serialized.
    #[error("request serialization error")]
    ReqSerialize,
    /// The request could not be deserialized.
    #[error("request deserialization error")]
    ReqDeserialize,
    /// The response could not be serialized.
    #[error("response serialization error")]
    RespSerialize,
    /// A notification payload could not be deserialized.
    #[error("request deserialization error")]
    NotificationDeserialize(String),
    /// A response payload could not be deserialized.
    #[error("response deserialization error")]
    RespDeserialize(String),
    /// Opaque binary error payload.
    #[error("data")]
    Data(Vec<u8>),
    /// Free-form textual error message.
    #[error("{0}")]
    Text(String),
    /// Underlying WebSocket error
    #[error("WebSocket -> {0}")]
    WebSocketError(String),
    /// Failure receiving from an internal receiver channel.
    #[error("Receiver channel")]
    ReceiveChannelRx,
    /// Failure sending to an internal receiver channel.
    #[error("Receiver channel send")]
    ReceiveChannelTx,
}

impl From<std::io::Error> for ServerError {
    fn from(_err: std::io::Error) -> Self {
        ServerError::RespSerialize
    }
}

impl<T> From<PoisonError<T>> for ServerError {
    fn from(_error: PoisonError<T>) -> ServerError {
        ServerError::PoisonError
    }
}

impl From<String> for ServerError {
    fn from(error: String) -> Self {
        ServerError::Text(error)
    }
}

impl From<&str> for ServerError {
    fn from(error: &str) -> Self {
        ServerError::Text(error.to_string())
    }
}

// impl From<serde_json::Error> for ServerError

// impl de::Error for Error {
//     fn custom<T: Display>(msg: T) -> Error {
//         Error::SerdeDeserialize(msg.to_string())
//     }
// }

// impl ser::Error for Error {
//     fn custom<T: Display>(msg: T) -> Error {
//         Error::SerdeSerialize(msg.to_string())
//     }
// }

impl From<workflow_websocket::client::Error> for ServerError {
    fn from(error: workflow_websocket::client::Error) -> Self {
        ServerError::WebSocketError(error.to_string())
    }
}

impl From<RecvError> for ServerError {
    fn from(_: RecvError) -> ServerError {
        ServerError::ReceiveChannelRx
    }
}

impl<T> From<SendError<T>> for ServerError {
    fn from(_error: SendError<T>) -> ServerError {
        ServerError::ReceiveChannelTx
    }
}

impl<T> From<TrySendError<T>> for ServerError {
    fn from(_error: TrySendError<T>) -> ServerError {
        ServerError::ReceiveChannelTx
    }
}
