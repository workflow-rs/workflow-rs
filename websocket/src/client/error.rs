// use super::message::DispatchMessage;
use std::sync::PoisonError;
use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_core::channel::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JsValue {0:?}")]
    JsValue(String),
    #[error("PoisonError")]
    PoisonError,

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

    #[error("Handshake negotiation failure (internal)")]
    NegotiationFailure,

    #[error("Dispatch channel ack error")]
    DispatchChannelAck,
    #[error("Dispatch channel send error")]
    DispatchChannelSend,
    #[error("Dispatch channel try_send error")]
    DispatchChannelTrySend, //(TrySendError<DispatchMessage>)

    #[error("Dispatcher signal error")]
    DispatcherSignal,

    #[error("Receive channel error")]
    ReceiveChannel,

    #[error("Connect channel error")]
    ConnectChannel,

    // #[error("Channel error")]
    // Channel(ChannelError)
    #[error(transparent)]
    Callback(#[from] workflow_wasm::callback::CallbackError),

    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("WebSocket error: {0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    // Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Unable to send ctl to receiver")]
    ReceiverCtlSend(SendError<super::message::Message>),
    // #[error("Unable to send ctl to receiver: {0}")]
    // ReceiverCtlSend(#[from] SendError<super::message::Message>),
}

// impl From<tokio_tungstenite::tungstenite::Error> for Error {
//     fn from(error: tokio_tungstenite::tungstenite::Error) -> Error {
//         Error::JsValue(error)
//     }
// }

// impl Into<JsValue> for Error {
//     fn into(self) -> JsValue {
//         JsValue::from_str(&format!("{:?}", self))
//     }
// }

impl From<JsValue> for Error {
    fn from(error: JsValue) -> Error {
        Error::JsValue(format!("{error:?}"))
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Error {
        Error::PoisonError
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_error: SendError<T>) -> Error {
        Error::DispatchChannelSend //(error)
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Error {
        Error::ReceiveChannel
    }
}

// impl From<SendError<DispatchMessage>> for Error {
//     fn from(_error: SendError<DispatchMessage>) -> Error {
//         Error::DispatchChannelSend //(error)
//     }
// }

// impl From<TrySendError<DispatchMessage>> for Error {
//     fn from(_error: TrySendError<DispatchMessage>) -> Error {
//         Error::DispatchChannelTrySend //(error)
//     }
// }
