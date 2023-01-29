//!
//! Common [`enum@Error`] definitions used by both [`super::client`] and [`super::server`] modules.
//!

use borsh::{BorshDeserialize, BorshSerialize};
use serde::*;
use std::sync::PoisonError;
use thiserror::Error;
use workflow_core::channel::{RecvError, SendError, TrySendError};

#[derive(Error, Debug)]
pub enum Error {
    /// Received message is smaller than the minimum header size
    #[error("Invalid header size")]
    HeaderSize,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),
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
    #[error("connection is closed")]
    Close,
    #[error("RPC call timed out")]
    Timeout,
    #[error("no data")]
    NoData,
    #[error("RPC method not found")]
    NotFound,
    #[error("resource lock error")]
    PoisonError,
    #[error("not a borsh request")]
    NonBorshRequest,
    #[error("not a serde request")]
    NonSerdeRequest,
    #[error("request serialization error")]
    ReqSerialize,
    #[error("request deserialization error")]
    ReqDeserialize,
    #[error("response serialization error")]
    RespSerialize,
    #[error("request deserialization error")]
    NotificationDeserialize(String),
    #[error("response deserialization error")]
    RespDeserialize(String),
    #[error("data")]
    Data(Vec<u8>),
    #[error("{0}")]
    Text(String),
    /// Underlying WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    #[error("Receiver channel")]
    ReceiveChannelRx,
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
