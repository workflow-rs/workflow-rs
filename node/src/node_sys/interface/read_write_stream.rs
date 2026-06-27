use crate::node_sys::interface::{ReadableStream, WritableStream};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ReadableStream, extends = WritableStream)]
    #[derive(Clone, Debug)]
    pub type ReadWriteStream;
}
