//! Macros to obtain Rust references from within the WASM ABI.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Expr, Result, Token,
};

#[derive(Debug)]
struct RefFromWasmAbiArgs {
    class: Expr,
    jsvalue: Expr,
}

impl Parse for RefFromWasmAbiArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() != 2 {
            return Err(Error::new_spanned(
                parsed,
                "usage: ref_from_abi!(Type, JsValue)".to_string(),
            ));
        }

        let mut iter = parsed.iter();
        let class = iter.next().unwrap().clone();
        let jsvalue = iter.next().unwrap().clone();

        Ok(RefFromWasmAbiArgs { class, jsvalue })
    }
}

impl ToTokens for RefFromWasmAbiArgs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let RefFromWasmAbiArgs { class, jsvalue } = self;
        let name = class.to_token_stream().to_string();
        quote! {
            workflow_wasm::abi::ref_from_abi_safe::<#class>(#name, #jsvalue)
        }
        .to_tokens(tokens);
    }
}

pub fn ref_from_abi(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as RefFromWasmAbiArgs);
    let ts = args.to_token_stream();
    // println!("MACRO: {}", ts.to_string());
    ts.into()
}

#[derive(Debug)]
struct RefFromWasmAbiAsOptionArgs {
    inner: RefFromWasmAbiArgs,
}
impl Parse for RefFromWasmAbiAsOptionArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner = RefFromWasmAbiArgs::parse(input)?;
        Ok(Self { inner })
    }
}
impl ToTokens for RefFromWasmAbiAsOptionArgs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let RefFromWasmAbiArgs { class, jsvalue } = &self.inner;
        let name = class.to_token_stream().to_string();
        quote! {
            workflow_wasm::abi::ref_from_abi_safe_as_option::<#class>(#name, #jsvalue)
        }
        .to_tokens(tokens);
    }
}

pub fn ref_from_abi_as_option(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as RefFromWasmAbiAsOptionArgs);
    let ts = args.to_token_stream();
    // println!("MACRO: {}", ts.to_string());
    ts.into()
}
