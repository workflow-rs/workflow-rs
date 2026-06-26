mod constants;

pub use crate::node_sys::{buffer::constants::BufferConstants, class::Buffer};
use js_sys::{JsString, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "buffer")]
extern "C" {
    #[allow(non_snake_case)]
    pub static INSPECT_MAX_BYTES: f64;

    pub static constants: BufferConstants;

    #[wasm_bindgen(js_name = "kMaxLength")]
    pub static k_max_length: f64;

    pub fn transcode(source: &Uint8Array, from_enc: &JsString, to_enc: &JsString) -> Buffer;
}
