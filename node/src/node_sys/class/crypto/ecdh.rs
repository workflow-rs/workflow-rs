use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = "ECDH")]
    #[derive(Clone, Debug)]
    pub type Ecdh;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = Ecdh)]
    pub fn convert_key(
        key: &JsValue,
        curve: &str,
        input_encoding: Option<&str>,
        output_encoding: Option<&str>,
        format: Option<&str>,
    ) -> JsValue;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "computeSecret")]
    pub fn compute_secret(
        this: &Ecdh,
        other_public_key: &JsValue,
        input_encoding: Option<&str>,
        output_encoding: Option<&str>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = "generateKeys")]
    pub fn generate_keys(this: &Ecdh, encoding: Option<&str>, format: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "getPrivateKey")]
    pub fn get_private_key(this: &Ecdh, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "getPublicKey")]
    pub fn get_public_key(this: &Ecdh, encoding: Option<&str>, format: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "setPrivateKey")]
    pub fn set_private_key(this: &Ecdh, private_key: &JsValue, encoding: Option<&str>);
}
