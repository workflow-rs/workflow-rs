use crate::node_sys::interface::{NodeModule, NodeRequireFunction, RequireResolve};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = NodeRequireFunction)]
    pub type NodeRequire;

    pub static require: NodeRequire;

    #[wasm_bindgen(method, getter)]
    pub fn cache(this: &NodeRequire) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn main(this: &NodeRequire) -> Option<NodeModule>;

    #[wasm_bindgen(method, getter)]
    pub fn resolve(this: &NodeRequire) -> RequireResolve;
}
