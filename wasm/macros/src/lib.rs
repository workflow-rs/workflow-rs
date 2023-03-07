use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::ToTokens;
use syn::parse_macro_input;
mod callback;
use callback::Callback;
mod derive_try_from_jsvalue;
mod ref_from_abi;

#[proc_macro]
#[proc_macro_error]
pub fn callback(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as Callback);
    let ts = result.to_token_stream();
    //println!("\n===========> Callback <===========\n{}\n", ts.to_string());
    ts.into()
}

#[proc_macro_derive(TryFromJsValue)]
pub fn derive_try_from_jsvalue(input: TokenStream) -> TokenStream {
    derive_try_from_jsvalue::derive_try_from_jsvalue(input)
}

/// Create a reference to a Rust object from a WASM ABI.
#[proc_macro]
pub fn ref_from_abi(input: TokenStream) -> TokenStream {
    ref_from_abi::ref_from_abi(input)
}

/// Create a Rust Option<object> from a WASM ABI.
#[proc_macro]
pub fn ref_from_abi_as_option(input: TokenStream) -> TokenStream {
    ref_from_abi::ref_from_abi_as_option(input)
}
