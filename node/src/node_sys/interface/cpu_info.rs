use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type CpuInfo;

    #[wasm_bindgen(method, getter)]
    pub fn model(this: &CpuInfo) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn speed(this: &CpuInfo) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn times(this: &CpuInfo) -> CpuInfoTimes;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type CpuInfoTimes;

    #[wasm_bindgen(method, getter)]
    pub fn idle(this: &CpuInfoTimes) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn irq(this: &CpuInfoTimes) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn nice(this: &CpuInfoTimes) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn sys(this: &CpuInfoTimes) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn user(this: &CpuInfoTimes) -> f64;
}
