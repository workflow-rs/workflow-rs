use std::sync::PoisonError;
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Callback Error: {0}")]
    CallbackError(#[from] workflow_wasm::callback::CallbackError),

    #[error("NW Error: {0}")]
    NwError(#[from] nw_sys::error::Error),

    #[error("Error: {0}")]
    String(String),

    #[error("Error: {0}")]
    JsValue(String),

    #[error("Poison Error: {0}")]
    PoisonError(String),
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
