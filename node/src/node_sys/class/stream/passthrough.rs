use crate::node_sys::class::stream::Transform;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "stream")]
extern "C" {
    #[wasm_bindgen(extends = Object, extends = Transform)]
    #[derive(Clone, Debug)]
    pub type Passthrough;
}
