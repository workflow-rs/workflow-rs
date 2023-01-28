use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use sha2::{Digest, Sha256};
use std::convert::Into;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Expr, Result, Token,
};

#[derive(Debug)]
struct Seal {
    hash: String, //ExprLit,
    hash_expr: Expr,
    content: TokenStream2,
}

impl Parse for Seal {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input);
        if parsed.is_err() {
            return Err(Error::new(
                Span::call_site(),
                "usage: seal!(<seal id>, { <code> })".to_string(),
            ));
        }
        let parsed = parsed.unwrap();
        if parsed.len() != 2 {
            return Err(Error::new_spanned(
                parsed,
                "usage: seal!(<seal id>, { <code> })".to_string(),
            ));
        }

        let mut iter = parsed.iter();

        let hash_expr = iter.next().unwrap().clone();
        let hash = match &hash_expr {
            Expr::Lit(lit) => lit,
            _ => {
                return Err(Error::new_spanned(
                    hash_expr,
                    "the first argument should be the seal number".to_string(),
                ));
            }
        };

        let content_expr = iter.next();
        if content_expr.is_none() {
            return Err(Error::new_spanned(
                parsed,
                "usage: seal!(<seal id>, { <code> })".to_string(),
            ));
        }
        let content_expr = content_expr.unwrap().clone();
        let expr_block = match &content_expr {
            Expr::Block(expr_block) => expr_block, // .block,
            _ => {
                return Err(Error::new_spanned(
                    content_expr,
                    "the second argument must be code block { <code> }".to_string(),
                ));
            }
        };

        let stmts = &expr_block.block.stmts;
        let content = quote! {
            #(#stmts)*
        };

        let hash = quote! {#hash};
        let handlers = Seal {
            hash: hash.to_string().to_ascii_lowercase(),
            hash_expr,
            content,
        };
        Ok(handlers)
    }
}

pub fn seal(input: TokenStream) -> TokenStream {
    let seal = parse_macro_input!(input as Seal);
    let content = seal.content;
    let content_ts = quote! { #content };
    let content_str = content_ts.to_string();

    let mut sha256 = Sha256::new();
    sha256.update(content_str);
    let hash_nc: String = format!("0x{:X}", sha256.finalize());
    let hash_str = hash_nc.to_ascii_lowercase();
    let hash: String = hash_str[0..6].into();

    if seal.hash != hash {
        return Error::new_spanned(
            seal.hash_expr,
            format!("Seal changed - was: {} now: {}", seal.hash, hash),
        )
        .to_compile_error()
        .into();
    }

    let output = quote! {
        #content
    };

    output.into()
}
