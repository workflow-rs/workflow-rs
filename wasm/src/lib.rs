//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-wasm.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-wasm)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--wasm-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/workflow-wasm)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-wasm.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/Node Webkit -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! This crate provides utilities for WASM32 environment
//!

extern crate self as workflow_wasm;

pub mod abi;
pub mod callback;
pub mod error;
pub mod extensions;
pub mod init;
pub mod jserror;
pub mod options;
pub mod panic;
pub mod prelude;
pub mod printable;
pub mod result;
pub mod serde;
pub mod utils;

#[cfg(feature = "defer")]
pub mod defer;

#[cfg(feature = "async-stream")]
pub mod stream;
