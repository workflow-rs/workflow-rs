use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen::prelude::JsValue;
use workflow_core::channel::*;

/// Errors produced by the IPC subsystem.
#[derive(Error, Debug)]
pub enum Error {
    /// A custom, free-form error message.
    #[error("{0}")]
    Custom(String),

    /// Underlying I/O error.
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    /// A JavaScript exception captured as a string.
    #[error("Error: {0}")]
    JsValue(String),

    /// Failed to send a message over an IPC channel.
    #[error("IPC channel send error")]
    ChannelSendError,

    /// Failed to receive a message from an IPC channel.
    #[error("IPC channel receive error")]
    ChannelRecvError,

    /// Borsh serialization of a message failed.
    #[error("BorshSerialize")]
    BorshSerialize,

    /// Borsh deserialization of a message failed.
    #[error("BorshDeserialize {0}")]
    BorshDeserialize(String),

    /// Error contained within an IPC response.
    #[error(transparent)]
    IpcResponse(#[from] crate::ipc::error::ResponseError),

    /// Error originating from a WASM callback invocation.
    #[error(transparent)]
    CallbackError(#[from] workflow_wasm::callback::CallbackError),

    /// Generic channel error described by a message.
    #[error("{0}")]
    ChannelError(String),
}

impl From<String> for Error {
    fn from(v: String) -> Self {
        Self::Custom(v)
    }
}

impl From<&str> for Error {
    fn from(v: &str) -> Self {
        Self::Custom(v.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(v: JsValue) -> Self {
        Self::JsValue(format!("{v:?}"))
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> JsValue {
        let s: String = err.to_string();
        JsValue::from_str(&s)
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(_: TrySendError<T>) -> Self {
        Error::ChannelSendError
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Error::ChannelRecvError
    }
}

impl<T> From<ChannelError<T>> for Error {
    fn from(e: ChannelError<T>) -> Error {
        Error::ChannelError(e.to_string())
    }
}

/// Errors returned to the caller as part of an IPC response.
#[derive(
    Error, Debug, Clone, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
pub enum ResponseError {
    /// The connection was closed before a response was received.
    #[error("connection is closed")]
    Close,
    /// The RPC call timed out before completing.
    #[error("RPC call timed out")]
    Timeout,
    /// The response contained no data.
    #[error("no data")]
    NoData,
    /// The requested IPC method handler was not found.
    #[error("IPC method not found")]
    NotFound,
    /// A mutex/lock was poisoned while handling the request.
    #[error("resource lock error")]
    PoisonError,
    /// The request payload was not Borsh-encoded as expected.
    #[error("not a borsh request")]
    NonBorshRequest,
    /// The request payload was not Serde-encoded as expected.
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
    /// The response could not be deserialized.
    #[error("response deserialization error")]
    RespDeserialize(String),
    /// Raw response data carried alongside the error.
    #[error("data")]
    Data(Vec<u8>),
    /// A custom, free-form error message.
    #[error("{0}")]
    Custom(String),
    /// Underlying WebSocket error
    #[error("Receiver channel")]
    ReceiveChannelRx,
    /// Failed to forward a message into the receiver channel.
    #[error("Receiver channel send")]
    ReceiveChannelTx,
}

impl From<std::io::Error> for ResponseError {
    fn from(_err: std::io::Error) -> Self {
        ResponseError::RespSerialize
    }
}

// impl<T> From<PoisonError<T>> for ResponseError {
//     fn from(_error: PoisonError<T>) -> ResponseError {
//         ResponseError::PoisonError
//     }
// }

impl From<String> for ResponseError {
    fn from(error: String) -> Self {
        ResponseError::Custom(error)
    }
}

impl From<&str> for ResponseError {
    fn from(error: &str) -> Self {
        ResponseError::Custom(error.to_string())
    }
}
