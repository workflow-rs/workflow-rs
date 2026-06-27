pub use crate::node_sys::interface::assertion_error_options::AssertionErrorOptions;
use js_sys::{Error, Function, JsString, Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "assert")]
extern "C" {
    #[wasm_bindgen(extends = Error)]
    pub type AssertionError;

    #[wasm_bindgen(constructor)]
    pub fn new(options: AssertionErrorOptions) -> AssertionError;

    #[must_use]
    #[wasm_bindgen(catch, js_name = "deepStrictEqual")]
    pub fn deep_strict_equal(
        actual: &JsValue,
        expected: &JsValue,
        message: Option<&JsString>,
    ) -> Result<(), JsValue>;

    #[must_use]
    #[wasm_bindgen(js_name = "doesNotReject")]
    pub fn does_not_reject_function(
        block: &Function,
        error: Option<&Object>,
        message: Option<&JsString>,
    ) -> Promise;

    #[must_use]
    #[wasm_bindgen(js_name = "doesNotReject")]
    pub fn does_not_reject_promise(
        block: &Promise,
        error: Option<&Object>,
        message: Option<&JsString>,
    ) -> Promise;

    #[must_use]
    #[wasm_bindgen(catch, js_name = "doesNotThrow")]
    pub fn does_not_throw(
        block: &Function,
        error: Option<&Object>,
        message: Option<&JsString>,
    ) -> Result<(), JsValue>;

    #[must_use]
    #[wasm_bindgen(catch)]
    pub fn fail(message: Option<&JsString>) -> Result<(), JsValue>;

    #[must_use]
    #[wasm_bindgen(catch, js_name = "ifError")]
    pub fn if_error(value: &JsValue) -> Result<(), JsValue>;

    #[must_use]
    #[wasm_bindgen(catch, js_name = "notDeepStrictEqual")]
    pub fn not_deep_strict_equal(
        actual: &JsValue,
        expected: &JsValue,
        message: Option<&JsString>,
    ) -> Result<(), JsValue>;

    #[must_use]
    #[wasm_bindgen(catch, js_name = "notStrictEqual")]
    pub fn not_strict_equal(
        actual: &JsValue,
        expected: &JsValue,
        message: Option<&JsString>,
    ) -> Result<(), JsValue>;

    #[must_use]
    #[wasm_bindgen(catch)]
    pub fn ok(value: &JsValue, message: Option<&JsString>) -> Result<(), JsValue>;

    #[must_use]
    #[wasm_bindgen(js_name = "rejects")]
    pub fn rejects_function(
        block: &Function,
        error: Option<&Object>,
        message: Option<&JsString>,
    ) -> Promise;

    #[must_use]
    #[wasm_bindgen(js_name = "rejects")]
    pub fn rejects_promise(
        block: &Promise,
        error: Option<&Object>,
        message: Option<&JsString>,
    ) -> Promise;

    #[must_use]
    #[wasm_bindgen(catch, js_name = "strictEqual")]
    pub fn strict_equal(
        actual: &JsValue,
        expected: &JsValue,
        message: Option<&JsString>,
    ) -> Result<(), JsValue>;

    #[must_use]
    #[wasm_bindgen(catch)]
    pub fn throws(
        block: &Function,
        error: Option<&Object>,
        message: Option<&JsString>,
    ) -> Result<(), JsValue>;
}
