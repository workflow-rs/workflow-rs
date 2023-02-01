//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs/workflow--websocket-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-websocket)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-websocket.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-websocket)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--websocket-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-websocket)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-websocket.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
//!
//!  <img src="https://img.shields.io/badge/client -native-informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/client -wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/client -wasm32/Node Webkit -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/server -native-informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! [`workflow-websocket`](self) crate provides async websocket client and
//! server interfaces. The client interface operates uniformally in native
//! and in the browser-WASM environment.
//!
//! This crate allows you to design APIs that work in regular native rust applications
//! and function the same in the browser.  If used as a foundation for APIs, this crate
//! makes APIs portable, allowing their use in native/command-line/desktop applications
//! and web-applications alike.
//!
//! - [`client::WebSocket`] operates in browser-WASM or native/tokio-backed environment
//! - [`server::WebSocketServer`] operates only in native/tokio-backed environment
//!

pub mod client;
#[cfg(not(target_arch = "wasm32"))]
pub mod server;
