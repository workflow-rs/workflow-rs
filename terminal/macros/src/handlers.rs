use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::convert::Into;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Colon2,
    DeriveInput, Error, Expr, ExprLit, ExprPath, Lit, LitStr, Meta, NestedMeta, Path, PathSegment,
    Result, Token,
};

#[derive(Debug)]
struct DeclareHandler {
    type_expr: Expr,
    verb: LitStr,
    help: LitStr,
}

impl Parse for DeclareHandler {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input);
        if parsed.is_err() {
            return Err(Error::new(
                Span::call_site(),
                "usage: declare_handler!(<type>, [<verb>,] <help>)".to_string(),
            ));
        }
        let parsed = parsed.unwrap();
        if !(parsed.len() == 2 || parsed.len() == 3) {
            return Err(Error::new_spanned(
                parsed,
                "usage: declare_handler!(<type>, [<verb>,] <help>)".to_string(),
            ));
        }

        let mut iter = parsed.iter();

        let type_expr = iter.next().unwrap().clone();

        let (type_ident, type_expr) = match &type_expr {
            Expr::Path(expr_path) => {
                if let Some(segment) = expr_path.path.segments.last() {
                    (segment.ident.clone(), type_expr.clone())
                } else {
                    return Err(Error::new_spanned(
                        parsed,
                        "usage: declare_handler!(<type>, [<verb>,] <help>)".to_string(),
                    ));
                }
            }
            _ => {
                return Err(Error::new_spanned(
                    parsed,
                    "usage: declare_handler!(<type>, [<verb>,] <help>)".to_string(),
                ));
            }
        };

        let verb = if parsed.len() == 2 {
            let type_expr_ts = quote! { #type_ident };
            let s = type_expr_ts.to_string().to_case(Case::Kebab);
            LitStr::new(s.as_str(), Span::call_site())
        } else {
            let expr = iter.next().unwrap().clone();
            let verb_expr = match &expr {
                Expr::Lit(lit) => lit,
                _ => {
                    return Err(Error::new_spanned(
                        expr,
                        "usage: declare_handler!(<type>, [<verb>,] <help>)".to_string(),
                    ));
                }
            };

            let type_expr_ts = quote! { #verb_expr };
            let s = type_expr_ts.to_string().to_case(Case::Kebab);
            LitStr::new(s.as_str(), Span::call_site())
        };

        let expr = iter.next().unwrap().clone();
        let help_expr = match &expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit), ..
            }) => lit,
            _ => {
                return Err(Error::new_spanned(
                    expr,
                    "usage: declare_handler!(<type>, [<verb>,] <help>)".to_string(),
                ));
            }
        };

        let handlers = DeclareHandler {
            // type_ident,
            type_expr,
            verb,
            help: help_expr.clone(),
        };
        Ok(handlers)
    }
}

pub fn declare_handler(input: TokenStream) -> TokenStream {
    let handler = parse_macro_input!(input as DeclareHandler);
    render(handler)
}

pub fn declare_handler_derive(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let type_ident = &ast.ident;
    let type_expr_ts = quote! { #type_ident };
    let type_expr = ident_to_expr(type_ident.clone());

    let verb = get_attribute(&mut ast, "verb").unwrap_or_else(|| {
        let s = type_expr_ts.to_string().to_case(Case::Kebab);
        LitStr::new(s.as_str(), Span::call_site())
    });

    let help =
        get_attribute(&mut ast, "help").unwrap_or_else(|| LitStr::new("", Span::call_site()));

    let handler = DeclareHandler {
        type_expr,
        verb,
        help,
    };

    render(handler)
}

fn render(handler: DeclareHandler) -> TokenStream {
    let DeclareHandler {
        type_expr,
        verb,
        help,
        ..
    } = handler;

    quote! {

        #[async_trait::async_trait]
        impl workflow_terminal::cli::Handler for #type_expr {

            fn verb(&self, _ctx: &Arc<dyn workflow_terminal::cli::Context>) -> Option<&'static str> {
                Some(#verb)
            }

            fn help(&self, _ctx: &Arc<dyn workflow_terminal::cli::Context>) -> &'static str {
                #help
            }

            async fn handle(self : Arc<Self>, ctx: &Arc<dyn workflow_terminal::cli::Context>, argv : Vec<String>, cmd: &str) -> workflow_terminal::cli::Result<()> {
                self.main(ctx,argv,cmd).await.map_err(|e|e.to_string().into())
            }
        }

    }.into()
}

fn ident_to_expr(ident: Ident) -> Expr {
    let mut segments = Punctuated::<PathSegment, Colon2>::default();
    segments.push_value(PathSegment::from(ident));

    let path = Path {
        leading_colon: None,
        segments,
    };

    Expr::Path(ExprPath {
        attrs: vec![],
        qself: None,
        path,
    })
}

fn get_attribute(ast: &mut DeriveInput, name: &str) -> Option<LitStr> {
    let attr = ast.attrs.iter().enumerate().find_map(|(i, attr)| {
        attr.parse_meta().ok().and_then(|meta| {
            if meta.path().is_ident(name) {
                match meta {
                    Meta::List(meta_list) => {
                        if let Some(NestedMeta::Lit(Lit::Str(lit_str))) = meta_list.nested.first() {
                            Some((i, lit_str.clone()))
                        } else {
                            None
                        }
                    }
                    Meta::NameValue(name_value) => {
                        if let Lit::Str(lit_str) = name_value.lit {
                            Some((i, lit_str))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
    });

    if let Some((index, attr)) = attr {
        ast.attrs.remove(index);
        Some(attr)
    } else {
        None
    }
}
