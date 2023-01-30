//!
//! Errors produced by [`workflow_terminal`](super)
//!

use std::sync::PoisonError;
use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_core::channel::{ChannelError, RecvError, SendError, TrySendError};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: {0}")]
    String(String),
    #[error("Error: {0}")]
    JsValue(String),
    #[error("Poison Error: {0}")]
    PoisonError(String),
    #[error("Channel Receive Error")]
    RecvError,
    #[error("Channel Send Error: {0}")]
    SendError(String),
    #[error("Channel TrySend Error: {0}")]
    TrySendError(String),
    #[error(transparent)]
    DomError(#[from] workflow_dom::error::Error),
    #[error("Channel error: {0}")]
    ChannelError(String),
}

impl From<String> for Error {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for Error {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(v: JsValue) -> Self {
        Self::JsValue(format!("{v:?}"))
    }
}

impl<T> From<PoisonError<T>> for Error
where
    T: std::fmt::Debug,
{
    fn from(err: PoisonError<T>) -> Error {
        Error::PoisonError(format!("{err:?}"))
    }
}

impl From<RecvError> for Error {
    fn from(_err: RecvError) -> Error {
        Error::RecvError
    }
}

impl<T> From<TrySendError<T>> for Error
where
    T: std::fmt::Debug,
{
    fn from(err: TrySendError<T>) -> Error {
        Error::TrySendError(format!("{err:?}"))
    }
}

impl<T> From<SendError<T>> for Error
where
    T: std::fmt::Debug,
{
    fn from(err: SendError<T>) -> Error {
        Error::SendError(format!("{err:?}"))
    }
}

impl From<Error> for String {
    fn from(err: Error) -> String {
        match err {
            Error::String(s)
            | Error::PoisonError(s)
            | Error::SendError(s)
            | Error::JsValue(s)
            | Error::ChannelError(s) => s,
            Error::TrySendError(s) => s,
            Error::RecvError => err.to_string(),
            Error::DomError(err) => err.to_string(),
        }
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> JsValue {
        let s: String = err.into();
        JsValue::from_str(&s)
    }
}

impl<T> From<ChannelError<T>> for Error {
    fn from(err: ChannelError<T>) -> Error {
        Error::ChannelError(err.to_string())
    }
}
