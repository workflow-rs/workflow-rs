use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type BufferConstants;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn MAX_LENGTH(this: &BufferConstants) -> f64;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn MAX_STRING_LENGTH(this: &BufferConstants) -> f64;
}
