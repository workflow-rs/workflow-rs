//! Macro utilities used by macros in the [workflow-rs](https://github.com/workflow-rs/workflow-rs) project

#[cfg(not(target_arch = "wasm32"))]
pub mod attributes;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Error;

pub fn parse_error<T: ToTokens>(tokens: T, message: &str) -> TokenStream {
    Error::new_spanned(tokens, message).to_compile_error()
}
