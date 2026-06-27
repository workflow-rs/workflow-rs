//! Procedural macros supporting the `workflow-wasm` crate, providing the
//! [`callback`] function-like macro for constructing JavaScript callbacks and
//! the [`CastFromJs`](derive_cast_from_js) derive for casting from JS values.

use proc_macro::TokenStream;
use proc_macro_error3::proc_macro_error;
use quote::ToTokens;
use syn::parse_macro_input;
mod callback;
use callback::Callback;
mod derive_cast_from_js;

/// Constructs a `workflow_wasm::callback::Callback` from a closure (or
/// expression), selecting the appropriate constructor based on the number of
/// closure arguments so the callback can be passed to JavaScript.
#[proc_macro]
#[proc_macro_error]
pub fn callback(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as Callback);
    let ts = result.to_token_stream();
    //println!("\n===========> Callback <===========\n{}\n", ts.to_string());
    ts.into()
}

/// Derives `CastFromJs` for a type, generating the implementation that safely
/// casts a `JsValue` reference back into the corresponding Rust type via
/// `workflow-wasm`'s conversion helpers.
#[proc_macro_derive(CastFromJs)]
pub fn derive_cast_from_js(input: TokenStream) -> TokenStream {
    derive_cast_from_js::derive_cast_from_js(input)
}
