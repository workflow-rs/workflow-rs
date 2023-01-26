//!
//! Workflow RS framework initializers
//!

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "init_workflow")]
pub fn init_workflow(workflow: &JsValue, modules: &JsValue) -> std::result::Result<(), JsValue> {
    let global = js_sys::Object::new();
    js_sys::Reflect::set(&js_sys::global(), &"$workflow$".into(), &global)?;
    js_sys::Reflect::set(&global, &"workflow".into(), workflow)?;
    js_sys::Reflect::set(&global, &"modules".into(), modules)?;
    Ok(())
}

pub fn global() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&js_sys::global(), &"$workflow$".into())
}

pub fn workflow() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&global()?, &"workflow".into())
}

pub fn modules() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&global()?, &"modules".into())
}
