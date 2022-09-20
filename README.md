# WORKFLOW-RS

WORKFLOW-RS project is designed to provide a unified environment for development of **async Rust applications** that are able to run in **native** platforms (desktops/servers) as well as **in-browser**
(by building to WASM32 browser-compatible target).

These crates contain a carefully curated collection of functions and crates (as well as re-exports) meant to provide a platform-neutral environment framework for Rust applications.

# Features:

* Platform neutral crates that are able to function in, or provide abstractions for, running on bare metal as well as inside of a browser WASM-powered environment.
* BPF-friendly environment that allows certain crates when building to the BPF targets.

# Crates:

This project is comprised of the following crates (exported as features):

* [**WORKFLOW-DOM**](https://github.com/workflow-rs/workflow-dom) Crate for DOM utilities offering JavaScript injection functionality at runtime, allowing you to load JavaScript into the browser environment at Runtime using Rust.  (This allows you to embed JavaScript modules directly into your Rust crates.
* [**WORKFLOW-WEBSOCKET**](https://github.com/workflow-rs/workflow-websocket) WebSocket crate with async Rust API that functions uniformly in the native environemnt (using Tokio) and within a browser using the native browser WebSockets.
* [**WORKFLOW-RPC**](https://github.com/workflow-rs/workflow-rpc) RPC library based on top of WORKFLOW-WEBSOCKET that offers asynchronous Binary data relay over Workflow-WebSocket-based connections using Borsh serialization. 
* [**WORKFLOW-CORE**](https://github.com/workflow-rs/workflow-core) Core utilities used by the Workflow framework.  These utilities implement as well as re-export curated implementations
that are compatible with async Rust environment requiring `Send` markers.
* [**WORKFLOW-LOG**](https://github.com/workflow-rs/workflow-log) Logging functionality that is Native, WASM (browser) and BPF-friendly.
* [**WORKFLOW-WASM**](https://github.com/workflow-rs/workflow-wasm) A set of WASM helper modules and utility functions for accessing JavaScript object properties.
* [**WORKFLOW-TERMINAL**](https://github.com/workflow-rs/workflow-terminal) A unified terminal implementation designed to offer a terminal user interface in a native shell (OS) as well as in-browser. This implementation is helpful for creating and testing crates that are meant to function in-browser and on native platforms.
* [**WORKFLOW-HTML**](https://github.com/workflow-rs/workflow-html) HTML templating marco meant to offer an easy-to-use runtime html templating against DOM when using async Rust in-browser. This crate is a foundational pillar behind WORKFLOW-UX crate that offers Rust-based DOM-driven UX creation.
* [**WORKFLOW-I18N**](https://github.com/workflow-rs/workflow-i18n) i18n framework for Workflow-UX Applications. This framework offers runtime translation of text based on a phrase-dictionary database.
* [**WORKFLOW-HTTP**](https://github.com/workflow-rs/workflow-http) HTTP server crate based on Tide HTTP server meant for serving applications based on Workflow-UX user interface library.
* [**WORKFLOW-UX**](https://github.com/workflow-rs/workflow-ux) Async Rust + HTML Web Component driven application user interface library.
* [**WORKFLOW-UNISTORE**](https://github.com/workflow-rs/workflow-unistore) A crate offering a simple platform-neutral file (data) storage but resolving file paths at runtime based on the OS as well as
supporting browser local-storage.
* [**WORKFLOW-ALLOCATOR**](https://github.com/workflow-rs/workflow-allocator) A security-oriented framework for developing Solana Programs (Smart Contract) and Solana client-side applications using pure async Rust.

*** 

# Rationale:

There are multiple reasons behind the creation of this framework, here are the few use cases:

1) At times, the need arises where developers need to use server-side tools to access client-side APIs. For example, initialization of the database layers via existing client-side APIs or providing client-side acces to server-side tools. This framework blurs the distinction between the two allowing creation of APIs that functon symmetrically in both environments (client-side and server-side).
2) Ability to develop server-side and client-side application as a single stack - based on the same codebase. The WebSocket libraries are specifically aimed at that, allowing a developer to create a WebSocket interface that functions symmetrically in both environments.
3) Rapid prototyping of APIs/frameworks using server-side/developer environment and later releasing them as client-side functionality. (i.e. avoid dealing with the browser during the initial development cycle) 
4) Ability to develop a Rust-centric async client-side applications that offer rich UX capabilities without relying on JavaScript dependencies.
5) Creation of high-security applications intended for cryptocurrency and smart-contract development that are not a subject to upstream vendor injection attacks (i.e. do not rely on JavaScript package ecosystems such as NPM) where only Rust is used both server-side and client-side.

# Design & Challenges:

As of Q3 2022, Rust ecosystem supports WASM targets that are able to run in a single-threaded browser environment (async_std + wasm_bindgen etc.)
as well on multi-threaded "bare metal" platforms (tokio etc.).  However, due to the fact that browser environment is
single-threaded, many libraries that are compatible with WASM32 do not implement the `Send` marker support in
their functionality, resulting in client-side functionality that can not be re-used server-side.

WORKFLOW-RS project is meant to addres this by curating (re-exporting) various essential functionality that is `Send`-compatible as well as implementing various async-based abstractions (such as WebSockets) that are meant to function symmetrically client and server-side.

As a result of the requirements imposed by this framework, all in-browser functionality must be Send-compatible
which means that even though browsers offer single-threaded executor environment, all underlying functionality
must be "thread-safe".  This means that even though you are running single-threaded, applications using this framework must utilize
thread-safe primitives such as `Arc`, `Mutex`, `RwLock` etc., to ensure that data has a thread-safe inner mutability.

The end-result, however, is extremely rewarding as it allows you to develop Rust libraries and applications that are,
as described above, able to run on native platforms as well as in-browser.

This crate is a composition of external crates exported as features.  You can use external crates directly or
import this single crate and enable features as-needed.  The goal behind this crate is to ensure that all re-exported
functionality and crates are compatible across different versions.

Please note that some of the crates are also BPF-friendly (like `workflow-log`), making them compatible for use 
with Solana Programs (Solana Smart Contracts).
