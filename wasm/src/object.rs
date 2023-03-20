//!
//! Js [`Object`] property access utilities
//!

use crate::utils::*;
use crate::{error::Error, jsvalue::JsValueTrait};
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

/// Custom trait implementing simplified property accessor functions for [`Object`].
pub trait ObjectTrait {
    /// get `JsValue` property
    fn get(&self, prop: &str) -> Result<JsValue, Error>;
    /// get `String` property
    fn get_string(&self, prop: &str) -> Result<String, Error>;
    /// get `Number` property as `u8`
    fn get_u8(&self, prop: &str) -> Result<u8, Error>;
    /// get `Number` property as `u16`
    fn get_u16(&self, prop: &str) -> Result<u16, Error>;
    /// get `Number` property as `u32`
    fn get_u32(&self, prop: &str) -> Result<u32, Error>;
    /// get `Number` property as `u64`
    fn get_u64(&self, prop: &str) -> Result<u64, Error>;
    /// get `Number` property as `f64`
    fn get_f64(&self, prop: &str) -> Result<f64, Error>;
    /// get `Boolean` property as `bool`
    fn get_bool(&self, prop: &str) -> Result<bool, Error>;
    /// get property as `Vec<JsValue>`
    fn get_vec(&self, prop: &str) -> Result<Vec<JsValue>, Error>;
    /// get `Vec<u8>` property from a hex string or an `Array`
    fn get_vec_u8(&self, prop: &str) -> Result<Vec<u8>, Error>;
    /// get `Uint8Array` property as `Vec<u8>`
    fn get_vec_u8_from_number_array(&self, prop: &str) -> Result<Vec<u8>, Error>;
    /// get `Uint8Array` property as `Vec<u8>`
    fn get_vec_u8_from_uint8_array(&self, prop: &str) -> Result<Vec<u8>, Error>;
    /// set `JsValue` property
    fn set(&self, prop: &str, value: &JsValue) -> Result<bool, Error>;
    /// set `Array` property from `&[JsValue]`
    fn set_vec(&self, prop: &str, values: &[JsValue]) -> Result<bool, Error>;
    /// set multiple `JsValue` properties
    fn set_properties(&self, props: &[(&str, &JsValue)]) -> Result<(), Error>;
    /// delete property
    fn delete(&self, prop: &str) -> Result<bool, Error>;
}

impl ObjectTrait for Object {
    fn get(&self, prop: &str) -> Result<JsValue, Error> {
        Ok(Reflect::get(self, &JsValue::from(prop))?)
    }

    fn get_string(&self, prop: &str) -> Result<String, Error> {
        try_get_string_from_prop(self, prop)
    }

    fn get_u8(&self, prop: &str) -> Result<u8, Error> {
        try_get_u8_from_prop(self, prop)
    }

    fn get_u16(&self, prop: &str) -> Result<u16, Error> {
        try_get_u16_from_prop(self, prop)
    }

    fn get_u32(&self, prop: &str) -> Result<u32, Error> {
        try_get_u32_from_prop(self, prop)
    }

    fn get_u64(&self, prop: &str) -> Result<u64, Error> {
        try_get_u64_from_prop(self, prop)
    }

    fn get_bool(&self, prop: &str) -> Result<bool, Error> {
        try_get_bool_from_prop(self, prop)
    }

    fn get_vec(&self, prop: &str) -> Result<Vec<JsValue>, Error> {
        let v = Reflect::get(self, &JsValue::from(prop))?;
        let array = v.dyn_into::<Array>()?;
        Ok(array.to_vec())
    }

    fn get_vec_u8(&self, prop: &str) -> Result<Vec<u8>, Error> {
        let v = Reflect::get(self, &JsValue::from(prop))?;
        v.try_as_vec_u8()
    }

    fn get_vec_u8_from_number_array(&self, prop: &str) -> Result<Vec<u8>, Error> {
        try_get_vec_u8_from_number_array_prop(self, prop)
    }

    fn get_vec_u8_from_uint8_array(&self, prop: &str) -> Result<Vec<u8>, Error> {
        try_get_vec_u8_from_uint8_array_prop(self, prop)
    }

    fn get_f64(&self, prop: &str) -> Result<f64, Error> {
        try_get_f64_from_prop(self, prop)
    }

    fn set(&self, prop: &str, value: &JsValue) -> Result<bool, Error> {
        Ok(Reflect::set(self, &JsValue::from(prop), value)?)
    }

    fn set_vec(&self, prop: &str, values: &[JsValue]) -> Result<bool, Error> {
        let array = js_sys::Array::new();
        for v in values {
            array.push(v);
        }
        Ok(Reflect::set(self, &JsValue::from(prop), &array)?)
    }

    fn set_properties(&self, props: &[(&str, &JsValue)]) -> Result<(), Error> {
        for (k, v) in props.iter() {
            Reflect::set(self, &JsValue::from(*k), v)?;
        }
        Ok(())
    }

    fn delete(&self, prop: &str) -> Result<bool, Error> {
        Ok(Reflect::delete_property(self, &JsValue::from(prop))?)
    }
}
