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
