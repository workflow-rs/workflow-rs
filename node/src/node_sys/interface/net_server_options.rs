use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NetServerOptions {
    allow_half_open: Option<bool>,
    pause_on_connect: Option<bool>,
}

#[wasm_bindgen]
impl NetServerOptions {
    #[wasm_bindgen(getter)]
    pub fn allow_half_open(self) -> Option<bool> {
        self.allow_half_open
    }

    #[wasm_bindgen(setter)]
    pub fn set_allow_half_open(mut self, value: Option<bool>) {
        self.allow_half_open = value;
    }

    #[wasm_bindgen(getter)]
    pub fn pause_on_connect(self) -> Option<bool> {
        self.pause_on_connect
    }

    #[wasm_bindgen(setter)]
    pub fn set_pause_on_connect(mut self, value: Option<bool>) {
        self.pause_on_connect = value;
    }
}
