use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AppendFileOptions {
    encoding: Option<JsString>,
    mode: Option<u32>,
    flag: Option<JsString>,
}

#[wasm_bindgen]
impl AppendFileOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        encoding: Option<JsString>,
        mode: Option<u32>,
        flag: Option<JsString>,
    ) -> AppendFileOptions {
        AppendFileOptions {
            encoding,
            mode,
            flag,
        }
    }

    pub fn new() -> AppendFileOptions {
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

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> Option<u32> {
        self.mode
    }

    #[wasm_bindgen(setter)]
    pub fn set_mode(&mut self, value: Option<u32>) {
        self.mode = value;
    }

    #[wasm_bindgen(getter)]
    pub fn flag(&self) -> Option<JsString> {
        self.flag.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_flag(&mut self, value: Option<JsString>) {
        self.flag = value;
    }
}
