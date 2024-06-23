//!
//! Abortable trigger, can be used to cancel (abort) an asynchronous task.
//!

use wasm_bindgen::prelude::*;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Error emitted by [`Abortable`].
/// @category General
#[wasm_bindgen]
pub struct Aborted;

impl std::error::Error for Aborted {}

impl std::fmt::Debug for Aborted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "task aborted")
    }
}

impl std::fmt::Display for Aborted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "task aborted")
    }
}

///
/// Abortable trigger wraps an `Arc<AtomicBool>`, which can be cloned
/// to signal task terminating using an atomic bool.
///
/// ```
/// let abortable = Abortable::default();
/// let result = my_task(abortable).await?;
/// // ... elsewhere
/// abortable.abort();
/// ```
///
/// @category General
/// 
#[derive(Default, Clone)]
#[wasm_bindgen]
pub struct Abortable(Arc<AtomicBool>);

#[wasm_bindgen]
impl Abortable {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    #[inline]
    #[wasm_bindgen(js_name=isAborted)]
    pub fn is_aborted(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn abort(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    #[inline]
    pub fn check(&self) -> Result<(), Aborted> {
        if self.is_aborted() {
            Err(Aborted)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn reset(&self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

impl TryFrom<&JsValue> for Abortable {
    type Error = JsValue;
    fn try_from(value: &JsValue) -> Result<Self, Self::Error> {
        use wasm_bindgen::convert::*;

        let idx = IntoWasmAbi::into_abi(value);
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        ))]
        extern "C" {
            fn __wbg_abortable_unwrap(ptr: u32) -> u32;
        }
        #[cfg(not(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )))]
        unsafe fn __wbg_abortable_unwrap(_: u32) -> u32 {
            panic!("cannot convert from JsValue outside of the wasm target")
        }
        let ptr = unsafe { __wbg_abortable_unwrap(idx) };
        if ptr == 0 {
            wasm_bindgen::__rt::std::result::Result::Err(value.clone())
        } else {
            unsafe {
                wasm_bindgen::__rt::std::result::Result::Ok(
                    <Self as FromWasmAbi>::from_abi(ptr).clone(),
                )
            }
        }
    }
}
