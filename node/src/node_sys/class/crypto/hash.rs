use crate::node_sys::{class::stream::Transform, interface::StreamTransformOptions};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Transform)]
    #[derive(Clone, Debug)]
    pub type Hash;

    #[wasm_bindgen]
    pub fn copy(this: &Hash, options: Option<StreamTransformOptions>) -> Hash;

    #[wasm_bindgen(method)]
    pub fn digest(this: &Hash, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn update(this: &Hash, data: &JsValue, input_encoding: Option<&str>) -> Hash;
}
