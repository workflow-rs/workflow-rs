//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-i18n.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-i18n)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--i18n-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/workflow-i18n)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-i18n.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- native -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! i18n is a performance-oriented library for internationalization and translation embedding into Rust applications.
//!
pub mod error;
pub mod i18n;
pub mod json;
pub mod result;

pub use i18n::i18n;

pub mod prelude {
    pub use crate::i18n;
}
