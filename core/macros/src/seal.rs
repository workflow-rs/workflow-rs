use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
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
        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() != 2 {
            return Err(Error::new_spanned(
                parsed,
                format!("usage: seal!(<seal id>, {{ <code> }})"),
            ));
        }

        let mut iter = parsed.iter();

        let hash_expr = iter.next().clone().unwrap().clone();
        let hash = match &hash_expr {
            Expr::Lit(lit) => lit,
            _ => {
                return Err(Error::new_spanned(
                    hash_expr,
                    format!("the first argument should be the seal number)"),
                ));
            }
        };

        let content_expr = iter.next().clone().unwrap().clone();
        let expr_block = match &content_expr {
            Expr::Block(expr_block) => expr_block, // .block,
            _ => {
                return Err(Error::new_spanned(
                    content_expr,
                    format!("the third argument must be an array of static functions"),
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
