# WORKFLOW-RS


WORKFLOW-RS project is designed to provide a unified environment for development
of async Rust application that are able to run in **native** platforms as well as **in-browser**
(by building to WASM32 browser-compatible target).

These crates contain a carefully curated collection of functions and crates (as well as re-exports)
meant to provide a platform-neutral environment framework for Rust applications.

## Rationale

There are multiple reasons behind the creation of this framework, here are the few use cases:

1) At times, the need arises where developers need to use server-side tools to access client-side APIs. For example, initialization of the database layers via existing client-side APIs or providing client-side acces to server-side tools. This framework blurs the distinction between the two allowing creation of APIs that functon symmetrically in both environments (client-side and server-side).
2) Ability to develop server-side and client-side application as a single stack - based on the same codebase. The WebSocket libraries are specifically aimed at that, allowing a developer to create a WebSocket interface that functions symmetrically in both environments.
3) Rapid prototyping of APIs/frameworks using server-side/developer environment and later releasing them as client-side functionality. (i.e. avoid dealing with the browser during the initial development cycle) 
4) Ability to develop a Rust-centric async client-side applications that offer rich UX capabilities without relying on JavaScript dependencies.
5) Creation of high-security applications intended for cryptocurrency and smart-contract development that are not a subject to upstream vendor injection attacks (i.e. do not rely on JavaScript package ecosystems such as NPM) where only Rust is used both server-side and client-side.

## Challenges

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

---

This project is comprised of the following crates (exported as features):

## WORKFLOW-HTML
HTML templating marco meant to offer an easy-to-use runtime html templating against DOM when using async Rust in-browser.
This crate is a foundational pillar behind WORKFLOW-UX crate that offers Rust-based DOM-driven UX creation.

One of the key features of this crate (as offered by workflow-ux crate) is the ability to create HTML links to primitives
such as Rust functions (i.e. clicking on a link invokes a Rust function).  Please see Workflow-UX for more information. 

## WORKFLOW-DOM
Crate for DOM utilities offering JavaScript injection functionality at runtime, allowing you to load JavaScript into
the environment at Runtime using Rust.  (This allows you to embed JavaScript(ES6) modules directly into your Rust crates.

## WORKFLOW-WEBSOCKET
Unified async WebSocket crate that functions symmetically in the native environemnt (using Tokio) and within a browser
using native browser WebSockets.  You do not need to cate which environment you are running under to gain access to a
websocket connection.

## WORKFLOW-RPC

RPC library based on top of WORKFLOW-WEBSOCKET that offers both synchronous and asynchronous Binary and JSON data relay over Workflow-WebSocket-based connections. 

The goal of this crate is to reduce boilerplate as much as possible
allowing remote function invocation using a single function with two generics `rpc.call<Req,Resp>()` where request and response must implement serlialization the respective serialization traits.

Binary RPC uses Borsh serialization and JSON RPC uses Serde serialization.

Current implementation status:
- [x] Asynchronous Binary RPC Client
- [x] Asynchronous Binary RPC Server
- [ ] Asynchronous Binary RPC Server Notifications
- [ ] Synchronous JSON RPC Client
- [ ] Synchronous JSON RPC Server
- [ ] Synchronous RPC Server Notifications

## WORKFLOW-I18N

i18n framework for Workflow-UX Applications (under development)

This framework offers runtime translation of text based on a phrase-dictionary database.

## WORKFLOW-CORE

Core utilities used by the Workflow framework.  These utilities implement as well as re-export curated implementations
that are async-Rust compatible.  Export of this (and other crates) is meant to provide LTS support:

* `#[describe_enum]` macro attribute - offers conversion of enums to and from strings as well as associating a custom
description attribute with each enum value.
* `id` module offering a random 64-bit UUID-like base58-encodable identifier representation (useful for DOM element IDs) 
* `task` module - offers `spawn()` functionality for async code as well as re-exports following modules:
    * `async_std::channel` (offering unbounded and bounded channels)
    * `channel::oneshot` (asias for `async_std::channel::bounded(1)`)
    * `triggered` re-export of the Triggered crate
* `utility` module functions for buffer manipulation

## WORKFLOW-LOG

Logging functionality that is Native, WASM (browser) and BPF-friendly.  On the native platforms, log functions will use Stdout,
in WASM the logging functions will use browser `console.log()` output and on BPF targets they will use `solana.log()` a.k.a. `msg!()`.

This crate also provides the following functionality:
* This crate can be bound to redirect to the the standard Rust `log` crate
* User can register a custom `sink` trait with the logger to capture the log output. (This is especially useful when using the workflow-terminal crate as it allows application log output redirection to a custom terminal)

## WORKFLOW-WASM

A set of WASM helper utility functions for accessing JavaScript object properties.  Also includes the following modules:
* `timer` - wasm_bindgen-based timers and intervals that produce a handle, retention of which denotes
the retention of the listener/callback closure and dropping of which denotes automatic destruction of the timer/interval. (This is 
useful to prevent memory leaks when creating JavaScript Closures and using `closure.forget()` functionality)

## WORKFLOW-TERMINAL

A unified terminal implementation designed to offer a terminal user interface in a native shell (OS) as well as in-browser.
This crate offers a single `Terminal` struct that wraps `XTermJS` in-browser and `Termion` on native platforms.  To interface
with this terminal, user needs to simply implement a `Cli` on his struct and supply it as a command receiver to the terminal.

This implementation is helpful for creating and testing crates that are meant to function in-browser and on native platforms.

For example, if you have a control interface that resides on top of Workflow-WebSocket or Workflow-RPC, you can quickly
prototype a series of functions using Workflow-Terminal that will run in all environments and only later, once ready, implement
the UI wrapping this functionality.

## WORKFLOW-UNISTORE

A crate offering a simple platform-neutral file (data) storage but resolving file paths at runtime based on the OS as well as
supporting browser local-storage.  The client application is meant to provide target paths based on the OS (for example: windows, macos, linux and browser) and then use the struct to read()/write() data in a binary `&[u8]` format.  The browser localstorage functionality uses base64 encoding.

## WORKFLOW-HTTP

HTTP server crate based on Tide HTTP server meant for serving applications based on Workflow-UX user interface library.
This server provides a custom router making it possible to reference FlowUX Web Component libraries in-browser as ES6 modules.

## WORKFLOW-UX

Async Rust + HTML Web Component driven User Interface library.

Please refer to the Workflow-UX README for more information.

## FLOW-UX
FlowUX module is a JavaScript library meant that offers a series of  Web Components based on top of Google's Lit-Html framework. This library internalizes all dependencies out of security considerations.

Flow-UX is an external component that is integral to the Workflow-UX crate.

## WORKFLOW-ALLOCATOR
This crate is an intermadiate-level framework for development of Solana Programs.
This crate is meant to address ergonomics of developing Solana Smart Contracts in Rust, in-browser, while offering
Rust-centric functionality such as client-side account cache, instruction processing, a light-weight Solana Program Emulator and much more.


