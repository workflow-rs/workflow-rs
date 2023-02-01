//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-log.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-log)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--log-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-log)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-log.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- native -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- solana_os -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! [`workflow_log`] is a part of the [`workflow-rs`](https://crates.io/workflow-rs)
//! framework, subset of which is designed to function uniformally across multiple
//! environments including native Rust, WASM-browser and Solana OS targets.
//!
//! When you application is built in the native application environment
//! macros such as `log_info!()` will invoke `println!()`, in WASM they will
//! invoke `console.log()` and under Solana they will invoke `sol_log()`
//! (used by `msg!()` macro)
//!
//! `workflow-log` macros operate the same way as regular functions such as
//! `println!()`
//!
//! The following core macros are available:
//! - `log_trace!()`
//! - `log_debug!()`
//! - `log_info!()`
//! - `log_warn()`
//! - `log_error!()`
//!
//! # Redirecting log output
//!
//! This crate allows you to configure a log sink that will receive
//! all log messages from your application.  This is useful to route log messages
//! to an external receiver or, for example, store logs to a file.
//!
//! Log sink can be installed using [`workflow_log::pipe`] function and supplying
//! it with an Arc of the [`workflow_log::Sink`] trait.  The trait function
//! [`workflow_log::Sink::write`] should return `true` to indicate the the text
//! should be outputed to the console, or `false` to prevent further output
//! (i.e. to consume the log text)
//!
//! ## Example:
//!
//! ```
//! use workflow_log::*;
//! use std::sync::Arc;
//!
//! pub struct MyStruct;
//! impl Sink for MyStruct {
//!     fn write(&self, target: Option<&str>, level:Level, args : &std::fmt::Arguments<'_>) -> bool {
//!         
//!         println!("target: {target:?}");
//!         println!("level: {level:?}");
//!         println!("args: {args:?}");
//!
//!         // return true to continue output
//!         // return false to prevent further output
//!         true
//!     }
//! }
//!
//! let my_struct = Arc::new(MyStruct{});
//! workflow_log::pipe(Some(my_struct));
//! log_trace!("test msg");
//! ```
//!
//! To can disable the sink by supplying [`Option::None`] to [`workflow_log::pipe`].  
//!

extern crate self as workflow_log;

mod log;
pub use self::log::*;

mod console;
pub use self::console::*;

pub mod levels;

pub mod prelude {
    pub use super::console::*;
    pub use super::levels::*;
    pub use super::log::*;
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::sync::Arc;

    #[test]
    fn log_sink_test() {
        pub struct MyStruct;
        impl Sink for MyStruct {
            fn write(
                &self,
                target: Option<&str>,
                level: Level,
                args: &std::fmt::Arguments<'_>,
            ) -> bool {
                println!("target: {target:?}");
                println!("level: {level:?}");
                println!("args: {args:?}");
                true
            }
        }

        let my_struct = Arc::new(MyStruct {});
        workflow_log::pipe(Some(my_struct));
        log_trace!("test msg");
    }
}
