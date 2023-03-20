//! Functions to obtain Rust object references from WASM ABI.

use crate::error::Error;
use wasm_bindgen::convert::RefFromWasmAbi;
use wasm_bindgen::prelude::*;
pub use workflow_wasm_macros::{ref_from_abi, ref_from_abi_as_option, TryFromJsValue};

/// Create a reference to a Rust object from a WASM ABI.
/// # Safety
/// Only basic sanity checks are performed. If user passes and invalid
/// object containing `ptr` property (i.e. another Rust-wasm-bindgen-created object)
/// the application will yield a memory access violation.
pub unsafe fn ref_from_abi<T>(js: &JsValue) -> std::result::Result<T, Error>
where
    T: RefFromWasmAbi<Abi = u32> + Clone,
{
    if !js.is_object() {
        return Err(Error::NotAnObject);
    }

    let ptr = ::js_sys::Reflect::get(js, &JsValue::from_str("ptr"))?;
    if ptr.is_falsy() {
        return Err(Error::NotWasmAbiPointer);
    }
    let ptr_u32: u32 = ptr.as_f64().ok_or(Error::NotWasmAbiPointer)? as u32;
    let instance_ref = unsafe { T::ref_from_abi(ptr_u32) };
    Ok(instance_ref.clone())
}

/// Create a reference to a Rust object from a WASM ABI.
/// This function validates the supplied object by comparing its `constructor.name` value to the supplied
/// `class` name. You can use this function in two forms: `ref_from_abi_safe("SomeStruct", jsvalue)` or
/// via a macro `ref_from_abi!(SomeStruct,jsvalue)`.
pub fn ref_from_abi_safe<T>(class: &str, js: &JsValue) -> std::result::Result<T, Error>
where
    T: RefFromWasmAbi<Abi = u32> + Clone,
{
    if !js.is_object() {
        return Err(Error::NotAnObjectOfClass(class.to_string()));
    }

    let ctor = ::js_sys::Reflect::get(js, &JsValue::from_str("constructor"))?;
    if ctor.is_falsy() {
        return Err(Error::NoConstructorOfClass(class.to_string()));
    } else {
        let name = ::js_sys::Reflect::get(&ctor, &JsValue::from_str("name"))?;
        if name.is_falsy() {
            return Err(Error::UnableToObtainConstructorName(class.to_string()));
        } else {
            let name = name
                .as_string()
                .ok_or(Error::UnableToObtainConstructorName(class.to_string()))?;
            if name != class {
                return Err(Error::ClassConstructorMatch(name, class.to_string()));
            }
        }
    }

    let ptr = ::js_sys::Reflect::get(js, &::wasm_bindgen::JsValue::from_str("ptr"))?;
    if ptr.is_falsy() {
        return Err(Error::NotWasmAbiPointerForClass(class.to_string()));
    }
    let ptr_u32: u32 = ptr
        .as_f64()
        .ok_or(Error::NotWasmAbiPointerForClass(class.to_string()))? as u32;
    let instance_ref = unsafe { T::ref_from_abi(ptr_u32) };
    Ok(instance_ref.clone())
}

/// Create a reference to a Rust object from a WASM ABI.
/// Returns None is the supplied value is `null` or `undefined`, otherwise tries to cast the object.
/// Casting validates the supplied object by comparing its `constructor.name` value to the supplied
/// `class` name. You can use this function in two forms: `ref_from_abi_safe_as_option("SomeStruct", jsvalue)` or
/// via a macro `ref_from_abi_as_option!(SomeStruct,jsvalue)`.
pub fn ref_from_abi_safe_as_option<T>(
    class: &str,
    js: &JsValue,
) -> std::result::Result<Option<T>, JsValue>
where
    T: RefFromWasmAbi<Abi = u32> + Clone,
{
    if !js.is_undefined() && !js.is_null() {
        Ok(Some(ref_from_abi_safe::<T>(class, js)?))
    } else {
        Ok(None)
    }
}
