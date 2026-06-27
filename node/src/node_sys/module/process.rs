use crate::node_sys::interface::Process;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub static process: Process;
}
