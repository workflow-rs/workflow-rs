use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "util")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type TextEncoder;
}
