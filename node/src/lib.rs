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

pub mod child_process;
pub mod error;
pub mod fs;
pub mod process;
pub mod require;
pub mod result;

pub mod prelude {
    pub use crate::process::*;
}

pub use require::*;
