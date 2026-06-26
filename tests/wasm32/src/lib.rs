//! wasm-bindgen codegen smoke test.
//!
//! This crate contains no logic. It depends on every workflow-rs crate that
//! emits `#[wasm_bindgen]` bindings and links each one (`use ... as _`) so that
//! building it as a cdylib and running the wasm-bindgen CLI over the result
//! (`wasm-pack build tests/wasm32`) exercises the wasm-bindgen codegen pass for
//! all of their bindings simultaneously.
//!
//! `cargo check`/`cargo clippy` only run the compiler; they never invoke the
//! wasm-bindgen CLI, so a whole class of errors (duplicate JS class names, a
//! `#[wasm_bindgen] impl` on an imported extern type, etc.) is invisible to
//! them and only surfaces when the CLI runs. Keeping this crate in `./check`
//! means those errors are caught for the entire workspace, not just whichever
//! crate a downstream app happens to bundle.

#![allow(unused_imports)]

use workflow_chrome as _;
use workflow_core as _;
use workflow_d3 as _;
use workflow_dom as _;
use workflow_egui as _;
use workflow_html as _;
use workflow_http as _;
use workflow_i18n as _;
use workflow_log as _;
use workflow_node as _;
use workflow_nw as _;
use workflow_panic_hook as _;
use workflow_rpc as _;
use workflow_store as _;
use workflow_task as _;
use workflow_terminal as _;
use workflow_wasm as _;
use workflow_websocket as _;
