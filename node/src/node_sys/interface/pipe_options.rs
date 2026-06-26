use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PipeOptions {
    end: Option<bool>,
}

#[wasm_bindgen]
impl PipeOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(end: Option<bool>) -> PipeOptions {
        PipeOptions { end }
    }

    #[wasm_bindgen(getter)]
    pub fn end(self) -> Option<bool> {
        self.end
    }

    #[wasm_bindgen(setter)]
    pub fn set_end(mut self, value: Option<bool>) {
        self.end = value;
    }
}
