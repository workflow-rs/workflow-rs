use crate::node_sys::interface::NodeRequireFunction;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = NodeRequireFunction)]
    pub type NodeRequire;
}
