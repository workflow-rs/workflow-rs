use crate::node_sys::{
    class::{Buffer, stream::Transform},
    interface::SetAadOptions,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Transform)]
    #[derive(Clone, Debug)]
    pub type Cipher;

    #[wasm_bindgen(method, js_name = "final")]
    pub fn final_(this: &Cipher, output_encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "setAAD")]
    pub fn set_aad(this: &Cipher, buffer: &JsValue, options: Option<SetAadOptions>) -> Cipher;

    #[wasm_bindgen(method, js_name = "getAuthTag")]
    pub fn get_auth_tag(this: &Cipher) -> Buffer;

    #[wasm_bindgen(method, js_name = "getAutoPadding")]
    pub fn get_auto_padding(this: &Cipher, auto_padding: Option<bool>) -> Cipher;

    #[wasm_bindgen(method)]
    pub fn update(
        this: &Cipher,
        data: &JsValue,
        input_encoding: Option<&str>,
        output_encoding: Option<&str>,
    ) -> JsValue;
}
