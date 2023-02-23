//!
//! Send trait implementation for JsValue
//!

// use workflow_wasm_macros::build_sendable_types;
// use std::ops::Deref;

// pub mod non_sendable {
//     pub use js_sys::*;
//     pub use wasm_bindgen::JsValue;
// }

// build_sendable_types!([
//     // JsValue,
//     Object, Function,
// ]);

pub struct Sendable<T>(pub T);
unsafe impl<T> Send for Sendable<T> {}

impl<T> std::ops::Deref for Sendable<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T> AsRef<T> for Sendable<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

// impl<T: ?Sized> From<Sendable<T>> for T {
//     fn from(value: Sendable<T>) -> Self {
//         value.0
//     }
// }

// NewType wrapper for JsValue implementing `Send` trait
// pub struct JsValue(pub non_sendable::JsValue);
// unsafe impl Send for JsValue {}

// impl std::ops::Deref for JsValue {
//     type Target = non_sendable::JsValue;
//     fn deref(&self) -> &non_sendable::JsValue {
//         &self.0
//     }
// }
// impl AsRef<non_sendable::JsValue> for JsValue {
//     fn as_ref(&self) -> &non_sendable::JsValue {
//         &self.0
//     }
// }
// impl From<JsValue> for non_sendable::JsValue {
//     fn from(value: JsValue) -> Self {
//         value.0
//     }
// }
