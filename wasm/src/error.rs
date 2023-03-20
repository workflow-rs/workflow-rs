use thiserror::Error;
use wasm_bindgen::prelude::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("type error: {0}")]
    WrongType(String),

    #[error("size error: {0}")]
    WrongSize(String),

    #[error("missing property: `{0}`")]
    MissingProperty(String),

    #[error("{0}")]
    Bounds(String),

    #[error("{0}")]
    Convert(String),

    #[error("hex string must have an even number of characters: `{0}`")]
    HexStringNotEven(String),

    #[error(transparent)]
    FasterHex(#[from] faster_hex::Error),

    #[error("JsValue: {0:?}")]
    JsValue(JsValue),

    #[error("WASM ABI: {0}")]
    Abi(String),

    #[error("supplied argument is not an object")]
    NotAnObject,

    #[error("supplied object is not a WASM ABI pointer")]
    NotWasmAbiPointer,

    #[error("supplied object is not a WASM ABI pointer for class `{0}`")]
    NotWasmAbiPointerForClass(String),

    #[error("supplied argument is not an object of class type `{0}`")]
    NotAnObjectOfClass(String),

    #[error("unable to obtain object constructor (for expected class `{0}`)")]
    NoConstructorOfClass(String),

    #[error("unable to obtain object constructor name (for expected class `{0}`)")]
    UnableToObtainConstructorName(String),

    #[error("object constructor `{0}` does not match expected class `{1}`")]
    ClassConstructorMatch(String, String),
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value)
    }
}
