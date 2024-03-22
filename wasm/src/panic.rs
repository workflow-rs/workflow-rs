//!
//! Handling of WASM panic hook that allows activation of console-based panic hook
//! as well as a browser-based panic hook.  (the browser-based panic hook activates a full-screen debug
//! information output in case of a panic - useful on mobile devices or where
//! the user otherwise has no access to console/developer tools)
//!

use wasm_bindgen::prelude::*;
use workflow_panic_hook::{set_once, show_logs as show_wasm_logs, Type};

/// Initialize Rust panic handler in console mode.
///
/// This will output additional debug information during a panic to the console.
/// This function should be called right after loading WASM libraries.
/// @category General
#[wasm_bindgen(js_name = "initConsolePanicHook")]
pub fn init_console_panic_hook() {
    set_once(Type::Console);
}

/// Initialize Rust panic handler in browser mode.
///
/// This will output additional debug information during a panic in the browser
/// by creating a full-screen `DIV`. This is useful on mobile devices or where
/// the user otherwise has no access to console/developer tools. Use
/// {@link presentPanicHookLogs} to activate the panic logs in the
/// browser environment.
/// @see {@link presentPanicHookLogs}
/// @category General
#[wasm_bindgen(js_name = "initBrowserPanicHook")]
pub fn init_browser_panic_hook() {
    set_once(Type::Popup);
}

/// Present panic logs to the user in the browser.
///
/// This function should be called after a panic has occurred and the
/// browser-based panic hook has been activated. It will present the
/// collected panic logs in a full-screen `DIV` in the browser.
/// @see {@link initBrowserPanicHook}
/// @category General
#[wasm_bindgen(js_name = "presentPanicHookLogs")]
pub fn show_panic_hook_logs() {
    show_wasm_logs();
}
