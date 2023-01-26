## WORKFLOW-TERMINAL

Part of the [WORKFLOW-RS](https://github.com/workflow-rs) application framework.

***

Terminal and command line interface (a custom shell) that runs in the browser and in the native environment.

[![Crates.io](https://img.shields.io/crates/l/workflow-terminal.svg?maxAge=2592000)](https://crates.io/crates/workflow-terminal)
[![Crates.io](https://img.shields.io/crates/v/workflow-terminal.svg?maxAge=2592000)](https://crates.io/crates/workflow-terminal)
![platform](https://img.shields.io/badge/platform-Native-informational)
![platform](https://img.shields.io/badge/platform-Web%20%28wasm32%29-informational)

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

Basic examples on using this crate can be found at: 
https://github.com/workflow-rs/workflow-terminal-examples
