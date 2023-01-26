use wasm_bindgen::prelude::*;

pub mod error;
pub mod result;
pub mod process;

pub mod prelude {
    pub use crate::process::*;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn require(s: &str) -> JsValue;
}
