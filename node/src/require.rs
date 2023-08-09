use wasm_bindgen::prelude::*;
use workflow_core::runtime;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = require)]
    pub fn require_impl(s: &str) -> JsValue;
}

pub fn require(s: &str) -> JsValue {
    if runtime::is_web() {
        JsValue::UNDEFINED
    } else {
        require_impl(s)
    }
}
