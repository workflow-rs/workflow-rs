use wasm_bindgen::prelude::*;
use workflow_core::runtime;

#[wasm_bindgen]
extern "C" {
    /// Binding to the Node.js `require` function for loading a module by name.
    #[wasm_bindgen(js_name = require)]
    pub fn require_impl(s: &str) -> JsValue;
}

/// Loads a Node.js module via `require`, returning [`JsValue::UNDEFINED`] when
/// running in a web (browser) environment where `require` is unavailable.
pub fn require(s: &str) -> JsValue {
    if runtime::is_web() {
        JsValue::UNDEFINED
    } else {
        require_impl(s)
    }
}
