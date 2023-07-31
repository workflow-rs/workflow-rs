use std::num::ParseIntError;
use thiserror::Error;
use wasm_bindgen::JsValue;
use workflow_wasm::printable::Printable;

/// Errors return by the [`workflow_d3`](super) module
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    Dom(#[from] workflow_dom::error::Error),

    #[error("{0}")]
    JsValue(Printable),

    #[error(transparent)]
    CallbackError(#[from] workflow_wasm::callback::CallbackError),

    #[error(transparent)]
    Wasm(#[from] workflow_wasm::error::Error),
}

impl From<Error> for JsValue {
    fn from(err: Error) -> JsValue {
        let s: String = err.to_string();
        JsValue::from_str(&s)
    }
}

impl From<JsValue> for Error {
    fn from(js_value: JsValue) -> Error {
        Error::JsValue(Printable::new(js_value))
    }
}
impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Custom(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self::Custom(err.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::Custom(err.to_string())
    }
}
