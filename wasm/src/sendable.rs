//!
//! Send trait implementation for JsValue
//!

use std::ops::Deref;
use wasm_bindgen::prelude::*;

/// NewType wrapper for JsValue implementing `Send` trait
pub struct JsValueSend(pub JsValue);
unsafe impl Send for JsValueSend {}

impl Deref for JsValueSend {
    type Target = JsValue;
    fn deref(&self) -> &JsValue {
        &self.0
    }
}
impl AsRef<JsValue> for JsValueSend {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}
impl From<JsValueSend> for JsValue {
    fn from(value: JsValueSend) -> Self {
        value.0
    }
}
