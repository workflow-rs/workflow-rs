## WORKFLOW-DOM

Part of the [WORKFLOW-RS](https://github.com/workflow-rs) application framework.

***

Browser DOM manipulation utilities

[![Crates.io](https://img.shields.io/crates/l/workflow-dom.svg?maxAge=2592000)](https://crates.io/crates/workflow-dom)
[![Crates.io](https://img.shields.io/crates/v/workflow-dom.svg?maxAge=2592000)](https://crates.io/crates/workflow-dom)
![platform](https://img.shields.io/badge/platform-Web%20%28wasm32%29-informational)

## Features

* Dynamic (runtime) injection of JsvaScript modules and CSS data into Browser DOM
* Optionally supplied callback gets invoked upon the successful load.

Combined with [`include_bytes!()`](https://doc.rust-lang.org/std/macro.include_bytes.html) macro this crate can be used to dynamically inject JavaScript and CSS files into the browser environment at runtime.

