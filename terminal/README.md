## `workflow-terminal`

Part of the [`workflow-rs`](https://github.com/workflow-rs) application framework.

***

Terminal and command line interface (a custom shell) that runs in the browser and in the native environment.


[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-terminal.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-terminal)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--terminal-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-terminal)
<img alt="license" src="https://img.shields.io/crates/l/workflow-terminal.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
<img src="https://img.shields.io/badge/platform- native -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">


## Overview

Workflow Terminal allows you to create a terminal interface that operates symmetrically in OS shell (console)
as well as in the web browser.  This crate is useful if you want to create a command-line interface for an
application meant to run natively on bare metal and in the browser. This crate is especially useful for prototyping
and testing wasm32 browser-compatible platform-neutral applications and crates.

This functionality is achieved by creating a terminal struct `Terminal` that simultaneously wraps:
* [Termion](https://crates.io/crates/termion) - for Native
* [XtermJS](https://github.com/xtermjs/xterm.js) - for Web (Browser)

This crate only depends on a minimal set of other crates and has no external (JavaScript) dependencies.
XtermJS es6 modules are injected directly into DOM during the Terminal initialization phase. This allows the terminal to be loaded using any http server without any additional configuration. (due to browser restrictions, WASM can not be loaded into a static page)

On the backend, you have a simple `Cli` trait which receives user-entered command line.

The Terminal interface also provides basic facilities such as prompt for user text and passwrod entry,
access to command history and binding to logging facilities (in case you want to output to the termina
outside of the terminal command callback).

Please note: this implementation is based on async Rust and is currently hard-wired to run on top of 
tokio in the console and uses wasm_bindgen and web-sys to interface with the browser environment.

Basic examples on using this crate can be found here: 
https://github.com/workflow-rs/workflow-rs/tree/master/examples/terminal

