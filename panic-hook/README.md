# `workflow-panic-hook`

[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-panic-hook.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-panic-hook)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--panic--hook-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-panic-hook)
<img alt="license" src="https://img.shields.io/crates/l/workflow-panic-hook.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">

## Features

This crate is based on [`console_error_panic_hook`](https://crates.io/crates/console_error_panic_hook) but provides two configuration modes - console output and a full page output, where the panic will create a full-screen `DIV` element in the browser window dumping the stack trace info in it.  This is useful when debugging on devices without access to console output (such as mobile devices).