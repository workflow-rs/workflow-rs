pub use crate::node_sys::class::EventEmitter;

use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "events")]
extern "C" {
    #[must_use]
    pub fn once(emitter: &EventEmitter, name: &str) -> Promise;
}
