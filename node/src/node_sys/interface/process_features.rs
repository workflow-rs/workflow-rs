use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type ProcessFeatures;

    #[wasm_bindgen(method, getter)]
    pub fn debug(this: &ProcessFeatures) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn inspector(this: &ProcessFeatures) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn ipv6(this: &ProcessFeatures) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn tls(this: &ProcessFeatures) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn tls_alpn(this: &ProcessFeatures) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn tls_ocsp(this: &ProcessFeatures) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn tls_sni(this: &ProcessFeatures) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn uv(this: &ProcessFeatures) -> bool;
}
