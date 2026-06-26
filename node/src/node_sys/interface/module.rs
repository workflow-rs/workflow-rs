use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Module;

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &Module) -> Array;

    #[wasm_bindgen(method, getter)]
    pub fn exports(this: &Module) -> Object;

    #[wasm_bindgen(method, getter)]
    pub fn filename(this: &Module) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Module) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn loaded(this: &Module) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn parent(this: &Module) -> Module;

    #[wasm_bindgen(method, getter)]
    pub fn paths(this: &Module) -> Array;

    #[wasm_bindgen(method)]
    pub fn require(this: &Module, id: &JsString) -> JsValue;
}
