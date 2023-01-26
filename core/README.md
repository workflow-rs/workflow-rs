## WORKFLOW-CORE

Part of the [WORKFLOW-RS](https://github.com/workflow-rs) application framework.

***

[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-core.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-core)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--core-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-core)
<img alt="license" src="https://img.shields.io/crates/l/workflow-core.svg?maxAge=2592000&color=6ac&style=for-the-badge&logo=opensourceinitiative&logoColor=fff" height="20">
<img src="https://img.shields.io/badge/platform- native-informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- solana_os/ignored-informational?style=for-the-badge&color=777787" height="20">


Collection of utilities and curated re-exports that are able to operate on native platforms as well as in the WASM32 Web Browser & Node.js environments.

## Features

* `#[derive(Describe)]` derive macro for enums offering conversion of enums to and from strings as well as associating a custom description attribute with each of the enum values.
* `id` module offering a random 64-bit UUID-like base58-encodable identifier representation (useful for DOM element IDs)
* `task` module offering async `spawn()` functionality for async code task execution as well as re-exports following modules:
    * `async_std::channel`: offering unbounded and bounded channels from [async_std](https://crates.io/crates/async-std)
    * `channel::oneshot`: asias for `async_std::channel::bounded(1)`
    * `triggered`: re-export of the [Triggered](https://crates.io/crates/triggered) crate
* async `sleep()` and `yield_now()` functions
* async `yield_executor()` for higher-level suspension of the browser event loop 
* `utility` module functions for buffer manipulation
