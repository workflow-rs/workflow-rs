## WORKFLOW-WASM

Part of the [WORKFLOW-RS](https://github.com/workflow-rs) application framework.

***

WASM (browser) functionality

[![Crates.io](https://img.shields.io/crates/l/workflow-wasm.svg?maxAge=2592000)](https://crates.io/crates/workflow-wasm)
[![Crates.io](https://img.shields.io/crates/v/workflow-wasm.svg?maxAge=2592000)](https://crates.io/crates/workflow-wasm)
![platform](https://img.shields.io/badge/platform-Web%20%28wasm32%29-informational)

## Features

* `timer` and `interval` functions that wrap JavaScript `setTimeout()` and `setInterval()` returning a handle that encapsulates the JavaScript handle and the callback closure.  Dropping this handle results in the closing of the timeout or interval as well as destruction of the closure. (This is useful to prevent memory leaks when creating JavaScript Closures and using `closure.forget()` functionality)
* `Callback` struct that encapsulates a JavaScript event listener (callback) closure making it easier to creaet and retain JavaScript closures.
* Utility functions that simplify accessing JavaScript object properties and function invocations (based on top of web-sys and js-sys APIs).
