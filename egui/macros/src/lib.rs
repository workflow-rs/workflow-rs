//! Procedural macros supporting the `workflow-egui` framework.
//!
//! Provides the [`register_modules`] macro, which generates the module
//! wiring and a factory function that instantiates a registry of UI modules.

use proc_macro::TokenStream;
mod register;

/// Generates `pub mod` declarations and re-exports for the listed module
/// types and emits a factory function (named by the first argument) that
/// constructs an `AHashMap<TypeId, Module>` of instantiated modules.
///
/// Usage: `register_modules!(<function name>, [<array of module paths>])`.
#[proc_macro]
pub fn register_modules(input: TokenStream) -> TokenStream {
    register::register_modules(input)
}
