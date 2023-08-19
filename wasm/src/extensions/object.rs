//!
//! Js [`Object`] property access utilities
//!

use crate::error::Error;
use crate::extensions::jsvalue::JsValueExtension;
use crate::utils::*;
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

/// Custom trait implementing simplified property accessor functions for [`Object`].
pub trait ObjectExtension {
    /// Get a type that implements [`TryFrom<JsValue>`] from a property of the [`Object`].
    fn get<T>(&self, prop: &str) -> Result<T, Error>
    where
        T: TryFrom<JsValue>,
        <T as TryFrom<wasm_bindgen::JsValue>>::Error: std::fmt::Display;

    /// Try to get a type that implements [`TryFrom<JsValue>`] from a property of the [`Object`].
    /// Returns `Ok(None)` if the property does not exist.
    fn try_get<T>(&self, prop: &str) -> Result<Option<T>, Error>
    where
        T: TryFrom<JsValue>,
        <T as TryFrom<wasm_bindgen::JsValue>>::Error: std::fmt::Display;

    /// Get `JsValue` property
    fn get_value(&self, prop: &str) -> Result<JsValue, Error>;
    /// Try Get `JsValue` property
    fn try_get_value(&self, prop: &str) -> Result<Option<JsValue>, Error>;
    /// get `String` property
    fn get_string(&self, prop: &str) -> Result<String, Error>;
    /// get `String` property
    fn try_get_string(&self, prop: &str) -> Result<Option<String>, Error>;
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
    fn try_get_bool(&self, prop: &str) -> Result<Option<bool>, Error>;
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

impl ObjectExtension for Object {
    fn get<T>(&self, prop: &str) -> Result<T, Error>
    where
        T: TryFrom<JsValue>,
        <T as TryFrom<wasm_bindgen::JsValue>>::Error: std::fmt::Display,
    {
        let js_value = Reflect::get(self, &JsValue::from(prop))?;
        T::try_from(js_value).map_err(Error::custom)
    }

    fn try_get<T>(&self, prop: &str) -> Result<Option<T>, Error>
    where
        T: TryFrom<JsValue>,
        <T as TryFrom<wasm_bindgen::JsValue>>::Error: std::fmt::Display,
    {
        let js_value = Reflect::get(self, &JsValue::from(prop))?;
        if js_value.is_falsy() {
            Ok(None)
        } else {
            Ok(Some(T::try_from(js_value).map_err(Error::custom)?))
        }
    }

    fn get_value(&self, prop: &str) -> Result<JsValue, Error> {
        Ok(Reflect::get(self, &JsValue::from(prop))?)
    }

    fn try_get_value(&self, prop: &str) -> Result<Option<JsValue>, Error> {
        let js_value = Reflect::get(self, &JsValue::from(prop))?;
        if js_value == JsValue::UNDEFINED {
            Ok(None)
        } else {
            Ok(Some(js_value))
        }
    }

    fn get_string(&self, prop: &str) -> Result<String, Error> {
        try_get_string_from_prop(self, prop)
    }

    fn try_get_string(&self, prop: &str) -> Result<Option<String>, Error> {
        Ok(self.get_value(prop)?.as_string())
    }

    fn get_bool(&self, prop: &str) -> Result<bool, Error> {
        try_get_bool_from_prop(self, prop)
    }

    fn try_get_bool(&self, prop: &str) -> Result<Option<bool>, Error> {
        Ok(self.get_value(prop)?.as_bool())
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

    fn get_vec(&self, prop: &str) -> Result<Vec<JsValue>, Error> {
        try_get_vec_from_prop(self, prop)
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
