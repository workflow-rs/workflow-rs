//! Raw bindings to the Node.js API for projects using wasm-bindgen.
//!
//! Vendored and internalized from the unmaintained `node-sys` v0.4.2 crate
//! (<https://github.com/interfaces-rs/node-sys>, Apache-2.0 OR MIT). Exposed as
//! `workflow_node::node_sys` so workflow-node no longer depends on the external
//! crate. Lints are relaxed here since this is third-party generated binding
//! code; the only functional change from upstream is renaming the `tty`
//! `ReadStream`/`WriteStream` types (which collided with the `fs` ones under
//! the same JS class name and are rejected by wasm-bindgen >= 0.2.126).
#![allow(
    clippy::all,
    dead_code,
    deprecated,
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    rustdoc::all
)]

pub(crate) mod class;
pub mod globals;
pub(crate) mod interface;
pub(crate) mod module;

pub use class::{Buffer, EventEmitter, Immediate, Timeout, Wasi};
pub use interface::*;
pub use module::*;
