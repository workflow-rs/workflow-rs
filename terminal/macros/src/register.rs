use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use std::convert::Into;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Expr, ExprArray, Result, Token,
};

#[derive(Debug)]
struct Handlers {
    ctx: Expr,
    target: Expr,
    handlers: ExprArray,
}

impl Parse for Handlers {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input);
        if parsed.is_err() {
            return Err(Error::new(
                Span::call_site(),
                "usage: regster_handlers!(<context>,<target>,<array of module names>)".to_string(),
            ));
        }
        let parsed = parsed.unwrap();
        if parsed.len() != 3 {
            return Err(Error::new_spanned(
                parsed,
                "usage: regster_handlers!(<context>,<target>,<array of module names>)".to_string(),
            ));
        }

        let mut iter = parsed.iter();
        let ctx = iter.next().unwrap().clone();
        let target = iter.next().unwrap().clone();
        let handlers = get_handlers(iter.next().unwrap().clone())?;

        Ok(Handlers {
            ctx,
            target,
            handlers,
        })
    }
}

fn get_handlers(handlers: Expr) -> Result<ExprArray> {
    let handlers = match handlers {
        Expr::Array(array) => array,
        _ => {
            return Err(Error::new_spanned(
                handlers,
                "the argument must be an array of enum variants".to_string(),
            ));
        }
    };

    for ph in handlers.elems.iter() {
        match ph {
            Expr::Path(_exp_path) => {}
            _ => {
                return Err(Error::new_spanned(
                    ph,
                    "handlers should contain enum variants".to_string(),
                ));
            }
        }
    }

    Ok(handlers)
}

pub fn register_handlers(input: TokenStream) -> TokenStream {
    let handler = parse_macro_input!(input as Handlers);
    render(handler)
}

fn render(handlers: Handlers) -> TokenStream {
    let Handlers {
        ctx,
        target,
        handlers,
    } = handlers;

    let mut targets = Vec::new();
    for handler in handlers.elems.into_iter() {
        let name = handler.to_token_stream().to_string();
        let module_name = handler;

        let type_name = Ident::new(&name.to_case(Case::UpperCamel), Span::call_site());
        targets.push(quote! {
            #target.register(&#ctx, #module_name::#type_name::default());
        });
    }

    quote! {
        #(#targets)*
    }
    .into()
}
