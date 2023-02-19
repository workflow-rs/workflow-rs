## `workflow-dom`

Part of the [`workflow-rs`](https://github.com/workflow-rs) application framework.

***

Browser DOM manipulation utilities


[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-dom.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-dom)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--dom-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/workflow-dom)
<img alt="license" src="https://img.shields.io/crates/l/workflow-dom.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">

## Features

* Dynamic (runtime) injection of JsvaScript modules and CSS data into Browser DOM
* Optionally supplied callback gets invoked upon the successful load.

Combined with [`include_bytes!()`](https://doc.rust-lang.org/std/macro.include_bytes.html) macro this crate can be used to dynamically inject JavaScript and CSS files into the browser environment at runtime.

