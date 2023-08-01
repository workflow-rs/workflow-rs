use std::sync::Arc;

use wasm_bindgen::prelude::*;
use workflow_core::sendable::Sendable;

pub trait JsErrorExtension {
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

pub trait JsValueErrorTrait {
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

#[derive(Clone)]
pub struct JsErrorData {
    inner: Arc<Inner>,
}

impl JsErrorData {
    pub fn name(&self) -> &Option<String> {
        &self.inner.name
    }

    pub fn message(&self) -> &Option<String> {
        &self.inner.message
    }

    pub fn cause(&self) -> &Option<String> {
        &self.inner.cause
    }

    pub fn stack(&self) -> &Option<String> {
        &self.inner.stack
    }

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
        error.inner.origin.clone().into()
    }
}
