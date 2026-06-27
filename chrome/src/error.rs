use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_core::channel::{RecvError, SendError, TrySendError};
use workflow_core::sendable::Sendable;

/// Errors produced by `workflow-chrome` operations.
#[derive(Debug, Error)]
pub enum Error {
    /// A custom error carrying an arbitrary message, also used to wrap
    /// JavaScript (`JsValue`) errors.
    #[error("{0}")]
    Custom(String),

    /// An underlying I/O error.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// A channel `send()` operation failed.
    #[error("Channel send() error")]
    SendError,

    /// A channel `try_send()` operation failed.
    #[error("Channel try_send() error")]
    TrySendError,

    /// A channel `recv()` operation failed.
    #[error("Channel recv() error")]
    RecvError,

    /// The requested key was not found in storage.
    #[error("No such key in storage {0}")]
    MissingKey(String),

    /// A key that already exists in storage was used where it must be absent.
    #[error("Key already exists in storage {0}")]
    KeyExists(String),
}

impl Error {
    /// Creates a [`Error::Custom`] from any value convertible into a `String`.
    pub fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Self::Custom(err.as_string().unwrap())
    }
}

impl From<Sendable<JsValue>> for Error {
    fn from(err: Sendable<JsValue>) -> Self {
        Self::Custom(err.0.as_string().unwrap())
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Custom(err)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Error::SendError
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Error::RecvError
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(_: TrySendError<T>) -> Self {
        Error::TrySendError
    }
}
