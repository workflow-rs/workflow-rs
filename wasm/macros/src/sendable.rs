use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::convert::Into;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Expr, ExprArray, Result, Token,
};

#[derive(Debug)]
struct SendableTypes {
    types: ExprArray,
}

impl Parse for SendableTypes {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() != 1 {
            return Err(Error::new_spanned(
                parsed,
                "usage: build_wrpc_client_interface!(interface, RpcApiOps,[getInfo, ..])"
                    .to_string(),
            ));
        }

        let mut iter = parsed.iter();
        let types = iter.next().unwrap().clone();
        let types = match types {
            Expr::Array(array) => array,
            _ => {
                return Err(Error::new_spanned(
                    types,
                    "the argument must be an array of types".to_string(),
                ));
            }
        };

        for ph in types.elems.iter() {
            match ph {
                Expr::Path(_exp_path) => {}
                _ => {
                    return Err(Error::new_spanned(
                        ph,
                        "handlers should contain an array of types".to_string(),
                    ));
                }
            }
        }

        Ok(SendableTypes { types })
    }
}

impl ToTokens for SendableTypes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut targets = Vec::new();
        for js_type in self.types.elems.iter() {
            targets.push(quote! {

                pub struct #js_type(pub non_sendable::#js_type);
                unsafe impl Send for #js_type {}

                impl std::ops::Deref for #js_type {
                    type Target = non_sendable::#js_type;
                    fn deref(&self) -> &non_sendable::#js_type {
                        &self.0
                    }
                }

                // impl std::ops::Deref for #js_type {
                //     type Target = non_sendable::JsValue;
                //     fn deref(&self) -> &non_sendable::JsValue {
                //         &self.0
                //     }
                // }

                impl AsRef<non_sendable::#js_type> for #js_type {
                    fn as_ref(&self) -> &non_sendable::#js_type {
                        &self.0
                    }
                }

                impl AsRef<non_sendable::JsValue> for #js_type {
                    fn as_ref(&self) -> &non_sendable::JsValue {
                        &self.0
                    }
                }

                impl From<#js_type> for non_sendable::#js_type {
                    fn from(value: #js_type) -> Self {
                        value.0
                    }
                }

                impl From<#js_type> for non_sendable::JsValue {
                    fn from(value: #js_type) -> Self {
                        value.0.into()
                    }
                }

            });
        }

        quote! {
            #(#targets)*
        }
        .to_tokens(tokens);
    }
}

pub fn build_sendable_types(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let types = parse_macro_input!(input as SendableTypes);
    let ts = types.to_token_stream();
    // println!("MACRO: {}", ts.to_string());
    ts.into()
}
