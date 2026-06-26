use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tty")]
extern "C" {
    // Renamed from `ReadStream` to avoid a JS class-name collision with
    // `fs::ReadStream` (which owns a `#[wasm_bindgen] impl` block). wasm-bindgen
    // >= 0.2.126 rejects two extern types sharing one JS class name. This type is
    // internal (never re-exported from the crate root) and never instantiated, so
    // giving it a distinct JS identity is inert.
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type TtyReadStream;
}
