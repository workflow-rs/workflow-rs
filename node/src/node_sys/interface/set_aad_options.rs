use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct SetAadOptions {
    flush: Function,
    plaintext_length: f64,
    transform: Function,
}

#[wasm_bindgen]
impl SetAadOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(flush: Function, plaintext_length: f64, transform: Function) -> SetAadOptions {
        SetAadOptions {
            flush,
            plaintext_length,
            transform,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn flush(&self) -> Function {
        self.flush.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_flush(&mut self, value: Function) {
        self.flush = value;
    }

    #[wasm_bindgen(getter, js_name = "plaintextLength")]
    pub fn plaintext_length(&self) -> f64 {
        self.plaintext_length
    }

    #[wasm_bindgen(setter)]
    pub fn set_plaintext_length(&mut self, value: f64) {
        self.plaintext_length = value;
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
