use crate::node_sys::class::stream::Writable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Writable)]
    #[derive(Clone, Debug)]
    pub type Verify;

    #[wasm_bindgen(method)]
    pub fn update(this: &Verify, data: &JsValue, input_encoding: Option<&str>);

    #[wasm_bindgen(method)]
    pub fn verify(
        this: &Verify,
        object: &JsValue,
        signature: &JsValue,
        signature_encoding: Option<&str>,
    ) -> bool;
}
