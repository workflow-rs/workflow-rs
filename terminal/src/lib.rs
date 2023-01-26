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
//! [https://github.com/workflow-rs/workflow-terminal-example](https://github.com/workflow-rs/workflow-terminal-example)
//!
//! Loading in both native and WASM-browser application environment:
//! ```rust
//! struct ExampleCli;
//! #[async_trait]
//! impl Cli for ExampleCli { ... }
//! ...
//! let cli = Arc::new(ExampleCli::new());
//! let term = Arc::new(Terminal::try_new(cli.clone(),"$ ")?);
//! term.init().await?;
//! term.writeln("Terminal example (type 'help' for list of commands)");
//! term.run().await?;
//! ```
//!

pub mod clear;
pub mod cli;
pub mod cursor;
pub mod error;
pub mod keys;
pub mod result;
pub mod terminal;

pub use cli::Cli;
pub use result::Result;
pub use terminal::parse;
pub use terminal::Options;
pub use terminal::Terminal;

#[cfg(target_arch = "wasm32")]
pub use terminal::{Theme, ThemeOption};
