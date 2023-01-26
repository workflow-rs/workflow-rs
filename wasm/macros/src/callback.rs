use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    ExprClosure, Ident, Result,
};

pub struct Callback {
    result: TokenStream,
}

impl Parse for Callback {
    fn parse(input: ParseStream) -> Result<Self> {
        let result = match input.parse::<ExprClosure>() {
            Ok(closure) => {
                let len = closure.inputs.len();

                let fn_name = if len == 1 {
                    Ident::new("new", Span::call_site())
                } else {
                    Ident::new(&format!("new_with_args_{}", len), Span::call_site())
                };

                quote! {workflow_wasm::callback::Callback::#fn_name(#closure)}
            }
            Err(_) => {
                let ts = input.cursor().token_stream();
                quote! {workflow_wasm::callback::Callback::new(#ts)}
            }
        };

        //empty input
        while !input.is_empty() {
            input.step(|cursor| {
                let mut rest = *cursor;
                while let Some((_, next)) = rest.token_tree() {
                    rest = next
                }
                Ok(((), rest))
            })?;
        }

        Ok(Self { result })
    }
}

impl ToTokens for Callback {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.result.to_tokens(tokens);
    }
}
