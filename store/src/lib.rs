//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-store.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-store)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--store-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/workflow-store)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-store.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! This crate provides an abstraction layer for storing and loading
//! data in different environments: File I/O on desktop devices and
//! local storage when running in the browser.  The goal behind this
//! crate is to allow for a single initialization-phase configuration,
//! following which the API can be used throughout the application
//! without the concern about the operating environment.
//!
//!
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(not(target_arch = "bpf"))] {
        pub mod prelude;
        pub mod error;
        pub mod result;
        pub mod fs;
        pub mod store;
    }
}
