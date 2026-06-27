//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-node.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-node)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--node-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/workflow-node)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-node.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/Node Webkit -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! Framework compoents for using Node.js and NWJS in WASM environment
//!

/// Bindings to the Node.js `child_process` module for spawning child processes.
pub mod child_process;
pub mod error;
/// Bindings to the Node.js `fs` and `fs/promises` modules for file system access.
pub mod fs;
pub mod node_sys;
pub mod process;
/// Wrapper around the Node.js `require` function for loading native modules.
pub mod require;
pub mod result;

/// Re-exports the most commonly used items of this crate for convenient glob import.
pub mod prelude {
    pub use crate::process::*;
}

pub use require::*;
