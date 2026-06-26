use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type ParsedPath;

    #[wasm_bindgen(method, getter)]
    pub fn base(this: &ParsedPath) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_base(this: &ParsedPath, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn dir(this: &ParsedPath) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_dir(this: &ParsedPath, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn ext(this: &ParsedPath) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_ext(this: &ParsedPath, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &ParsedPath) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &ParsedPath, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn root(this: &ParsedPath) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_root(this: &ParsedPath, value: JsString);
}
