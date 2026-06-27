use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "repl")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Recoverable;
}
