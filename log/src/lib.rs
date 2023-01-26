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
//! pub struct MyStruct;
//! impl Sink for MyStruct {
//!     fn write(&self, _level:Level, args : &std::fmt::Arguments<'_>) -> bool {
//!         // return true to continue output
//!         // return false to prevent further output
//!     }
//! }
//! ...
//! let my_struct = Arc::new(MyStruct{});
//! workflow_log::pipe(Some(my_struct));
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
