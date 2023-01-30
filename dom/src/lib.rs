//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-dom.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-dom)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--dom-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-dom)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-dom.svg?maxAge=2592000&color=6ac&style=for-the-badge&logo=opensourceinitiative&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! DOM manipulation utilities.
//!
//! Provides DOM injection functionality allowing injecting
//! buffer slices into DOM as [`Blob`](https://developer.mozilla.org/en-US/docs/Web/API/Blob)
//! objects and loading them as `<script>` or `<style>` elements.
//! This crate is useful for embedding JavaScript sources and
//! stylesheets directly into WASM files or loading JavaSctipt code
//! from external resources such as a WebSocket.
//!
//! Example:
//!
//! ```rust ignore
//! use workflow_dom::inject::{inject_blob, Content};
//!
//! let DATA: &[u8] = include_bytes!("source.js");
//! inject_blob(Content::Script(None, data)).await?;
//! ```

pub mod error;
pub mod inject;
pub mod loader;
pub mod result;
pub mod utils;
