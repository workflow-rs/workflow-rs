//!
//! WASM bindgen casting and utility functions.
//!
//! This module provides a `CastFromJs` trait and derive macro
//! that allows for easy casting of JavaScript objects into Rust.
//! The secondary goal of this module is to provide the ability
//! to dynamically interpret user-supplied JavaScript data that
//! instead of a Rust object may container other data that can
//! be used (interpreted) to create a Rust object.
//!
//! To accommodate this a [`TryCastFromJs`] trait is provided
//! where user needs to implement `try_cast_from` function that
//! can attempt to cast a JsValue into a Rust object or interpret
//! the source data and create a temporary struct owned by by the
//! [`Cast`] enum.
//!
//!

use crate::error::Error;
use crate::extensions::ObjectExtension;
use js_sys::Object;
pub use std::borrow::Borrow;
pub use std::ops::Deref;
use wasm_bindgen::convert::{LongRefFromWasmAbi, RefFromWasmAbi, RefMutFromWasmAbi};
use wasm_bindgen::prelude::*;
pub use workflow_wasm_macros::CastFromJs;

#[wasm_bindgen(typescript_custom_section)]
const IWASM32_BINDINGS_CONFIG: &'static str = r#"
/**
 * Interface for configuring workflow-rs WASM32 bindings.
 * 
 * @category General
 */
export interface IWASM32BindingsConfig {
    /**
     * This option can be used to disable the validation of class names
     * for instances of classes exported by Rust WASM32 when passing
     * these classes to WASM32 functions.
     * 
     * This can be useful to programmatically disable checks when using
     * a bundler that mangles class symbol names.
     */
    validateClassNames : boolean;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "IWASM32BindingsConfig")]
    pub type IWASM32BindingsConfig;
}

static mut VALIDATE_CLASS_NAMES: bool = true;
/// Configuration for the WASM32 bindings runtime interface.
/// @see {@link IWASM32BindingsConfig}
/// @category General
#[wasm_bindgen(js_name = "initWASM32Bindings")]
pub fn init_wasm32_bindings(config: IWASM32BindingsConfig) -> std::result::Result<(), Error> {
    if let Some(enable) = config.try_get_bool("validateClassNames")? {
        unsafe {
            VALIDATE_CLASS_NAMES = enable;
        }
    }
    Ok(())
}
#[inline(always)]
pub fn validate_class_names() -> bool {
    unsafe { VALIDATE_CLASS_NAMES }
}

/// A wrapper for a Rust object that can be either a reference or a value.
/// This wrapper is used to carry a Rust (WASM ABI) reference provided by
/// `wasm_bindgen`, but at the same time allows creation of a temporary
/// object that can be created by interpreting the source user-supplied data.
/// [`Cast`] then provides [`Cast::as_ref()`] to obtain the internally held
/// reference and [`Cast::into_owned()`] where the latter will consume the
/// value or clone the reference.
pub enum Cast<T>
where
    T: RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32>,
{
    Ref(<T as RefFromWasmAbi>::Anchor),
    LongRef(<T as LongRefFromWasmAbi>::Anchor),
    Value(T),
}

impl<T> Deref for Cast<T>
where
    T: RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32> + Deref,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Cast::Ref(r) => r,
            Cast::LongRef(r) => r.borrow(),
            Cast::Value(v) => v,
        }
    }
}

impl<T> AsRef<T> for Cast<T>
where
    T: RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32>,
{
    /// Obtain a reference to the internally held value.
    fn as_ref(&self) -> &T {
        match self {
            Cast::Ref(r) => r,
            Cast::LongRef(r) => r.borrow(),
            Cast::Value(v) => v,
        }
    }
}

impl<T> Cast<T>
where
    T: RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32> + Clone, // + ToOwned,
{
    /// Consume the [`Cast`] and return the owned value. If the
    /// [`Cast`] holds a reference, it will be cloned.
    pub fn into_owned(self) -> T {
        match self {
            Cast::Ref(r) => (*r).clone(),
            Cast::LongRef(r) => r.borrow().clone(),
            Cast::Value(v) => v,
        }
    }
}

/// Cast T value (struct) into `Cast<T>`
impl<T> From<T> for Cast<T>
where
    T: RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32>,
{
    fn from(value: T) -> Cast<T> {
        Cast::Value(value)
    }
}

/// `CastFromJs` trait is automatically implemented by deriving
/// the `CastFromJs` derive macro. This trait provides functions
/// for accessing Rust references from the WASM ABI.
pub trait CastFromJs
where
    Self: Sized + RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32>,
{
    /// Obtain safe reference from [`JsValue`]
    fn try_ref_from_js_value(
        js: impl AsRef<JsValue>,
    ) -> std::result::Result<<Self as RefFromWasmAbi>::Anchor, Error>;

    fn try_ref_from_js_value_as_cast(
        js: impl AsRef<JsValue>,
    ) -> std::result::Result<Cast<Self>, Error> {
        Self::try_ref_from_js_value(js).map(Cast::Ref)
    }

    /// Obtain safe long reference from [`JsValue`]
    fn try_long_ref_from_js_value(
        js: impl AsRef<JsValue>,
    ) -> std::result::Result<<Self as LongRefFromWasmAbi>::Anchor, Error>;

    fn try_long_ref_from_js_value_as_cast(
        js: impl AsRef<JsValue>,
    ) -> std::result::Result<Cast<Self>, Error> {
        Self::try_long_ref_from_js_value(js).map(Cast::LongRef)
    }
}

/// `TryCastFromJs` trait is meant to be implemented by the developer
/// on any struct implementing `CastFromJs` trait. This trait provides
/// a way to attempt to cast a JsValue into a Rust object or interpret
/// the source data and create a temporary struct owned by by the [`Cast`].
pub trait TryCastFromJs
where
    Self: CastFromJs + RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32> + Clone,
{
    type Error: std::fmt::Display + From<Error>;

    /// Try to cast a JsValue into a Rust object.
    /// This should be user-defined function that
    /// attempts to cast a JsValue into a Rust object
    /// or interpret a source data and create a
    /// temporary struct owned by by the [`Cast`].
    fn try_cast_from(value: impl AsRef<JsValue>) -> std::result::Result<Cast<Self>, Self::Error>;

    /// Perform a user cast and consume the [`Cast`] container.
    /// This function will return a temporary user-created
    /// object created during [`try_cast_from`] or a clone of the casted reference.
    fn try_owned_from(value: impl AsRef<JsValue>) -> std::result::Result<Self, Self::Error> {
        Self::try_cast_from(value).map(|c| c.into_owned())
    }

    /// Try to cast a JsValue into a Rust object, in cast of failure
    /// invoke a user-supplied closure that can try to create an instance
    /// of the object based on the supplied JsValue.
    fn resolve(
        js: impl AsRef<JsValue>,
        create: impl FnOnce() -> std::result::Result<Self, Self::Error>,
    ) -> std::result::Result<Cast<Self>, Self::Error> {
        Self::try_ref_from_js_value(js)
            .map(Cast::<Self>::Ref)
            .or_else(|_| create().map(Cast::<Self>::Value))
    }

    /// Try to cast a JsValue into a Rust object, in cast of failure
    /// invoke a user-supplied closure that can try to create an instance
    /// of the object based on the supplied JsValue. Unlike the [`resolve`]
    /// function, this function expects `create` closure to return a [`Cast`].
    /// This is useful when routing the creation of the object to another
    /// function that is capable of creating a compatible Cast wrapper.
    fn resolve_cast(
        js: impl AsRef<JsValue>,
        create: impl FnOnce() -> std::result::Result<Cast<Self>, Self::Error>,
    ) -> std::result::Result<Cast<Self>, Self::Error> {
        Self::try_ref_from_js_value(js)
            .map(Cast::<Self>::Ref)
            .or_else(|_| create())
    }
}

pub trait TryCastJsInto<T>
where
    T: TryCastFromJs,
{
    type Error: From<Error>;
    fn try_into_cast(&self) -> std::result::Result<Cast<T>, Self::Error>;
    fn try_into_owned(&self) -> std::result::Result<T, Self::Error>;
}

impl<T> TryCastJsInto<T> for JsValue
where
    T: TryCastFromJs,
    <T as TryCastFromJs>::Error: From<Error>,
{
    type Error = <T as TryCastFromJs>::Error;
    fn try_into_cast(&self) -> std::result::Result<Cast<T>, Self::Error> {
        T::try_cast_from(self)
    }

    fn try_into_owned(&self) -> std::result::Result<T, Self::Error> {
        T::try_owned_from(self)
    }
}

/// Obtain a WASM bingen ABI pointer from a supplied JsValue.
/// This function validates the acquired object ptr by comparing its
/// `constructor.name` value to the supplied `class` name.
fn get_ptr_u32_safe(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<Option<u32>, Error> {
    let js = js.as_ref();

    if js.is_undefined() || js.is_null() {
        return Ok(None);
    } else if !js.is_object() {
        return Err(Error::NotAnObjectOfClass(class.to_string()));
    }

    if validate_class_names() {
        let ctor = ::js_sys::Reflect::get(js, &JsValue::from_str("constructor"))?;
        if ctor.is_undefined() {
            return Err(Error::NoConstructorOfClass(class.to_string()));
        } else {
            let name = ::js_sys::Reflect::get(&ctor, &JsValue::from_str("name"))?;
            if name.is_undefined() {
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
    }

    let ptr = ::js_sys::Reflect::get(js, &::wasm_bindgen::JsValue::from_str("__wbg_ptr"))?;
    if ptr.is_undefined() {
        return Err(Error::NotWasmAbiPointerForClass(class.to_string()));
    }
    let ptr_u32: u32 = ptr
        .as_f64()
        .ok_or(Error::NotWasmAbiPointerForClass(class.to_string()))? as u32;

    Ok(Some(ptr_u32))
}

/// Create a reference to a Rust object from a WASM ABI.
#[inline]
pub fn try_ref_from_abi_safe<T>(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<<T as RefFromWasmAbi>::Anchor, Error>
where
    T: RefFromWasmAbi<Abi = u32>,
{
    let ptr_u32 =
        get_ptr_u32_safe(class, js)?.ok_or_else(|| Error::NotAnObjectOfClass(class.to_string()))?;
    Ok(unsafe { T::ref_from_abi(ptr_u32) })
}

#[inline]
pub fn try_long_ref_from_abi_safe<T>(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<<T as LongRefFromWasmAbi>::Anchor, Error>
where
    T: LongRefFromWasmAbi<Abi = u32>,
{
    let ptr_u32 =
        get_ptr_u32_safe(class, js)?.ok_or_else(|| Error::NotAnObjectOfClass(class.to_string()))?;
    Ok(unsafe { T::long_ref_from_abi(ptr_u32) })
}

#[inline]
pub fn try_ref_mut_from_abi_safe<T>(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<<T as RefMutFromWasmAbi>::Anchor, Error>
where
    T: RefMutFromWasmAbi<Abi = u32>,
{
    let ptr_u32 =
        get_ptr_u32_safe(class, js)?.ok_or_else(|| Error::NotAnObjectOfClass(class.to_string()))?;
    Ok(unsafe { T::ref_mut_from_abi(ptr_u32) })
}

#[inline]
pub fn try_clone_from_abi_safe<T>(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<T, Error>
where
    T: RefFromWasmAbi<Abi = u32> + Clone,
{
    try_ref_from_abi_safe::<T>(class, js).map(|r| r.clone())
}

#[inline]
pub fn try_copy_from_abi_safe<T>(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<T, Error>
where
    T: RefFromWasmAbi<Abi = u32> + Copy,
{
    try_ref_from_abi_safe::<T>(class, js).map(|r| *r)
}

/// Create a reference to a Rust object from a WASM ABI.
/// Returns None is the supplied value is `null` or `undefined`,
/// otherwise attempts to cast the object.
#[inline]
pub fn try_ref_from_abi_safe_as_option<T>(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<Option<<T as RefFromWasmAbi>::Anchor>, JsValue>
where
    T: RefFromWasmAbi<Abi = u32>,
{
    Ok(get_ptr_u32_safe(class, js)?.map(|ptr_u32| unsafe { T::ref_from_abi(ptr_u32) }))
}

#[inline]
pub fn try_ref_mut_from_abi_safe_as_option<T>(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<Option<<T as RefMutFromWasmAbi>::Anchor>, JsValue>
where
    T: RefMutFromWasmAbi<Abi = u32>,
{
    Ok(get_ptr_u32_safe(class, js)?.map(|ptr_u32| unsafe { T::ref_mut_from_abi(ptr_u32) }))
}

#[inline]
pub fn try_clone_from_abi_safe_as_option<T>(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<Option<T>, JsValue>
where
    T: RefFromWasmAbi<Abi = u32> + Clone,
{
    Ok(get_ptr_u32_safe(class, js)?.map(|ptr_u32| unsafe { T::ref_from_abi(ptr_u32).clone() }))
}

#[inline]
pub fn try_copy_from_abi_safe_as_option<T>(
    class: &str,
    js: impl AsRef<JsValue>,
) -> std::result::Result<Option<T>, JsValue>
where
    T: RefFromWasmAbi<Abi = u32> + Copy,
{
    Ok(get_ptr_u32_safe(class, js)?.map(|ptr_u32| unsafe { *T::ref_from_abi(ptr_u32) }))
}
