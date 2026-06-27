//!
//! Workflow RS framework initializers
//!

use wasm_bindgen::prelude::*;

// #[wasm_bindgen(js_name = "initApplicationFramework")]
/// Initializes the workflow framework by registering the supplied `workflow` and
/// `modules` objects under the `$workflow$` JavaScript global and enabling log colors.
pub fn init_workflow(workflow: &JsValue, modules: &JsValue) -> std::result::Result<(), JsValue> {
    let global = js_sys::Object::new();
    js_sys::Reflect::set(&js_sys::global(), &"$workflow$".into(), &global)?;
    js_sys::Reflect::set(&global, &"workflow".into(), workflow)?;
    js_sys::Reflect::set(&global, &"modules".into(), modules)?;

    workflow_log::set_colors_enabled(true);

    Ok(())
}

/// Returns the `$workflow$` global object holding the registered workflow and modules.
pub fn global() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&js_sys::global(), &"$workflow$".into())
}

/// Returns the registered workflow object stored in the workflow global.
pub fn workflow() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&global()?, &"workflow".into())
}

/// Returns the registered application modules object stored in the workflow global.
pub fn modules() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&global()?, &"modules".into())
}
