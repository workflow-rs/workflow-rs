use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    token::Comma,
    Expr, ExprClosure, Result,
};

pub struct Task {
    result: TokenStream,
}

impl Parse for Task {
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

impl ToTokens for Task {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.result.to_tokens(tokens);
    }
}

pub struct SetTask {
    result: TokenStream,
}

impl Parse for SetTask {
    fn parse(input: ParseStream) -> Result<Self> {
        let source = input.parse::<Expr>()?;
        input.parse::<Comma>()?;
        let task = Task::parse(input)?;
        let result = quote! {
            #source.set_task_fn(#task)
        };
        Ok(Self { result })
    }
}

impl ToTokens for SetTask {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.result.to_tokens(tokens);
    }
}
