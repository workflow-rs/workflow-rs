//! `to_value` utility that serializes data to a [`JsValue`] using
//! `serde_wasm_bindgen` with `BigInt` serialization enabled.
pub use serde_wasm_bindgen::*;
use wasm_bindgen::JsValue;
type Result<T> = std::result::Result<T, Error>;

/// Converts a Rust value into a [`JsValue`].
pub fn to_value<T: serde::ser::Serialize + ?Sized>(value: &T) -> Result<JsValue> {
    value.serialize(&Serializer::new().serialize_large_number_types_as_bigints(true))
}
