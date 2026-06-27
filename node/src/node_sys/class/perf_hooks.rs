use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "perf_hooks")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type PerformanceObserver;
}
