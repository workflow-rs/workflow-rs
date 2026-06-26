use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type CpuUsage;

    #[wasm_bindgen(method)]
    pub fn user(this: &CpuUsage) -> f64;

    #[wasm_bindgen(method)]
    pub fn system(this: &CpuUsage) -> f64;
}
