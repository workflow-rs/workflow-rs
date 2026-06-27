use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "events")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Immediate;

    #[wasm_bindgen(method, js_name = "hasRef")]
    pub fn has_ref(this: &Immediate) -> bool;

    #[wasm_bindgen(method, js_name = "ref")]
    pub fn ref_(this: &Immediate) -> Immediate;

    #[wasm_bindgen(method)]
    pub fn unref(this: &Immediate) -> Immediate;
}
