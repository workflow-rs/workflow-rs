//!
//! Client [`enum@Error`] enum declaration
//!

use crate::error::ServerError;
use crate::messages::serde_json::JsonServerError;
use serde::*;
use std::fmt::Display;
use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_core::channel::{RecvError, SendError, TrySendError};
pub use workflow_websocket::client::error::Error as WebSocketError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid event '{0}'")]
    InvalidEvent(String),

    #[error("Invalid URL {0}")]
    InvalidUrl(String),

    #[error(transparent)]
    RpcError(#[from] crate::error::Error),

    #[error("response handler for id {0} not found")]
    ResponseHandler(String),

    #[error("WebSocket disconnected")]
    Disconnect,

    #[error("Missing method in notification message")]
    NotificationMethod,

    #[error("invalid WebSocket message type for protocol")]
    WebSocketMessageType,

    #[error("RPC client is missing notification handler")]
    MissingNotificationHandler,

    /// Underlying WebSocket error
    #[error("WebSocket -> {0}")]
    WebSocketError(#[from] WebSocketError),
    /// RPC call timeout
    #[error("RPC request timeout")]
    Timeout,
    /// Unable to send shutdown message to receiver
    #[error("Receiver ctl failure")]
    ReceiverCtl,
    /// RPC call succeeded but no data was received in success response
    #[error("RPC has no data in success response")]
    NoDataInSuccessResponse,
    #[error("RPC has no data in notification")]
    NoDataInNotificationMessage,
    /// RPC call failed but no data was received in error response
    #[error("RPC has no data in the error response")]
    NoDataInErrorResponse,
    /// Unable to deserialize response data
    #[error("RPC error deserializing server message data")]
    ErrorDeserializingServerMessageData(crate::error::Error),
    /// Unable to deserialize response data
    #[error("RPC error deserializing response data")]
    ErrorDeserializingResponseData,
    /// Response produced an unknown status code
    #[error("RPC status code {0}")]
    StatusCode(u32),
    /// RPC call executed successfully but produced an error response
    #[error("RPC response error {0:?}")]
    RpcCall(ServerError),
    /// Unable to serialize borsh data    
    #[error("RPC borsh serialization error")]
    BorshSerialize,
    /// Unable to deserialize borsh data
    #[error("RPC borsh deserialization error: {0}")]
    BorshDeserialize(String),
    /// Unable to serialize serde data    
    #[error("RPC serde serialization error: {0}")]
    SerdeSerialize(String), //#[from] dyn serde::de::Error),
    /// Unable to deserialize serde data
    #[error("RPC serde deserialization error: {0}")]
    SerdeDeserialize(String),
    /// RPC call succeeded, but error occurred deserializing borsh response
    #[error("RPC borsh error deserializing response: {0}")]
    BorshResponseDeserialize(String),

    #[error("RPC: channel receive error")]
    ChannelRecvError,

    #[error("RPC: channel send error")]
    ChannelSendError,

    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("SerdeJSON error: {0}")]
    SerdeJSON(#[from] serde_json::Error),

    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),

    #[error("{0}")]
    ServerError(ServerError),

    #[error("{0}")]
    JsonServerError(JsonServerError),
    // #[error("{0}")]
    // RegexError(#[from] regex::Error),
}

impl From<ServerError> for Error {
    fn from(err: ServerError) -> Self {
        Error::ServerError(err)
    }
}

/// Transform Error into JsValue containing the error message
impl From<Error> for JsValue {
    fn from(err: Error) -> JsValue {
        JsValue::from(err.to_string())
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Error::ChannelRecvError
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Error::ChannelSendError
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(_: TrySendError<T>) -> Self {
        Error::ChannelSendError
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::SerdeDeserialize(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::SerdeSerialize(msg.to_string())
    }
}

impl From<JsonServerError> for Error {
    fn from(err: JsonServerError) -> Self {
        Error::JsonServerError(err)
    }
}
