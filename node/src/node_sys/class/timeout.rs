use crate::node_sys::interface::Timer;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, extends = Timer)]
    #[derive(Clone, Debug)]
    pub type Timeout;

    #[wasm_bindgen(method, js_name = "hasRef")]
    pub fn has_ref(this: &Timeout) -> bool;

    #[wasm_bindgen(method, js_name = "ref")]
    pub fn ref_(this: &Timeout) -> Timeout;

    #[wasm_bindgen(method)]
    pub fn refresh(this: &Timeout) -> Timeout;

    #[wasm_bindgen(method)]
    pub fn unref(this: &Timeout) -> Timeout;
}
