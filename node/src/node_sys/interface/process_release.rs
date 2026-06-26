use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type ProcessRelease;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &ProcessRelease) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "sourceUrl")]
    pub fn source_url(this: &ProcessRelease) -> Option<JsString>;

    #[wasm_bindgen(method, getter, js_name = "headersUrl")]
    pub fn headers_url(this: &ProcessRelease) -> Option<JsString>;

    #[wasm_bindgen(method, getter, js_name = "libUrl")]
    pub fn lib_url(this: &ProcessRelease) -> Option<JsString>;

    #[wasm_bindgen(method, getter)]
    pub fn lts(this: &ProcessRelease) -> Option<JsString>;
}
