use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_core::channel::{RecvError, SendError, TrySendError};
use workflow_core::sendable::Sendable;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Channel send() error")]
    SendError,

    #[error("Channel try_send() error")]
    TrySendError,

    #[error("Channel recv() error")]
    RecvError,
}

impl Error {
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
