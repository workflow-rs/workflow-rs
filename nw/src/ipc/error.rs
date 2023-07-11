use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen::prelude::JsValue;
use workflow_core::channel::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Error: {0}")]
    JsValue(String),

    #[error("IPC channel send error")]
    ChannelSendError,

    #[error("IPC channel receive error")]
    ChannelRecvError,

    #[error("BorshSerialize")]
    BorshSerialize,

    #[error("BorshDeserialize {0}")]
    BorshDeserialize(String),

    #[error(transparent)]
    IpcResponse(#[from] crate::ipc::error::ResponseError),

    #[error(transparent)]
    CallbackError(#[from] workflow_wasm::callback::CallbackError),
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

#[derive(
    Error, Debug, Clone, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
pub enum ResponseError {
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
    #[error("Receiver channel")]
    ReceiveChannelRx,
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
        ResponseError::Text(error)
    }
}

impl From<&str> for ResponseError {
    fn from(error: &str) -> Self {
        ResponseError::Text(error.to_string())
    }
}
