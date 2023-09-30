//!
//!  Errors produced by this crate.
//!

use base64::DecodeError;
use thiserror::Error;
use wasm_bindgen::prelude::*;
// use workflow_core::sendable::Sendable;
use chrome_sys::error::Error as ChromeError;
use workflow_wasm::jserror::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("ChromeSys error: {0}")]
    ChromeError(#[from] ChromeError),

    // #[error("JavaScript error: {0:?}")]
    // JsValue(Sendable<JsValue>),
    #[error("{0}")]
    JsValue(JsErrorData),

    #[error("Base64 decode error: {0}")]
    DecodeError(DecodeError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Not a text data: {0}")]
    DataIsNotAString(String),

    #[error("Not a buffer data: {0}")]
    DataIsNotABuffer(String),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error("invalid path: {0}")]
    InvalidPath(String),

    #[error("Unable to obtain user home directory")]
    HomeDir(String),

    #[error("No file metadata")]
    Metadata,

    #[error(transparent)]
    FasterHex(#[from] faster_hex::Error),
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        match err {
            Error::JsValue(js_err) => js_err.into(),
            _ => err.into(),
        }
    }
}

// impl From<Error> for JsErrorData {
//     fn from(err: Error) -> Self {
//         match err {
//             Error::JsValue(js_err) => js_err.into(),
//             _ => err.into(),
//         }
//     }
// }

impl From<JsValue> for Error {
    fn from(error: JsValue) -> Error {
        Error::JsValue(error.into())
    }
}

impl From<JsErrorData> for Error {
    fn from(error: JsErrorData) -> Error {
        Error::JsValue(error)
    }
}

impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Error {
        Error::DecodeError(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Error {
        Error::Custom(error)
    }
}

impl Error {
    pub fn code(&self) -> Option<&str> {
        match self {
            Error::JsValue(js_err) => js_err.code().as_deref(),
            _ => None,
        }
    }
}
