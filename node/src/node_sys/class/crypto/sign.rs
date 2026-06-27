use crate::node_sys::class::stream::Writable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Writable)]
    #[derive(Clone, Debug)]
    pub type Sign;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn sign(this: &Sign, private_key: &JsValue, output_encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn update(this: &Sign, data: &JsValue, input_encoding: Option<&str>);
}
