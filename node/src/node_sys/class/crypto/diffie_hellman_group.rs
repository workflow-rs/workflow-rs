use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type DiffieHellmanGroup;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn compute_secret(
        this: &DiffieHellmanGroup,
        other_public_key: &JsValue,
        input_encoding: Option<&str>,
        output_encoding: Option<&str>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = "generateKeys")]
    pub fn generate_keys(this: &DiffieHellmanGroup, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "getGenerator")]
    pub fn get_generator(this: &DiffieHellmanGroup, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "getPrime")]
    pub fn get_prime(this: &DiffieHellmanGroup, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "getPrivateKey")]
    pub fn get_private_key(this: &DiffieHellmanGroup, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "getPublicKey")]
    pub fn get_public_key(this: &DiffieHellmanGroup, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method, js_name = "setPrivateKey")]
    pub fn set_private_key(
        this: &DiffieHellmanGroup,
        private_key: &JsValue,
        encoding: Option<&str>,
    );

    #[wasm_bindgen(method, js_name = "setPublicKey")]
    pub fn set_public_key(this: &DiffieHellmanGroup, public_key: &JsValue, encoding: Option<&str>);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "verifyError")]
    pub fn verify_error(this: &DiffieHellmanGroup) -> u32;
}
