use crate::node_sys::class::stream::Duplex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "stream")]
extern "C" {
    #[wasm_bindgen(extends = Duplex)]
    #[derive(Clone, Debug)]
    pub type Transform;
}
