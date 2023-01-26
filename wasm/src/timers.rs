//!
//! Interval and Timeout functions that return an [`IntervalHandle`] or [`TimeoutHandle`] handles
//! dropping which results in automatic clearing of the respective timeout or interval.
//!

use std::sync::{Arc, Mutex};
use thiserror::Error;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Error, Debug)]
pub enum Error {
    #[error("JsValue {0:?}")]
    JsValue(JsValue),

    #[error("Invalid interval handle")]
    InvalidIntervalHandle,

    #[error("Invalid timeout handle")]
    InvalidTimeoutHandle,
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value)
    }
}

pub mod native {
    //! bindings to JavaScript functions
    use js_sys::Function;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {

        /// [`mod@wasm_bindgen`] binding to the native JavaScript [`setInterval()`](https://developer.mozilla.org/en-US/docs/Web/API/setInterval) function
        #[wasm_bindgen (catch, js_name = setInterval)]
        pub fn set_interval(closure: &Function, timeout: u32) -> std::result::Result<u32, JsValue>;

        /// [`mod@wasm_bindgen`] binding to the native JavaScript [`clearInterval()`](https://developer.mozilla.org/en-US/docs/Web/API/clearInterval) function
        #[wasm_bindgen (catch, js_name = clearInterval)]
        pub fn clear_interval(interval: u32) -> std::result::Result<(), JsValue>;

        /// [`mod@wasm_bindgen`] binding to the native JavaScript [`setTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/setTimeout) function
        #[wasm_bindgen (catch, js_name = setTimeout)]
        pub fn set_timeout(closure: &Function, timeout: u32) -> std::result::Result<u32, JsValue>;

        /// [`mod@wasm_bindgen`] binding to the native JavaScript [`clearTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/clearTimeout) function
        #[wasm_bindgen (catch, js_name = clearTimeout)]
        pub fn clear_timeout(interval: u32) -> std::result::Result<(), JsValue>;

    }
}

/// JavaScript interval handle dropping which stops and clears the associated interval
#[derive(Clone, Debug)]
pub struct IntervalHandle(Arc<Mutex<u32>>);

impl Drop for IntervalHandle {
    fn drop(&mut self) {
        let handle = self.0.lock().unwrap();
        if *handle != 0 {
            native::clear_interval(*handle).expect("Unable to clear interval");
        }
    }
}

/// JavaScript timeout handle, droppping which cancels the associated timeout.
#[derive(Clone)]
pub struct TimeoutHandle(Arc<Mutex<u32>>);

impl Drop for TimeoutHandle {
    fn drop(&mut self) {
        let handle = self.0.lock().unwrap();
        if *handle != 0 {
            native::clear_timeout(*handle).expect("Unable to clear timeout");
        }
    }
}

/// Create JavaScript interval
pub fn set_interval(closure: &Closure<dyn FnMut()>, timeout: u32) -> Result<IntervalHandle, Error> {
    let handle = native::set_interval(closure.as_ref().unchecked_ref(), timeout)?;
    Ok(IntervalHandle(Arc::new(Mutex::new(handle))))
}

/// Clear JavaScript interval using a handle returned by [`set_interval`]
pub fn clear_interval(handle: &IntervalHandle) -> Result<(), Error> {
    let mut handle = handle.0.lock().unwrap();
    if *handle != 0 {
        native::clear_timeout(*handle)?;
        *handle = 0;
        Ok(())
    } else {
        Err(Error::InvalidIntervalHandle)
    }
}

/// Create JavaScript timeout
pub fn set_timeout(closure: &Closure<dyn FnMut()>, timeout: u32) -> Result<TimeoutHandle, Error> {
    let handle = native::set_timeout(closure.as_ref().unchecked_ref(), timeout)?;
    Ok(TimeoutHandle(Arc::new(Mutex::new(handle))))
}

/// Clear JavaScript timeout using a handle returns by [`set_timeout`]
pub fn clear_timeout(handle: &TimeoutHandle) -> Result<(), Error> {
    let mut handle = handle.0.lock().unwrap();
    if *handle != 0 {
        native::clear_timeout(*handle)?;
        *handle = 0;
        Ok(())
    } else {
        Err(Error::InvalidTimeoutHandle)
    }
}
