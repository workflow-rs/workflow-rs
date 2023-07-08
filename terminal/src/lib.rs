//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-terminal.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-terminal)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--terminal-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/workflow-terminal)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-terminal.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- native -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! [`workflow-terminal`] is a terminal shell that functions uniformly in native
//! Rust application command-line environment and in WASM-based browser environment.
//!
//! This is achieved by combining [`termion`](https://crates.io/crates/termion) and
//! [xterm.js](http://xtermjs.org) into a unified module and offering an intermediary
//! API that can interface with both libraries.
//!
//! You can initialize this crate from a regular bin project or a WASM project using
//! dedicated functions and provide a [`Cli`] trait implementing the command-line
//! interface that will receive input from the underlying terminal.
//!
//! Workflow Terminal example can be found at
//! [https://github.com/workflow-rs/workflow-terminal-examples](https://github.com/workflow-rs/workflow-terminal-examples)
//!
//! Loading in both native and WASM-browser application environment:
//! ```rust
//! use async_trait::async_trait;
//! use std::sync::Arc;
//! use workflow_terminal::Cli;
//! use workflow_terminal::Terminal;
//! use workflow_terminal::result::Result;
//!
//! struct ExampleCli;
//! #[async_trait]
//! impl Cli for ExampleCli {
//!     async fn digest(&self, _term: Arc<Terminal>, _cmd: String) -> Result<()>{
//!         Ok(())
//!     }
//!     async fn complete(&self, _term: Arc<Terminal>, _cmd: String) -> Result<Vec<String>>{
//!         Ok(vec![])
//!     }
//! }
//!
//! #[tokio::test]
//! # async fn test()->Result<()>{
//! let cli = Arc::new(ExampleCli{});
//! let term = Arc::new(Terminal::try_new(cli.clone(),"$ ")?);
//! term.init().await?;
//! term.writeln("Terminal example (type 'help' for list of commands)");
//! term.run().await?;
//! # Ok(())
//! # }
//!
//! ```
//!
//! Loading terminal in specific element
//! ```rust
//! use async_trait::async_trait;
//! use std::sync::Arc;
//! use workflow_terminal::Cli;
//! use workflow_terminal::Terminal;
//! use workflow_terminal::result::Result;
//! use workflow_terminal::{Options, TargetElement};
//!
//! #[derive(Clone)]
//! struct ExampleCli;
//! #[async_trait]
//! impl Cli for ExampleCli {
//!     async fn digest(&self, _term: Arc<Terminal>, _cmd: String) -> Result<()>{
//!         Ok(())
//!     }
//!     async fn complete(&self, _term: Arc<Terminal>, _cmd: String) -> Result<Vec<String>>{
//!         Ok(vec![])
//!     }
//! }
//!
//! # fn test(){
//!
//! let cli = Arc::new(ExampleCli{});
//! let options = Options::new()
//!     .with_prompt("$ ")
//!     .with_element(TargetElement::Id("terminal_container".to_string()));
//!     //.with_element(TargetElement::Element(element));
//!     //.with_element(TargetElement::Body);
//!     //.with_element(TargetElement::TagName("body".to_string()));
//! let term = Arc::new(Terminal::try_new_with_options(cli.clone(), options).unwrap());
//!
//! # }
//! ```
//!

extern crate self as workflow_terminal;

pub mod clear;
pub mod cli;
pub mod crlf;
pub mod cursor;
pub mod error;
pub mod keys;
pub mod macros;
pub mod result;
pub mod terminal;

pub use cli::{Cli, Context, Handler, HandlerCli};
pub use crlf::CrLf;
pub use macros::*;
pub use result::Result;
pub use terminal::parse;
pub use terminal::Options;
pub use terminal::TargetElement;
pub use terminal::Terminal;
pub use textwrap;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use terminal::{Theme, ThemeOption, xterm, bindings};
    } else {
        pub use terminal::{disable_raw_mode,init_panic_hook};
    }
}
