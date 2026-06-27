use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type ProcessVersions;

    #[wasm_bindgen(method, getter)]
    pub fn ares(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn brotli(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn cldr(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn icu(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn llhttp(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn modules(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn napi(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn nghttp2(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn node(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn openssl(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn tz(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn unicode(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn uv(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn v8(this: &ProcessVersions) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn zlib(this: &ProcessVersions) -> JsString;
}
