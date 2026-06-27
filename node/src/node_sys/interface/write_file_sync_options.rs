use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WriteFileSyncOptions {
    encoding: Option<JsString>,
    flag: Option<JsString>,
    mode: Option<u32>,
}

#[wasm_bindgen]
impl WriteFileSyncOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        encoding: Option<JsString>,
        flag: Option<JsString>,
        mode: Option<u32>,
    ) -> WriteFileSyncOptions {
        WriteFileSyncOptions {
            encoding,
            flag,
            mode,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn encoding(&self) -> Option<JsString> {
        self.encoding.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_encoding(&mut self, value: Option<JsString>) {
        self.encoding = value;
    }

    #[wasm_bindgen(getter)]
    pub fn flag(&self) -> Option<JsString> {
        self.flag.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_flag(&mut self, value: Option<JsString>) {
        self.flag = value;
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> Option<u32> {
        self.mode
    }

    #[wasm_bindgen(setter)]
    pub fn set_mode(&mut self, value: Option<u32>) {
        self.mode = value;
    }
}
