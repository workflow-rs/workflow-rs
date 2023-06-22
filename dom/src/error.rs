//!
//! Errors return by the [`workflow_dom`](super) module
//!
use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_core::channel::RecvError;
use workflow_wasm::sendable::Sendable;

/// Errors return by the [`workflow_dom`](super) module
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// Custom string error
    #[error("{0}")]
    String(String),
    /// Error containing [`wasm_bindgen::JsValue`] value
    #[error("{0:?}")]
    JsValue(Sendable<JsValue>),
    #[error("{0}")]
    RecvError(RecvError), //#[from] workflow_core::channel::RecvError),
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

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
        Self::JsValue(Sendable(v))
    }
}

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Self {
        Self::RecvError(err)
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}
