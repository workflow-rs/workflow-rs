use crate::node_sys::interface::KeyObjectExportOptions;
use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type KeyObject;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn export(this: &KeyObject, options: Option<KeyObjectExportOptions>) -> JsValue;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "asymmetricKeyType")]
    pub fn asymmetric_key_type(this: &KeyObject) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "symmetricKeySize")]
    pub fn symmetric_key_size(this: &KeyObject) -> f64;

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn type_(this: &KeyObject) -> JsString;
}
