## WORKFLOW-CORE

Part of the [WORKFLOW-RS](https://github.com/workflow-rs) application framework.

***

Collection of utilities and curated re-exports that are able to operate on native platforms as well as in-browser _in the async Rust environment requiring `Send` markers_.

[![Crates.io](https://img.shields.io/crates/l/workflow-core.svg?maxAge=2592000)](https://crates.io/crates/workflow-core)
[![Crates.io](https://img.shields.io/crates/v/workflow-core.svg?maxAge=2592000)](https://crates.io/crates/workflow-core)
![platform](https://img.shields.io/badge/platform-Native-informational)
![platform](https://img.shields.io/badge/platform-Web%20%28wasm32%29-informational)
![platform](https://img.shields.io/badge/platform-BPF%20%28ignores%29-informational)

## Features

* `#[describe_enum]` enum macro attribute offering conversion of enums to and from strings as well as associating a custom description attribute with each of the enum values.
* `id` module offering a random 64-bit UUID-like base58-encodable identifier representation (useful for DOM element IDs)
* `task` module offering async `spawn()` functionality for async code task execution as well as re-exports following modules:
    * `async_std::channel` (offering unbounded and bounded channels from [async_std](https://crates.io/crates/async-std))
    * `channel::oneshot` (asias for `async_std::channel::bounded(1)`)
    * `triggered` re-export of the [Triggered](https://crates.io/crates/triggered) crate
* async `sleep()` and `yield_now()` functions
* `utility` module functions for buffer manipulation
