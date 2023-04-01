//!
//! Utilities for calling JavaScript functions and retrieving values
//! from JavaScript object properties.
//!

use crate::error::Error;
use crate::jsvalue::*;
use js_sys::{Array, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;

/// Call a JavaScript function without arguments
pub fn apply_with_args0(this_jsv: &JsValue, fn_name: &str) -> Result<JsValue, JsValue> {
    let fn_jsv = Reflect::get(this_jsv, &JsValue::from(fn_name))?;
    let args = Array::new();
    let ret_jsv = Reflect::apply(&fn_jsv.into(), this_jsv, &args)?;
    Ok(ret_jsv)
}

/// Call a JavaScript function with a single argument
pub fn apply_with_args1(
    this_jsv: &JsValue,
    fn_name: &str,
    arg_jsv: JsValue,
) -> Result<JsValue, JsValue> {
    let fn_jsv = Reflect::get(this_jsv, &JsValue::from(fn_name))?;
    let args = Array::new_with_length(1);
    args.set(0, arg_jsv);
    let ret_jsv = Reflect::apply(&fn_jsv.into(), this_jsv, &args)?;
    Ok(ret_jsv)
}

/// Call a JavaScript function with two arguments
pub fn apply_with_args2(
    this_jsv: &JsValue,
    fn_name: &str,
    arg_jsv: JsValue,
    arg2_jsv: JsValue,
) -> Result<JsValue, JsValue> {
    let fn_jsv = Reflect::get(this_jsv, &JsValue::from(fn_name))?;
    let args = Array::new_with_length(2);
    args.set(0, arg_jsv);
    args.set(1, arg2_jsv);
    let ret_jsv = Reflect::apply(&fn_jsv.into(), this_jsv, &args)?;
    Ok(ret_jsv)
}

/// Obtain a `u64` value from an object property.
/// Results in an `Error` if the value is not a number, rounded `u64` if the value is a number.
pub fn try_get_u64_from_prop(jsv: &JsValue, prop: &str) -> Result<u64, Error> {
    let v = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;

    if v.is_bigint() {
        Ok(v.clone().try_into().map_err(|err| {
            Error::Convert(format!(
                "unable to convert property `{prop}` (BigInt) value: `{v:?}`: {err:?}"
            ))
        })?)
    } else {
        Ok(v.as_f64()
            .ok_or_else(|| Error::WrongType(format!("property `{prop}` is not a number ({v:?})")))?
            as u64)
    }
}

/// Obtain `f64` value from an object property.
/// Results in an `Error` if the value is not a number.
pub fn try_get_f64_from_prop(jsv: &JsValue, prop: &str) -> Result<f64, Error> {
    let v = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;
    let f = v
        .as_f64()
        .ok_or_else(|| Error::WrongType(format!("property `{prop}` is not a number ({v:?})")))?;
    Ok(f)
}

/// Obtain `u8` value from the object property `prop`.
/// Results in an `Error` if the value is not a number or the number value is out of bounds (0..u8::MAX).
pub fn try_get_u8_from_prop(jsv: &JsValue, prop: &str) -> Result<u8, Error> {
    let v = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;
    v.try_as_u8().map_err(|err| {
        Error::WrongType(format!("unable to convert property `{prop}` to u8: {err}"))
    })
}

/// Obtain `u16` value from the object property `prop`.
/// Results in an `Error` if the value is not a number or the number value is out of bounds (0..u16::MAX).
pub fn try_get_u16_from_prop(jsv: &JsValue, prop: &str) -> Result<u16, Error> {
    let v = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;

    v.try_as_u16()
        .map_err(|err| Error::Convert(format!("unable to convert property `{prop}` to u16: {err}")))
}

/// Obtain `u32` value from the object property `prop`.
pub fn try_get_u32_from_prop(jsv: &JsValue, prop: &str) -> Result<u32, Error> {
    let v = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;

    v.try_as_u32()
        .map_err(|err| Error::Convert(format!("unable to convert property `{prop}` to u32: {err}")))
}

/// Obtain a `bool` value from the object property `prop`
pub fn try_get_bool_from_prop(jsv: &JsValue, prop: &str) -> Result<bool, Error> {
    let v = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;

    v.as_bool()
        .ok_or_else(|| Error::WrongType(format!("property {prop} is not a boolean",)))
}

/// Obtain a `Vec<u8>` value from the object property `prop` (using `Uint8Array`)
pub fn try_get_vec_u8_from_number_array_prop(jsv: &JsValue, prop: &str) -> Result<Vec<u8>, Error> {
    let v = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;

    if v.is_array() {
        let array = Array::from(jsv);
        let array: Result<Vec<u8>, Error> = array.to_vec().iter().map(|v| v.try_as_u8()).collect();
        Ok(array?)
    } else {
        Err(Error::WrongType(format!(
            "try_get_vec_u8_from_number_array_prop: property {prop} is not an array"
        )))
    }
}

/// Obtain a `Vec<u8>` value from the object property `prop` (using `Uint8Array`)
pub fn try_get_vec_u8_from_uint8_array_prop(jsv: &JsValue, prop: &str) -> Result<Vec<u8>, Error> {
    let buffer = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;

    let array = Uint8Array::new(&buffer);
    let data: Vec<u8> = array.to_vec();
    Ok(data)
}

/// Obtain a `Vec<u8>` from the property `prop` expressed as a big number
pub fn try_get_vec_u8_from_bn_prop(jsv: &JsValue, prop: &str) -> Result<Vec<u8>, Error> {
    let bn_jsv = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;

    // let bn_jsv = Reflect::get(object_jsv, &JsValue::from(prop))?;
    let bytes = apply_with_args0(&bn_jsv, "toBytes")?;
    let array = Uint8Array::new(&bytes);
    Ok(array.to_vec())
}

/// Obtain `Vec<u8>` from the supplied big number
pub fn try_get_vec_u8_from_bn(bn_jsv: &JsValue) -> Result<Vec<u8>, Error> {
    let bytes = apply_with_args0(bn_jsv, "toBytes")?;
    let array = Uint8Array::new(&bytes);
    Ok(array.to_vec())
}

/// Obtain a `String` value from the object property `prop`
pub fn try_get_string_from_prop(jsv: &JsValue, prop: &str) -> Result<String, Error> {
    let v = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;
    match v.as_string() {
        Some(str) => Ok(str),
        None => Err(Error::WrongType(format!(
            "property '{prop}' is not a string",
        ))),
    }
}

/// Obtain a `JsValue` value from the object property `prop`
pub fn try_get_js_value(jsv: &JsValue, prop: &str) -> Result<JsValue, Error> {
    let v = Reflect::get(jsv, &JsValue::from(prop))
        .map_err(|_| Error::MissingProperty(prop.to_string()))?;
    // let v = Reflect::get(this_jsv, &JsValue::from(prop))?;
    Ok(v)
}
