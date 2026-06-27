use js_sys::{Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function)]
    pub type RequireResolve;

    #[wasm_bindgen(method)]
    fn paths(this: &RequireResolve, request: &JsString) -> Option<Box<[JsValue]>>;
}
