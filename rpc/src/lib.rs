//!
//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rpc-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rpc)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-rpc.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-rpc)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--rpc-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-rpc)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-rpc.svg?maxAge=2592000&color=6ac&style=for-the-badge&logo=opensourceinitiative&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform: client-native-informational?style=for-the-badge&color=69f" height="20">
//! <img src="https://img.shields.io/badge/platform: client-wasm32 (web) -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform: server-native-informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! async Rust-centric RPC framework supporting a custom high-performance `Borsh` and an extended `JSON-RPC` protocols.
//!
//! Features:
//! - High-performance Borsh message encoding protocol
//! - RPC method and notification handler declarations based on serializable generics
//! - Client to Server RPC method invocation
//! - Client to Server notification messages
//! - Server to Client notification messages
//! - Server-side handshake scaffolding for custom connection negotiation
//! - Easy to retain connection data structure for posting async client notifications
//!
//! This framework provides [`server`] and [`client`] modules. The server infrastructure is built on top of
//! [Tokio](https://crates.io/crates/tokio) and [Tungtenite](https://crates.io/crates/tungstenite) and
//! provides scaffolding for connection handshake negotiation.
//!
//! The client is built on top of [Workflow WebSocket](https://crates.io/crates/workflow-websocket) and
//! operates uniformly in native applications and in the browser WASM environment.  For native applications
//! Workflow Websocket uses Tokio and Tungstenite and in the browser environment it uses the browser
//! `WebSocket` object.
//!
//!
//! ### Client-side
//! ```rust
//!     interface.notification(
//!         TestOps::Notify,
//!         notification!(|notification: TestNotify| async move {
//!             // handle notification
//!             Ok(())
//!         }),
//!     );
//!     
//!     let resp: MyResponse = rpc.call(MyOps::MyMethod, MethodRequest { ... }).await?;
//! ```
//! ### Server-side
//! ```rust
//!     interface.method(
//!         TestOps::SomeRequest,
//!         method!(|connection_ctx, server_ctx, request: TestReq| async move {
//!             // handle request and return a response
//!             Ok(SomeResponse { })
//!         }),
//!     );
//!     
//!     interface.notification(
//!         TestOps::Notify,
//!         notification!(
//!             |connection_ctx, server_ctx, notification: TestNotify| async move {
//!                 // handle notification
//!                 Ok(())
//!             }
//!         ),
//!     );
//!     
//! ```
//!

extern crate self as workflow_rpc;

pub mod client;
pub mod error;
pub mod id;
mod imports;
pub mod messages;
pub mod result;
pub mod types;

pub mod encoding;
#[cfg(not(any(target_arch = "wasm32", target_os = "solana")))]
pub mod server;
