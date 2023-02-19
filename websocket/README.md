## `workflow-websocket`

Part of the [`workflow-rs`](https://github.com/workflow-rs) application framework.

***

[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs/workflow--websocket-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-websocket)
[<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-websocket.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-websocket)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--websocket-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/workflow-websocket)
<img alt="license" src="https://img.shields.io/crates/l/workflow-websocket.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
<img src="https://img.shields.io/badge/platform: client-native-informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform: client-wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform: client-wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform: server-native-informational?style=for-the-badge&color=50a0f0" height="20">

Platform-neutral WebSocket Client and native Server.

## Features

* Uniform async Rust WebSocket client API that functions in the browser environment (backed by browser `WebSocket` class) as well as on native platforms (backed by [Tungstenite](https://crates.io/crates/async-tungstenite) client).
* Trait-based WebSocket server API backed by [Tungstenite](https://crates.io/crates/async-tungstenite) server.

This crate allows you to develop a WebSocket client that will work uniformly in in hte native environment and in-browser.

Workflow-WebSocket crate is currently (as of Q3 2022) one of the few available async Rust client-side in-browser WebSocket implementations.

This web socket crate offers an async message send API as well as provides access to [Receiver](https://docs.rs/async-channel/latest/async_channel/struct.Receiver.html) and [Sender](https://docs.rs/async-channel/latest/async_channel/struct.Sender.html) async_std channels ([async_channel])(https://docs.rs/async-channel/latest/async_channel/) that can be used to send and receive WebSocket messages asynchronously.

NOTE: to use `workflow-websocket` in the Node.js environment, you need to introduce a W3C WebSocket object before loading the WASM32 library.
You can use any Node.js module that exposes a W3C-compatible WebSocket implementation. Two of such modules are [WebSocket](https://www.npmjs.com/package/websocket) (provides a custom implementation) and [isomorphic-ws](https://www.npmjs.com/package/isomorphic-ws) (built on top of the [`ws`](https://www.npmjs.com/package/ws) WebSocket module).

You can use the following shims:
```
// WebSocket
globalThis.WebSocket = require('websocket').w3cwebsocket;
// isomorphic-ws
globalThis.WebSocket = require('isomorphic-ws');
```
