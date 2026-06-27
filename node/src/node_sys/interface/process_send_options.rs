use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ProcessSendOptions {
    swallow_errors: Option<bool>,
}

#[wasm_bindgen]
impl ProcessSendOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(swallow_errors: Option<bool>) -> ProcessSendOptions {
        ProcessSendOptions { swallow_errors }
    }

    #[wasm_bindgen(getter)]
    pub fn swallow_errors(&self) -> Option<bool> {
        self.swallow_errors
    }

    #[wasm_bindgen(setter)]
    pub fn set_swallow_errors(&mut self, value: Option<bool>) {
        self.swallow_errors = value;
    }
}
