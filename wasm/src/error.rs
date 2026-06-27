//! Error enum used by the `workflow_wasm` crate.

use crate::jserror::JsErrorData;
use thiserror::Error;
use wasm_bindgen::prelude::*;

/// Errors produced by the `workflow_wasm` crate.
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// Custom error carrying an arbitrary message.
    #[error("{0}")]
    Custom(String),

    /// A value did not have the expected JavaScript type.
    #[error("type error: {0}")]
    WrongType(String),

    /// A value or buffer did not have the expected size.
    #[error("size error: {0}")]
    WrongSize(String),

    /// A required object property was not present.
    #[error("missing property `{0}`")]
    MissingProperty(String),

    /// An error occurred while accessing an object property.
    #[error("error accessing property `{0}`")]
    PropertyAccess(String),

    /// A numeric value fell outside of its permitted range.
    #[error("{0}")]
    Bounds(String),

    /// A value could not be converted to the requested type.
    #[error("{0}")]
    Convert(String),

    /// A hex string had an odd number of characters and cannot be decoded.
    #[error("hex string must have an even number of characters: `{0}`")]
    HexStringNotEven(String),

    /// Error propagated from the `faster_hex` decoding library.
    #[error(transparent)]
    FasterHex(#[from] faster_hex::Error),

    /// Error originating from a JavaScript [`JsValue`].
    #[error(transparent)]
    JsValue(JsErrorData),

    /// Error relating to the WASM ABI representation.
    #[error("WASM ABI: {0}")]
    Abi(String),

    /// The supplied argument was not a JavaScript object.
    #[error("supplied argument is not an object")]
    NotAnObject,

    /// The supplied object did not carry a WASM ABI pointer.
    #[error("supplied object is not a WASM ABI pointer")]
    NotWasmAbiPointer,

    /// The supplied object did not carry a WASM ABI pointer for the named class.
    #[error("supplied object is not a WASM ABI pointer for class `{0}`")]
    NotWasmAbiPointerForClass(String),

    /// The supplied argument was not an object of the named class type.
    #[error("supplied argument is not an object of class type `{0}`")]
    NotAnObjectOfClass(String),

    /// The object's constructor could not be obtained for the expected class.
    #[error("unable to obtain object constructor (for expected class `{0}`)")]
    NoConstructorOfClass(String),

    /// The object's constructor name could not be obtained for the expected class.
    #[error("unable to obtain object constructor name (for expected class `{0}`)")]
    UnableToObtainConstructorName(String),

    /// The object's constructor name did not match the expected class name.
    #[error("object constructor `{0}` does not match expected class `{1}`")]
    ClassConstructorMatch(String, String),
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(error: JsValue) -> Self {
        Error::JsValue(error.into())
    }
}

impl Error {
    /// Construct a [`Error::Custom`] error from any stringifiable message.
    pub fn custom<S: ToString>(msg: S) -> Self {
        Self::Custom(msg.to_string())
    }

    /// Construct a [`Error::Convert`] error from a displayable conversion message.
    pub fn convert<S: std::fmt::Display>(msg: S) -> Self {
        Self::Convert(msg.to_string())
    }
}
