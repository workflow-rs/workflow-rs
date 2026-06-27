//! Procedural macros for the `workflow-terminal` CLI framework, providing
//! declarative generation of command [`Handler`] implementations and bulk
//! handler registration.
use proc_macro::TokenStream;
mod handlers;
mod register;

/// Implements the `workflow_terminal::cli::Handler` trait for a given type from
/// `declare_handler!(<type>, [<verb>,] <help>)`, deriving the command verb from
/// the type name (kebab-cased) when one is not supplied.
#[proc_macro]
pub fn declare_handler(input: TokenStream) -> TokenStream {
    handlers::declare_handler(input)
}

/// Derives a `workflow_terminal::cli::Handler` implementation for the annotated
/// type, taking the command verb and `help` text from `#[verb(..)]` and
/// `#[help(..)]` attributes and defaulting the verb to the kebab-cased type name.
#[proc_macro_derive(Handler, attributes(help))]
pub fn declare_handler_derive(input: TokenStream) -> TokenStream {
    handlers::declare_handler_derive(input)
}

/// Registers a list of command handlers with a target from
/// `register_handlers!(<context>, <target>, [<module names>])`, instantiating
/// each handler and attaching it to the target's command tree.
#[proc_macro]
pub fn register_handlers(input: TokenStream) -> TokenStream {
    register::register_handlers(input)
}
