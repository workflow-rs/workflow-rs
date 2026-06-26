use crate::node_sys::class::Buffer;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Certificate;

    #[wasm_bindgen(method, js_name = "exportChallenge")]
    pub fn export_challenge(this: &Certificate, spkac: &JsValue) -> Buffer;

    #[wasm_bindgen(method, js_name = "exportPublicKey")]
    pub fn export_public_key(this: &Certificate, spkac: &JsValue, encoding: Option<&str>)
    -> Buffer;

    #[wasm_bindgen(method, js_name = "verifySpkac")]
    pub fn verify_spkac(this: &Certificate, spkac: &JsValue) -> bool;
}
