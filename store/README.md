## WORKFLOW-UNISTORE

Part of the [WORKFLOW-RS](https://github.com/workflow-rs) application framework.

***

A simple file I/O abstraction that supports browser localstorage.


[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-store.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-store)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--store-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-store)
<img alt="license" src="https://img.shields.io/crates/l/workflow-store.svg?maxAge=2592000&color=6ac&style=for-the-badge&logo=opensourceinitiative&logoColor=fff" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">

## Features

* A single set of per-operating-system filename configuration options with fallbacks. (i.e. filename for `macos` or `linux` will fallback on `unix` or `generic` if not defined)
* Automatic resolution of user home-folder is using `~` as a path prefix.
* Support for in-browser storage using localstorage and base64 encoding for binary data.


This crate allows you to create a single file reference while specifying multiple per-operating-system file paths, including in-browser localstorage keyname.  Subsequent read/write operations will work against the specified paths.
