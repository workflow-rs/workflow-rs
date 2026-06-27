use crate::node_sys::{class::stream::Transform, interface::SetAadOptions};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Transform)]
    #[derive(Clone, Debug)]
    pub type Decipher;

    #[wasm_bindgen(method, js_name = "final")]
    pub fn final_(this: &Decipher, output_encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "setAAD")]
    pub fn set_aad(this: &Decipher, buffer: &JsValue, options: Option<SetAadOptions>) -> Decipher;

    #[wasm_bindgen(method, js_name = "getAuthTag")]
    pub fn set_auth_tag(this: &Decipher, buffer: &JsValue) -> Decipher;

    #[wasm_bindgen(method, js_name = "getAutoPadding")]
    pub fn set_auto_padding(this: &Decipher, auto_padding: Option<bool>) -> Decipher;

    #[wasm_bindgen(method)]
    pub fn update(
        this: &Decipher,
        data: &JsValue,
        input_encoding: Option<&str>,
        output_encoding: Option<&str>,
    ) -> JsValue;
}
