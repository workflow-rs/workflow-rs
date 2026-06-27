use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MkdtempSyncOptions {
    encoding: Option<JsString>,
}

#[wasm_bindgen]
impl MkdtempSyncOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(encoding: Option<JsString>) -> MkdtempSyncOptions {
        MkdtempSyncOptions { encoding }
    }

    pub fn new() -> MkdtempSyncOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn encoding(&self) -> Option<JsString> {
        self.encoding.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_encoding(&mut self, value: Option<JsString>) {
        self.encoding = value;
    }
}
