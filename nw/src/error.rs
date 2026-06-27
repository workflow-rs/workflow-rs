use std::sync::PoisonError;
use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_core::channel::{RecvError, TrySendError};
use workflow_core::id::Id;

/// Errors produced by the `workflow-nw` crate.
#[derive(Error, Debug)]
pub enum Error {
    /// A custom, free-form error message.
    #[error("Error: {0}")]
    Custom(String),

    /// Error originating from a WASM callback invocation.
    #[error("Callback Error: {0}")]
    CallbackError(#[from] workflow_wasm::callback::CallbackError),

    /// Underlying I/O error.
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    /// Error returned from the `nw_sys` NW.js bindings.
    #[error("NW error: {0}")]
    NW(#[from] nw_sys::error::Error),

    /// A JavaScript exception captured as a string.
    #[error("Error: {0}")]
    JsValue(String),

    /// A mutex/lock was poisoned.
    #[error("Poison Error: {0}")]
    PoisonError(String),

    /// The JavaScript `window.global` object could not be found.
    #[error("Error: `window.global` object not found")]
    GlobalObjectNotFound,

    /// No IPC target window matching the given id was found.
    #[error("IPC Error: target window `{0}` not found")]
    IpcTargetNotFound(Id),

    /// Serialization/deserialization error via `serde-wasm-bindgen`.
    #[error("Serde WASM bindgen ser/deser error: {0}")]
    SerdeWasmBindgen(#[from] serde_wasm_bindgen::Error),

    /// A received broadcast message had an unrecognized kind.
    #[error("Unknown broadcast message kind")]
    UnknownBroadcastMessageKind,

    /// Error parsing an identifier.
    #[error("Error parsing id: {0}")]
    Id(#[from] workflow_core::id::Error),

    /// A control (`Ctl`) message could not be parsed.
    #[error("Malformed Ctl message")]
    MalformedCtl,

    /// Failed to send a message over an IPC channel.
    #[error("IPC channel send error")]
    ChannelSendError,

    /// Failed to receive a message from an IPC channel.
    #[error("IPC channel receive error")]
    ChannelRecvError,

    /// Broadcast payload was expected to be a JavaScript object but was not.
    #[error("Broadcast data is not an object")]
    BroadcastDataNotObject,

    /// Error originating from the `workflow-wasm` crate.
    #[error(transparent)]
    Wasm(#[from] workflow_wasm::error::Error),

    /// Error originating from the IPC subsystem.
    #[error(transparent)]
    Ipc(#[from] crate::ipc::error::Error),
    // #[error(transparent)]
    // IpcResponse(#[from] crate::ipc::error::ResponseError),
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
