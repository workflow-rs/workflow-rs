use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    ExprClosure, Result,
};

pub struct Method {
    result: TokenStream,
}

impl Parse for Method {
    fn parse(input: ParseStream) -> Result<Self> {
        let result = match input.parse::<ExprClosure>() {
            Ok(closure) => {
                let inputs = closure.inputs;
                let body = closure.body;
                quote! {
                    |#inputs|{
                        Box::pin(#body)
                    }
                }
            }
            Err(_) => {
                let ts = input.cursor().token_stream();
                ts
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

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.result.to_tokens(tokens);
    }
}
