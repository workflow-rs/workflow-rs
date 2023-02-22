//!
//! Send trait implementation for JsValue
//!

use std::ops::Deref;

pub mod wasm {
    pub use js_sys::Function;
    pub use wasm_bindgen::JsValue;
}

/// NewType wrapper for JsValue implementing `Send` trait
pub struct JsValue(pub wasm::JsValue);
unsafe impl Send for JsValue {}

impl Deref for JsValue {
    type Target = wasm::JsValue;
    fn deref(&self) -> &wasm::JsValue {
        &self.0
    }
}
impl AsRef<wasm::JsValue> for JsValue {
    fn as_ref(&self) -> &wasm::JsValue {
        &self.0
    }
}
impl From<JsValue> for wasm::JsValue {
    fn from(value: JsValue) -> Self {
        value.0
    }
}

/// NewType wrapper for Function implementing `Send` trait
pub struct Function(pub wasm::Function);
unsafe impl Send for Function {}

impl Deref for Function {
    type Target = wasm::Function;
    fn deref(&self) -> &wasm::Function {
        &self.0
    }
}
impl AsRef<wasm::Function> for Function {
    fn as_ref(&self) -> &wasm::Function {
        &self.0
    }
}
impl From<Function> for wasm::Function {
    fn from(value: Function) -> Self {
        value.0
    }
}
