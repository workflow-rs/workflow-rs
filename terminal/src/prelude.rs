pub use crate::{
    cli,
    cli::get_handler_help,
    cli::{declare_handler, register_handlers},
    parse, terrorln, tpara, tprintln, twarnln, Cli, Context, CrLf, Handler, Options as TerminalOptions, Result as TerminalResult,
    TargetElement as TerminalTargetElement,
    terminal::Theme,
};
pub use std::ops::Deref;
