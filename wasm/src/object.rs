//!
//! Js [`Object`] property access utilities
//!

use crate::utils::*;
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

/// Custom trait implementing simplified property accessor functions for [`Object`].
pub trait ObjectTrait {
    /// get `JsValue` property
    fn get(&self, prop: &str) -> Result<JsValue, JsValue>;
    /// get `String` property
    fn get_string(&self, prop: &str) -> Result<String, JsValue>;
    /// get `Number` property as `u64`
    fn get_u64(&self, prop: &str) -> Result<u64, JsValue>;
    /// get `Number` property as `f64`
    fn get_f64(&self, prop: &str) -> Result<f64, JsValue>;
    /// get `Boolean` property as `bool`
    fn get_bool(&self, prop: &str) -> Result<bool, JsValue>;
    /// get `Uint8Array` property as `Vec<u8>`
    fn get_vec_u8(&self, prop: &str) -> Result<Vec<u8>, JsValue>;
    /// set `JsValue` property
    fn set(&self, prop: &str, value: &JsValue) -> Result<bool, JsValue>;
    /// delete property
    fn delete(&self, prop: &str) -> Result<bool, JsValue>;
    /// set multiple `JsValue` properties
    fn set_properties(&self, props: &[(&str, &JsValue)]) -> Result<(), JsValue>;
}

impl ObjectTrait for Object {
    fn get(&self, prop: &str) -> Result<JsValue, JsValue> {
        Reflect::get(self, &JsValue::from(prop))
    }

    fn get_string(&self, prop: &str) -> Result<String, JsValue> {
        try_get_string_from_prop(self, prop)
    }

    fn get_u64(&self, prop: &str) -> Result<u64, JsValue> {
        try_get_u64_from_prop(self, prop)
    }

    fn get_bool(&self, prop: &str) -> Result<bool, JsValue> {
        try_get_bool_from_prop(self, prop)
    }

    fn get_vec_u8(&self, prop: &str) -> Result<Vec<u8>, JsValue> {
        try_get_vec_u8_from_prop(self, prop)
    }

    fn get_f64(&self, prop: &str) -> Result<f64, JsValue> {
        try_get_f64_from_prop(self, prop)
    }

    fn set(&self, prop: &str, value: &JsValue) -> Result<bool, JsValue> {
        Reflect::set(self, &JsValue::from(prop), value)
    }

    fn delete(&self, prop: &str) -> Result<bool, JsValue> {
        Reflect::delete_property(self, &JsValue::from(prop))
    }

    fn set_properties(&self, props: &[(&str, &JsValue)]) -> Result<(), JsValue> {
        for (k, v) in props.iter() {
            Reflect::set(self, &JsValue::from(*k), v)?;
        }
        Ok(())
    }
}
