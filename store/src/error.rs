//!
//!  Errors produced by this crate.
//!

use base64::DecodeError;
use thiserror::Error;
use wasm_bindgen::prelude::*;
use workflow_wasm::jserror::*;

/// Errors produced by the `workflow-store` crate.
#[derive(Error, Debug)]
pub enum Error {
    /// A custom, free-form error message.
    #[error("{0}")]
    Custom(String),

    /// An underlying file system I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// An error originating from the Chrome extension storage backend.
    #[error("Chrome error: {0}")]
    ChromeError(#[from] workflow_chrome::error::Error),

    // #[error("JavaScript error: {0:?}")]
    // JsValue(Sendable<JsValue>),
    /// An error captured from a JavaScript exception or rejected promise.
    #[error("{0}")]
    JsValue(JsErrorData),

    /// A Base64 string could not be decoded.
    #[error("Base64 decode error: {0}")]
    DecodeError(DecodeError),

    /// The requested file or storage key does not exist.
    #[error("Not found: {0}")]
    NotFound(String),

    /// The stored data was expected to be a UTF-8 string but is not.
    #[error("Not a text data: {0}")]
    DataIsNotAString(String),

    /// The stored data was expected to be a binary buffer but is not.
    #[error("Not a buffer data: {0}")]
    DataIsNotABuffer(String),

    /// A JSON serialization or deserialization error.
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    /// The supplied path is malformed or otherwise invalid.
    #[error("invalid path: {0}")]
    InvalidPath(String),

    /// The user's home directory could not be determined.
    #[error("Unable to obtain user home directory")]
    HomeDir(String),

    /// File metadata is unavailable.
    #[error("No file metadata")]
    Metadata,

    /// A hex encoding or decoding error.
    #[error(transparent)]
    FasterHex(#[from] faster_hex::Error),

    /// The requested operation is not supported in the current environment.
    #[error("This operation is not supported")]
    NotSupported,
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
    /// Returns the error code carried by an underlying JavaScript error,
    /// or `None` for error variants that do not have one.
    pub fn code(&self) -> Option<&str> {
        match self {
            Error::JsValue(js_err) => js_err.code().as_deref(),
            _ => None,
        }
    }
}
