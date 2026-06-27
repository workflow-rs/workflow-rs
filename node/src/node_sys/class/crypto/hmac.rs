use crate::node_sys::class::stream::Transform;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Transform)]
    #[derive(Clone, Debug)]
    pub type Hmac;

    #[wasm_bindgen(method)]
    pub fn digest(this: &Hmac, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn update(this: &Hmac, data: &JsValue, input_encoding: Option<&str>);
}
