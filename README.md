## `workflow-rs`

***

[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
<img alt="license" src="https://img.shields.io/crates/l/workflow-dom.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">

WORKFLOW-RS project is designed to provide a unified environment for development of **async Rust applications** that are able to run in *native* platforms (desktops and servers), and `WASM32` environments such as *Web Browsers*, *Node.js* *NWJS (Node Webkit)* and *Electron*.

WORKFLOW-RS is developed by ASPECTRON development team @ https://aspectron.org

## Features

* Platform neutral crates that are able to function in, or provide abstractions for, running on bare metal (native) as well as inside of a browser, Node.js or NWJS WASM-powered environments.

## Crates

This project is comprised of the following crates. These crates contain a carefully curated collection of functions and re-exports meant to provide a platform-neutral environment framework for Rust applications.


* [`workflow-dom`](https://github.com/workflow-rs/workflow-rs/tree/master/dom) - DOM utilities offering JavaScript injection functionality at runtime, allowing you to load JavaScript into the browser environment at Runtime using Rust.  (This allows you to embed JavaScript modules directly into your Rust crates.
* [`workflow-websocket`](https://github.com/workflow-rs/workflow-rs/tree/master/websocket) - WebSocket crate with async Rust API that functions uniformly in the native environemnt (using Tokio) and within a browser using the native browser WebSockets.
* [`workflow-rpc`](https://github.com/workflow-rs/workflow-rs/tree/master/rpc) - RPC crate based on top of `workflow-websocket` that offers asynchronous Binary data relay over Workflow-WebSocket-based connections using Borsh serialization. 
* [`workflow-core`](https://github.com/workflow-rs/workflow-rs/tree/master/core) - Core utilities used by the Workflow framework.  These utilities implement as well as re-export curated implementations
that are compatible with async Rust environment requiring `Send` markers.
* [`workflow-log`](https://github.com/workflow-rs/workflow-rs/tree/master/log) Logging functionality that is Native, WASM (browser) and BPF-friendly.
* [`workflow-wasm`](https://github.com/workflow-rs/workflow-rs/tree/master/wasm) A set of WASM helper modules and utility functions for accessing JavaScript object properties.
* [`workflow-terminal`](https://github.com/workflow-rs/workflow-rs/tree/master/terminal) A unified terminal implementation designed to offer a terminal user interface in a native shell (OS) as well as in-browser. This implementation is helpful for creating and testing crates that are meant to function in-browser and on native platforms.
* [`workflow-html`](https://github.com/workflow-rs/workflow-rs/tree/master/html) HTML templating marco meant to offer an easy-to-use runtime html templating against DOM when using async Rust in-browser. This crate is a foundational pillar behind WORKFLOW-UX crate that offers Rust-based DOM-driven UX creation.
* [`workflow-i18n`](https://github.com/workflow-rs/workflow-rs/tree/master/i18n) i18n framework for Workflow-UX Applications. This framework offers runtime translation of text based on a phrase-dictionary database.
* [`workflow-store`](https://github.com/workflow-rs/workflow-rs/tree/master/store) A crate offering a simple platform-neutral file (data) storage but resolving file paths at runtime based on the OS as well as supporting browser local-storage.

Crates that are a part of this project but are currently outside of this repository:

* [`workflow-ux`](https://github.com/workflow-rs/workflow-ux) Async Rust + HTML Web Component driven application user interface library.


## Examples

Examples for `workflow-websocket`, `workflow-rpc` and `workflow-terminal` are available in the [/examples](https://github.com/workflow-rs/workflow-rs/tree/master/examples) folder.

*** 

## Contributing

This project is under heavy development. Any contributions, ideas or feedback would be very welcome. 