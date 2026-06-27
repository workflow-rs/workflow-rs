//! Structures for handling JavaScript errors. Specifically this module
//! provides a `JsErrorData` struct which is used to extract information
//! from a `JsValue` that represents a JavaScript error.

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use workflow_core::sendable::Sendable;

/// Extension trait that exposes the `message` property of a [`JsError`].
pub trait JsErrorExtension {
    /// Returns the `message` property of the error as a string.
    fn message(&self) -> String;
}

impl JsErrorExtension for JsError {
    fn message(&self) -> String {
        let inner = JsValue::from(self.clone());
        let msg = js_sys::Reflect::get(&inner, &JsValue::from_str("message"))
            .expect("unable to get error message");
        msg.as_string()
            .expect("unable to convert error message to string")
    }
}

/// Extension trait for reading the `message` property from a [`JsValue`]
/// that represents a JavaScript error.
pub trait JsValueErrorTrait {
    /// Returns the `message` property of the error value as a string.
    fn message(&self) -> String;
}

impl JsValueErrorTrait for JsValue {
    fn message(&self) -> String {
        let msg = js_sys::Reflect::get(self, &JsValue::from_str("message"))
            .expect("unable to get error message");
        msg.as_string()
            .expect("unable to convert error message to string")
    }
}

struct Inner {
    name: Option<String>,
    message: Option<String>,
    cause: Option<String>,
    stack: Option<String>,
    code: Option<String>,
    // origin
    origin: Sendable<JsValue>,
}

/// Owned, cloneable snapshot of a JavaScript error, capturing its `name`,
/// `message`, `cause`, `stack` and `code` fields along with the original
/// `JsValue`. Implements [`std::error::Error`] so it can be used in Rust
/// error handling.
#[derive(Clone)]
pub struct JsErrorData {
    inner: Arc<Inner>,
}

impl std::error::Error for JsErrorData {}

impl JsErrorData {
    /// The error name (e.g. `"TypeError"`), if present on the JavaScript error.
    pub fn name(&self) -> &Option<String> {
        &self.inner.name
    }

    /// The human-readable error message, if present on the JavaScript error.
    pub fn message(&self) -> &Option<String> {
        &self.inner.message
    }

    /// The underlying cause of the error, if present on the JavaScript error.
    pub fn cause(&self) -> &Option<String> {
        &self.inner.cause
    }

    /// The captured stack trace, if present on the JavaScript error.
    pub fn stack(&self) -> &Option<String> {
        &self.inner.stack
    }

    /// The error code, if present on the JavaScript error.
    pub fn code(&self) -> &Option<String> {
        &self.inner.code
    }
}

impl std::fmt::Debug for JsErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsErrorData")
            .field("name", &self.inner.name)
            .field("message", &self.inner.message)
            .field("cause", &self.inner.cause)
            .field("stack", &self.inner.stack)
            .field("code", &self.inner.code)
            .finish()
    }
}

impl std::fmt::Display for JsErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .message
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        )
    }
}

impl From<JsValue> for JsErrorData {
    fn from(error: JsValue) -> Self {
        let name = js_sys::Reflect::get(&error, &"name".into())
            .ok()
            .and_then(|v| v.as_string());
        let message = js_sys::Reflect::get(&error, &"message".into())
            .ok()
            .and_then(|v| v.as_string());
        let cause = js_sys::Reflect::get(&error, &"cause".into())
            .ok()
            .and_then(|v| v.as_string());
        let stack = js_sys::Reflect::get(&error, &"stack".into())
            .ok()
            .and_then(|v| v.as_string());
        let code = js_sys::Reflect::get(&error, &"code".into())
            .ok()
            .and_then(|v| v.as_string());

        Self {
            inner: Arc::new(Inner {
                name,
                message,
                cause,
                stack,
                code,
                origin: Sendable::new(error),
            }),
        }
    }
}

impl From<JsError> for JsErrorData {
    fn from(error: JsError) -> Self {
        JsValue::from(error).into()
    }
}

impl From<JsErrorData> for JsValue {
    fn from(error: JsErrorData) -> Self {
        error.inner.origin.as_ref().clone()
    }
}
