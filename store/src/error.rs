use wasm_bindgen::prelude::*;
use thiserror::Error;
use base64::DecodeError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JavaScript error: {0}")]
    JsValue(String),

    #[error("Base64 decode error: {0}")]
    DecodeError(DecodeError),
}


impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&format!("{:?}", self))
    }
}

impl From<JsValue> for Error {
    fn from(error: JsValue) -> Error {
        Error::JsValue(format!("{:?}",error))
    }
}

impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Error {
        Error::DecodeError(error)
    }
}
