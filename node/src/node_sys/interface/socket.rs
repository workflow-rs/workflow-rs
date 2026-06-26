use crate::node_sys::interface::ReadWriteStream;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ReadWriteStream)]
    pub type Socket;

    #[wasm_bindgen(method, getter, js_name = "isTTY")]
    pub fn is_tty(this: &Socket) -> bool;
}
