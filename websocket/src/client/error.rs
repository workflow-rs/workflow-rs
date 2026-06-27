use std::sync::PoisonError;
use thiserror::Error;
// use tokio::sync::broadcast::error;
use wasm_bindgen::JsValue;
use workflow_core::channel::*;
use workflow_core::sendable::*;
use workflow_wasm::printable::*;

/// Errors produced by the WebSocket client.
#[derive(Error, Debug)]
pub enum Error {
    /// A custom, free-form error message.
    #[error("{0}")]
    Custom(String),

    /// An error originating from the JavaScript/WASM boundary.
    #[error("{0}")]
    JsValue(Sendable<Printable>),

    /// A mutex was poisoned due to a panic in another thread.
    #[error("PoisonError")]
    PoisonError,

    /// No URL was supplied via the constructor, `connect()`, or a resolver.
    #[error("Missing WebSocket URL (must be supplied in constructor or the connect() method)")]
    MissingUrl,

    /// The supplied URL does not use the `ws://` or `wss://` scheme.
    #[error("WebSocket URL must start with ws:// or wss:// - supplied argument is:`{0}`")]
    AddressSchema(String),

    /// A received message was of an unsupported or unexpected type.
    #[error("Invalid message type")]
    InvalidMessageType,

    /// The socket was in an invalid ready state (value carried) for the operation.
    #[error("InvalidState")]
    InvalidState(u16),

    /// Message payload could not be encoded.
    #[error("DataEncoding")]
    DataEncoding,

    /// The message data was of an unexpected type.
    #[error("DataType")]
    DataType,

    /// The connection has already been initialized.
    #[error("WebSocket connection already initialized")]
    AlreadyInitialized,

    /// The WebSocket is already connected.
    #[error("WebSocket is already connected")]
    AlreadyConnected,

    /// The WebSocket is not currently connected.
    #[error("WebSocket is not connected")]
    NotConnected,

    /// Connection to the given URL could not be established.
    #[error("Unable to connect to {0}")]
    Connect(String),

    /// The custom handshake negotiation failed.
    #[error("Handshake negotiation failure (internal)")]
    NegotiationFailure,

    /// Failed to receive an acknowledgement on the dispatch channel.
    #[error("Dispatch channel ack error")]
    DispatchChannelAck,

    /// A channel send operation failed.
    #[error("Channel send error")]
    ChannelSend,

    /// A non-blocking dispatch channel `try_send` failed.
    #[error("Dispatch channel try_send error")]
    DispatchChannelTrySend,

    /// Failed to signal the dispatcher task.
    #[error("Dispatcher signal error")]
    DispatcherSignal,

    /// The receive channel failed.
    #[error("Receive channel error")]
    ReceiveChannel,

    /// The connect-notification channel failed.
    #[error("Connect channel error")]
    ConnectChannel,

    /// An error originating from a WASM callback.
    #[error(transparent)]
    Callback(#[from] workflow_wasm::callback::CallbackError),

    /// An error originating from the task subsystem.
    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),

    /// An error from the native `tungstenite` WebSocket implementation.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("WebSocket error: {0}")]
    Tungstenite(Box<tokio_tungstenite::tungstenite::Error>),

    /// Failed to relay a control message to the receiver.
    #[error("Unable to send ctl to receiver")]
    ReceiverCtlSend(SendError<super::message::Message>),

    /// An error originating from the `workflow-wasm` crate.
    #[error(transparent)]
    WorkflowWasm(#[from] workflow_wasm::error::Error),

    /// The connection attempt timed out.
    #[error("Connection timeout")]
    ConnectionTimeout,

    /// The supplied connect-strategy argument was invalid.
    #[error("Invalid connect strategy argument: {0}")]
    InvalidConnectStrategyArg(String),

    /// The configured connect strategy was invalid.
    #[error("Invalid connect strategy")]
    InvalidConnectStrategy,
}

impl Error {
    /// Construct a [`Error::Custom`] error from any displayable value.
    pub fn custom<T: std::fmt::Display>(message: T) -> Self {
        Error::Custom(message.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        Error::Tungstenite(Box::new(error))
    }
}

impl From<String> for Error {
    fn from(error: String) -> Error {
        Error::Custom(error)
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Error {
        Error::Custom(error.to_string())
    }
}

impl From<Error> for JsValue {
    fn from(error: Error) -> JsValue {
        JsValue::from_str(&error.to_string())
        // match error {
        //     Error::JsValue(sendable) => sendable.into(),
        //     _ => JsValue::from_str(&error.to_string()),
        // }
    }
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
