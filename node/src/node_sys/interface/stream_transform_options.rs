use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamTransformOptions {
    flush: Function,
    transform: Function,
}

#[wasm_bindgen]
impl StreamTransformOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(flush: Function, transform: Function) -> StreamTransformOptions {
        StreamTransformOptions { flush, transform }
    }

    #[wasm_bindgen(getter)]
    pub fn flush(&self) -> Function {
        self.flush.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_flush(&mut self, value: Function) {
        self.flush = value;
    }

    #[wasm_bindgen(getter)]
    pub fn transform(&self) -> Function {
        self.transform.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_transform(&mut self, value: Function) {
        self.transform = value;
    }
}
