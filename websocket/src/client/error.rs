use std::sync::PoisonError;
use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_core::channel::*;
use workflow_wasm::printable::*;
use workflow_wasm::sendable::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    JsValue(Sendable<Printable>),

    #[error("PoisonError")]
    PoisonError,

    #[error("WebSocket URL must start with ws:// or wss:// - supplied argument is:`{0}`")]
    AddressSchema(String),

    #[error("Invalid message type")]
    InvalidMessageType,

    #[error("InvalidState")]
    InvalidState(u16),

    #[error("DataEncoding")]
    DataEncoding,

    #[error("DataType")]
    DataType,

    #[error("WebSocket connection already initialized")]
    AlreadyInitialized,

    #[error("WebSocket is already connected")]
    AlreadyConnected,

    #[error("WebSocket is not connected")]
    NotConnected,

    #[error("Unable to connect to {0}")]
    Connect(String),

    #[error("Handshake negotiation failure (internal)")]
    NegotiationFailure,

    #[error("Dispatch channel ack error")]
    DispatchChannelAck,

    #[error("Channel send error")]
    ChannelSend,

    #[error("Dispatch channel try_send error")]
    DispatchChannelTrySend,

    #[error("Dispatcher signal error")]
    DispatcherSignal,

    #[error("Receive channel error")]
    ReceiveChannel,

    #[error("Connect channel error")]
    ConnectChannel,

    #[error(transparent)]
    Callback(#[from] workflow_wasm::callback::CallbackError),

    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("WebSocket error: {0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Unable to send ctl to receiver")]
    ReceiverCtlSend(SendError<super::message::Message>),

    #[error(transparent)]
    WorkflowWasm(#[from] workflow_wasm::error::Error),
}

impl From<JsValue> for Error {
    fn from(error: JsValue) -> Error {
        Error::JsValue(Sendable(Printable::new(error)))
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Error {
        Error::PoisonError
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_error: SendError<T>) -> Error {
        Error::ChannelSend
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(_error: TrySendError<T>) -> Error {
        Error::ChannelSend
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Error {
        Error::ReceiveChannel
    }
}
