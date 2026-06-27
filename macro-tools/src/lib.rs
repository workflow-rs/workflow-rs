//! Macro utilities used by macros in the [workflow-rs](https://github.com/workflow-rs/workflow-rs) project

/// Parsing helpers for procedural-macro attribute arguments.
#[cfg(not(target_arch = "wasm32"))]
pub mod attributes;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Error;

/// Produces a `compile_error!` token stream spanned over `tokens`,
/// reporting `message` at the location of the offending tokens.
pub fn parse_error<T: ToTokens>(tokens: T, message: &str) -> TokenStream {
    Error::new_spanned(tokens, message).to_compile_error()
}
