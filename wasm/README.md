## WORKFLOW-WASM

Part of the [WORKFLOW-RS](https://github.com/workflow-rs) application framework.

***

WASM (browser) functionality


[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-wasm.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-wasm)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--wasm-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-wasm)
<img alt="license" src="https://img.shields.io/crates/l/workflow-wasm.svg?maxAge=2592000&color=6ac&style=for-the-badge&logo=opensourceinitiative&logoColor=fff" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">

## Features

* `timer` and `interval` functions that wrap JavaScript `setTimeout()` and `setInterval()` returning a handle that encapsulates the JavaScript handle and the callback closure.  Dropping this handle results in the closing of the timeout or interval as well as destruction of the closure. (This is useful to prevent memory leaks when creating JavaScript Closures and using `closure.forget()` functionality)
* `Callback` struct that encapsulates a JavaScript event listener (callback) closure making it easier to creaet and retain JavaScript closures.
* Utility functions that simplify accessing JavaScript object properties and function invocations (based on top of web-sys and js-sys APIs).
