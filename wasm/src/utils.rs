//!
//! Utilities for calling JavaScript functions and retrieving values
//! from JavaScript object properties.
//!

use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::*;

/// Call a JavaScript function without arguments
pub fn apply_with_args0(this_jsv: &JsValue, fn_name: &str) -> Result<JsValue, JsValue> {
    let fn_jsv = js_sys::Reflect::get(this_jsv, &JsValue::from(fn_name))?;
    let args = Array::new();
    let ret_jsv = js_sys::Reflect::apply(&fn_jsv.into(), this_jsv, &args)?;
    Ok(ret_jsv)
}

/// Call a JavaScript function with a single argument
pub fn apply_with_args1(
    this_jsv: &JsValue,
    fn_name: &str,
    arg_jsv: JsValue,
) -> Result<JsValue, JsValue> {
    let fn_jsv = js_sys::Reflect::get(this_jsv, &JsValue::from(fn_name))?;
    let args = Array::new_with_length(1);
    args.set(0, arg_jsv);
    let ret_jsv = js_sys::Reflect::apply(&fn_jsv.into(), this_jsv, &args)?;
    Ok(ret_jsv)
}

/// Call a JavaScript function with two arguments
pub fn apply_with_args2(
    this_jsv: &JsValue,
    fn_name: &str,
    arg_jsv: JsValue,
    arg2_jsv: JsValue,
) -> Result<JsValue, JsValue> {
    let fn_jsv = js_sys::Reflect::get(this_jsv, &JsValue::from(fn_name))?;
    let args = Array::new_with_length(2);
    args.set(0, arg_jsv);
    args.set(1, arg2_jsv);
    let ret_jsv = js_sys::Reflect::apply(&fn_jsv.into(), this_jsv, &args)?;
    Ok(ret_jsv)
}

/// Obtain a `u64` value from an object property.
/// Returns successfully parsed value or 0.
pub fn try_get_u64_from_prop(jsv: &JsValue, prop: &str) -> Result<u64, JsValue> {
    let v = js_sys::Reflect::get(jsv, &JsValue::from(prop))?;
    Ok(v.as_f64().ok_or_else(|| {
        JsValue::from(format!(
            "try_get_u64(): error parsing property '{prop}' with value '{v:?}'"
        ))
    })? as u64)
}

/// Obtain `f64` value from an object property.
pub fn try_get_f64_from_prop(jsv: &JsValue, prop: &str) -> Result<f64, JsValue> {
    let v = js_sys::Reflect::get(jsv, &JsValue::from(prop))?;
    v.as_f64().ok_or_else(|| {
        JsValue::from(format!(
            "try_get_f64(): error parsing property '{prop}' with value '{v:?}'",
        ))
    })
}

/// Obtain `u8` value from the object property `prop`.
pub fn try_get_u8_from_prop(jsv: &JsValue, prop: &str) -> Result<u8, JsValue> {
    let v = js_sys::Reflect::get(jsv, &JsValue::from(prop))?;
    Ok(v.as_f64().ok_or_else(|| {
        JsValue::from(format!(
            "try_get_u8(): error parsing property '{prop}' with value '{v:?}'",
        ))
    })? as u8)
}

/// Obtain a `bool` value from the object property `prop`
pub fn try_get_bool_from_prop(jsv: &JsValue, prop: &str) -> Result<bool, JsValue> {
    js_sys::Reflect::get(jsv, &JsValue::from(prop))?
        .as_bool()
        .ok_or_else(|| {
            JsValue::from(format!(
                "try_get_bool(): property {prop} is missing or not a boolean",
            ))
        })
}

/// Obtain a `Vec<u8>` value from the object property `prop` (using `Uint8Array`)
pub fn try_get_vec_from_prop(jsv: &JsValue, prop: &str) -> Result<Vec<u8>, JsValue> {
    let buffer = js_sys::Reflect::get(jsv, &JsValue::from(prop))?;
    let array = Uint8Array::new(&buffer);
    let data: Vec<u8> = array.to_vec();
    Ok(data)
}

/// Obtain a `Vec<u8>` from the property `prop` expressed as a big number
pub fn try_get_vec_from_bn_prop(object_jsv: &JsValue, prop: &str) -> Result<Vec<u8>, JsValue> {
    let bn_jsv = js_sys::Reflect::get(object_jsv, &JsValue::from(prop))?;
    let bytes = apply_with_args0(&bn_jsv, "toBytes")?;
    let array = Uint8Array::new(&bytes);
    Ok(array.to_vec())
}

/// Obtain `Vec<u8>` from the supplied big number
pub fn try_get_vec_from_bn(bn_jsv: &JsValue) -> Result<Vec<u8>, JsValue> {
    let bytes = apply_with_args0(bn_jsv, "toBytes")?;
    let array = Uint8Array::new(&bytes);
    Ok(array.to_vec())
}

/// Obtain a `String` value from the object property `prop`
pub fn try_get_string(jsv: &JsValue, prop: &str) -> Result<String, JsValue> {
    let str = js_sys::Reflect::get(jsv, &JsValue::from(prop))?;
    match str.as_string() {
        Some(str) => Ok(str),
        None => Err(JsValue::from(format!(
            "Unable to find property '{prop}' on object '{jsv:?}'",
        ))),
    }
}

/// Obtain a `JsValue` value from the object property `prop`
pub fn try_get_js_value(this_jsv: &JsValue, prop: &str) -> Result<JsValue, JsValue> {
    let v = js_sys::Reflect::get(this_jsv, &JsValue::from(prop))?;
    Ok(v)
}
