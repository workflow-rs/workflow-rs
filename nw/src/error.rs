use std::sync::PoisonError;
use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_core::channel::{RecvError, TrySendError};
use workflow_core::id::Id;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Callback Error: {0}")]
    CallbackError(#[from] workflow_wasm::callback::CallbackError),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("NW error: {0}")]
    NW(#[from] nw_sys::error::Error),

    #[error("Error: {0}")]
    String(String),

    #[error("Error: {0}")]
    JsValue(String),

    #[error("Poison Error: {0}")]
    PoisonError(String),

    #[error("Error: `window.global` object not found")]
    GlobalObjectNotFound,

    #[error("IPC Error: target window `{0}` not found")]
    IpcTargetNotFound(Id),

    #[error("Serde WASM bindgen ser/deser error: {0}")]
    SerdeWasmBindgen(#[from] serde_wasm_bindgen::Error),

    #[error("Unknown broadcast message kind")]
    UnknownBroadcastMessageKind,

    #[error("Error parsing id: {0}")]
    Id(#[from] workflow_core::id::Error),

    #[error("Malformed Ctl message")]
    MalformedCtl,

    #[error("IPC channel send error")]
    ChannelSendError,

    #[error("IPC channel receive error")]
    ChannelRecvError,

    #[error("Broadcast data is not an object")]
    BroadcastDataNotObject,

    #[error(transparent)]
    Wasm(#[from] workflow_wasm::error::Error),
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
