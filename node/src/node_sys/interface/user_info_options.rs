use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserInfoOptions {
    encoding: Option<JsString>,
}

#[wasm_bindgen]
impl UserInfoOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(encoding: Option<JsString>) -> UserInfoOptions {
        UserInfoOptions { encoding }
    }

    pub fn new() -> UserInfoOptions {
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
