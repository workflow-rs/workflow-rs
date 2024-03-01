//! WASM bindgen casting and utility functions

use std::borrow::Borrow;
use std::ops::Deref;

use crate::error::Error;
use wasm_bindgen::convert::{LongRefFromWasmAbi, RefFromWasmAbi, RefMutFromWasmAbi};
use wasm_bindgen::prelude::*;
pub use workflow_wasm_macros::CastFromJs;

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
    pub fn into_owned(self) -> T {
        match self {
            Cast::Ref(r) => (*r).clone(),
            Cast::LongRef(r) => r.borrow().clone(),
            Cast::Value(v) => v,
        }
    }
}

impl<T> From<T> for Cast<T>
where
    T: RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32>,
{
    fn from(value: T) -> Cast<T> {
        Cast::Value(value)
    }
}

pub trait CastFromJs
where
    Self: Sized + RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32>,
{
    /// Obtain safe reference from [`JsValue`]
    fn try_ref_from_js_value(
        js: impl AsRef<JsValue>,
    ) -> std::result::Result<<Self as RefFromWasmAbi>::Anchor, Error>;
    /// Obtain safe long reference from [`JsValue`]
    fn try_long_ref_from_js_value(
        js: impl AsRef<JsValue>,
    ) -> std::result::Result<<Self as RefFromWasmAbi>::Anchor, Error>;
}

pub trait TryCastFromJs
where
    Self: CastFromJs + RefFromWasmAbi<Abi = u32> + LongRefFromWasmAbi<Abi = u32> + Clone,
{
    type Error: std::fmt::Display + From<Error>;

    /// Try to cast a JsValue into a Rust object.
    /// This should be user-defined function that
    /// attempts to cast a JsValue into a Rust object
    /// or interpret a source data and create a
    /// temporary struct owned by by the [`Container`].
    fn try_cast_from(value: impl AsRef<JsValue>) -> std::result::Result<Cast<Self>, Self::Error>;

    /// Perform a user cast and consume the [`Cast`] container.
    /// This function will return a temporary user-created
    /// object created during [`try_cast_from`] or a clone of the casted reference.
    fn try_value_from(value: impl AsRef<JsValue>) -> std::result::Result<Self, Self::Error> {
        Self::try_cast_from(value).map(|c| c.into_owned())
    }
    /// Try to cast a JsValue into a Rust object.
    /// Returns `Some(Ok(Container<T>))` if the JsValue
    /// was casted successfully. Returns `None` if the
    /// cast has failed.
    fn try_ref_from(
        js: impl AsRef<JsValue>,
    ) -> Result<Cast<Self>, Self::Error> {
        Ok(Self::try_ref_from_js_value(js).map(Cast::Ref)?)
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

}

pub trait TryCastJsInto<T>
where
    T: TryCastFromJs,
{
    type Error: From<Error>;
    fn try_cast_into(&self) -> std::result::Result<Cast<T>, Self::Error>;
    fn try_ref_into(&self) -> std::result::Result<Cast<T>, Self::Error>;
}

impl<T> TryCastJsInto<T> for JsValue
where
    T: TryCastFromJs,
    <T as TryCastFromJs>::Error: From<Error>,
{
    type Error = <T as TryCastFromJs>::Error;
    fn try_cast_into(&self) -> std::result::Result<Cast<T>, Self::Error> {
        T::try_cast_from(self)
    }

    fn try_ref_into(&self) -> std::result::Result<Cast<T>, Self::Error> {
        T::try_ref_from(self)
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
