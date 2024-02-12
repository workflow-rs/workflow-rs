//!
//! Handling of WASM panic hook that allows activation of console-based panic hook
//! as well as a browser-based panic hook.  (the browser-based panic hook activates a full-screen debug
//! information output in case of a panic - useful on mobile devices or where
//! the user otherwise has no access to console/developer tools)
//!

use wasm_bindgen::prelude::*;
use workflow_panic_hook::{set_once, show_logs as show_wasm_logs, Type};

/// Initialize panic hook in console mode
/// @category General
#[wasm_bindgen(js_name = "initConsolePanicHook")]
pub fn init_console_panic_hook() {
    set_once(Type::Console);
}

/// Initialize panic hook in browser mode
/// @category General
#[wasm_bindgen(js_name = "initBrowserPanicHook")]
pub fn init_browser_panic_hook() {
    set_once(Type::Popup);
}

/// Present panic logs to the user
/// @category General
#[wasm_bindgen(js_name = "presentPanicHookLogs")]
pub fn show_panic_hook_logs() {
    show_wasm_logs();
}
