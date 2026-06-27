use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type MemoryUsage;

    #[wasm_bindgen(method, getter)]
    pub fn external(this: &MemoryUsage) -> f64;

    #[wasm_bindgen(method, getter, js_name = "heapTotal")]
    pub fn heap_total(this: &MemoryUsage) -> f64;

    #[wasm_bindgen(method, getter, js_name = "heapUsed")]
    pub fn heap_used(this: &MemoryUsage) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn rss(this: &MemoryUsage) -> f64;
}
