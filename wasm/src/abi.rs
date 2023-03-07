//! Functions to obtain Rust object references from WASM ABI.

use wasm_bindgen::convert::RefFromWasmAbi;
use wasm_bindgen::prelude::*;
pub use workflow_wasm_macros::{ref_from_abi, ref_from_abi_as_option, TryFromJsValue};

/// Create a reference to a Rust object from a WASM ABI.
/// # Safety
/// Only basic sanity checks are performed. If user passes and invalid
/// object containing `ptr` property (i.e. another Rust-wasm-bindgen-created object)
/// the application will yield a memory access violation.
pub unsafe fn ref_from_abi<T>(js: &JsValue) -> std::result::Result<T, JsValue>
where
    T: RefFromWasmAbi<Abi = u32> + Clone,
{
    if !js.is_object() {
        return Err(JsValue::from(
            "ref_from_abi_safe(): supplied argument is not a class object",
        ));
    }

    let ptr = ::js_sys::Reflect::get(js, &JsValue::from_str("ptr"))?;
    if ptr.is_falsy() {
        return Err(JsValue::from(
            "ref_from_abi(): WASM API received an invalid JavaScript object (the object is not a WASM ABI pointer)",
        ));
    }
    let ptr_u32: u32 = ptr.as_f64().ok_or(JsValue::from(
        "reg_from_abi(): WASM API received an invalid JavaScript object (the object is not a WASM ABI pointer)",
    ))? as u32;
    let instance_ref = unsafe { T::ref_from_abi(ptr_u32) };
    Ok(instance_ref.clone())
}

/// Create a reference to a Rust object from a WASM ABI.
/// This function validates the supplied object by comparing its `constructor.name` value to the supplied
/// `class` name. You can use this function in two forms: `ref_from_abi_safe("SomeStruct", jsvalue)` or
/// via a macro `ref_from_abi!(SomeStruct,jsvalue)`.
pub fn ref_from_abi_safe<T>(class: &str, js: &JsValue) -> std::result::Result<T, JsValue>
where
    T: RefFromWasmAbi<Abi = u32> + Clone,
{
    if !js.is_object() {
        return Err(JsValue::from(format!(
            "ref_from_abi_safe(): supplied argument is not of expected class type `{class}`"
        )));
    }

    let ctor = ::js_sys::Reflect::get(js, &JsValue::from_str("constructor"))?;
    if ctor.is_falsy() {
        return Err(JsValue::from(format!("ref_from_abi_safe(): unable to obtain object constructor (for expected class `{class}`)")));
    } else {
        let name = ::js_sys::Reflect::get(&ctor, &JsValue::from_str("name"))?;
        if name.is_falsy() {
            return Err(JsValue::from(
                format!("ref_from_abi_safe(): unable to obtain object constructor name (for expected class `{class}`)")
            ));
        } else {
            let name = name.as_string().ok_or(JsValue::from(format!("ref_from_abi_safe(): unable to obtain object constructor name (for expected class `{class}`)")))?;
            if name != class {
                return Err(JsValue::from(format!(
                    "ref_from_abi_safe(): object constructor `{name}` does not match expected class `{class}`"
                )));
            }
        }
    }

    let ptr = ::js_sys::Reflect::get(js, &::wasm_bindgen::JsValue::from_str("ptr"))?;
    if ptr.is_falsy() {
        return Err(JsValue::from(
            "ref_from_abi(): WASM API received an invalid JavaScript object (the object is not a WASM ABI pointer)",
        ));
    }
    let ptr_u32: u32 = ptr.as_f64().ok_or(::wasm_bindgen::JsValue::from(
        "reg_from_abi(): WASM API received an invalid JavaScript object (the object is not a WASM ABI pointer)",
    ))? as u32;
    let instance_ref = unsafe { T::ref_from_abi(ptr_u32) };
    Ok(instance_ref.clone())
}


/// Create a reference to a Rust object from a WASM ABI.
/// Returns None is the supplied value is `null` or `undefined`, otherwise tries to cast the object.
/// Casting validates the supplied object by comparing its `constructor.name` value to the supplied
/// `class` name. You can use this function in two forms: `ref_from_abi_safe_as_option("SomeStruct", jsvalue)` or
/// via a macro `ref_from_abi_as_option!(SomeStruct,jsvalue)`.
pub fn ref_from_abi_safe_as_option<T>(class: &str, js: &JsValue) -> std::result::Result<Option<T>, JsValue>
where
    T: RefFromWasmAbi<Abi = u32> + Clone,
{
    if !js.is_undefined() && !js.is_null(){
        Ok(Some(ref_from_abi_safe::<T>(class, js)?))
    }else{
        Ok(None)
    }

}