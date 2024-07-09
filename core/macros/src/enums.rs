use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenTree};
use quote::{quote, ToTokens};
use std::convert::Into;
use syn::{parse_macro_input, DeriveInput};
use syn::{Error, Ident, Lit, LitStr, Meta, NestedMeta, Variant};
use workflow_macro_tools::attributes::*;

#[derive(Debug)]
struct Enum {
    pub args: Args,
    pub docs: Vec<Literal>,
    variant: Variant,
}

impl ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.variant.to_tokens(tokens);
    }
}

pub fn macro_handler(item: TokenStream) -> TokenStream {
    let enum_decl_ast = item;
    let mut ast = parse_macro_input!(enum_decl_ast as DeriveInput);

    let caption =
        get_attribute(&mut ast, "caption").unwrap_or_else(|| LitStr::new("", Span::call_site()));

    let _enum_attrs = &ast.attrs;

    let enum_name = &ast.ident;
    let _enum_params = &ast.generics;

    let enum_fields = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = ast.data {
        variants
    } else {
        return Error::new_spanned(
            enum_name,
            "#[derive(Describe)] macro only supports enums".to_string(),
        )
        .to_compile_error()
        .into();
    };

    let mut enums: Vec<Enum> = Vec::new();
    for variant in enum_fields.iter() {
        let name: String = variant.to_token_stream().to_string();
        let attrs: Vec<_> = variant
            .attrs
            .iter()
            .filter(|attr| attr.path.is_ident("descr") || attr.path.is_ident("describe"))
            .collect();
        if attrs.len() > 1 {
            return Error::new_spanned(
                enum_name,
                format!(
                    "#[describe]: more than one #[describe()] attributes while processing {name}"
                ),
            )
            .to_compile_error()
            .into();
        }

        let args = match attrs.first() {
            Some(attr) => match get_attributes(attr) {
                Some(attr) => attr,
                _ => Args::new(),
            },
            _ => Args::new(),
        };

        let mut docs = Vec::new();
        for attr in variant.attrs.iter() {
            let path_seg = attr.path.segments.last();
            if path_seg.is_none() {
                continue;
            }
            let segment = path_seg.unwrap();
            if segment.ident == "doc" {
                let mut tokens = attr.tokens.clone().into_iter();

                if let Some(TokenTree::Punct(_punct)) = tokens.next() {
                    if let Some(TokenTree::Literal(lit)) = tokens.next() {
                        docs.push(lit.clone());
                    }
                }
            }
        }

        let enum_instance = Enum {
            docs,
            args,
            variant: variant.clone(),
        };

        enums.push(enum_instance);
    }

    let entries: Vec<Ident> = enums.iter().map(|e| e.variant.ident.clone()).collect();

    let strings: Vec<String> = entries.iter().map(|ident| ident.to_string()).collect();
    let strings_ns: Vec<String> = entries
        .iter()
        .map(|ident| format!("{enum_name}::{ident}"))
        .collect();

    let mut descr: Vec<String> = Vec::new();
    let mut docs: Vec<String> = Vec::new();
    for e in enums.iter() {
        let have_key = e.args.has("default");
        if !have_key {
            descr.push(format!("{}", e.variant.ident.clone()));
        } else if let Some(info) = e.args.get("default").unwrap() {
            descr.push(info.to_token_stream().to_string().replace('\"', ""));
        } else {
            descr.push(format!("{}", e.variant.ident.clone()));
        }

        if e.docs.is_empty() {
            docs.push("".to_string());
        } else {
            let doc = e
                .docs
                .iter()
                .map(|doc| doc.to_token_stream().to_string())
                .collect::<Vec<String>>()
                .join(" ");
            let collapse_spaces_regex = regex::Regex::new(r"\s+").unwrap();
            let wrappers = regex::Regex::new(r#"(^\"|\"$)"#).unwrap();
            let doc = wrappers
                .replace_all(&doc, "")
                .replace("\\\"", "\"")
                .replace("\\'", "'");
            //let single_quotes = regex::Regex::new(r###"(^\"|\"$)"###).unwrap();
            // .trim()
            // .to_string();
            let doc = collapse_spaces_regex
                .replace_all(&doc, " ")
                .trim()
                .to_string();

            docs.push(doc);
        }
        // docs.push
    }

    #[cfg(target_os = "solana")]
    let enum_impl = quote! {};

    #[cfg(not(target_os = "solana"))]
    let enum_impl = quote! {

        impl #enum_name {

            pub fn caption() -> &'static str {
                #caption
            }

            pub fn iter() -> impl Iterator<Item = &'static Self> {
                [#( #enum_name::#entries ),*].iter()
            }

            pub fn into_iter() -> impl Iterator<Item = Self> {
                [#( #enum_name::#entries ),*].iter().cloned()
            }

            pub fn as_str(&self)->&'static str{
                match self {
                    #( #enum_name::#entries => { #strings.into() }),*
                }
            }

            pub fn as_str_ns(&self)->&'static str{
                match self {
                    #( #enum_name::#entries => { #strings_ns.into() }),*
                }
            }

            pub fn from_str(str:&str)->Option<#enum_name>{
                match str {
                    #( #strings => { Some(#enum_name::#entries) }),*
                    _ => None
                }
            }

            pub fn from_str_ns(str:&str)->Option<#enum_name>{
                match str {
                    #( #strings_ns => { Some(#enum_name::#entries) }),*
                    _ => None
                }
            }

            pub fn describe(&self) -> &'static str {
                match self {
                    #( #enum_name::#entries => { #descr.into() }),*
                }
            }

            pub fn rustdoc(&self) -> &'static str {
                match self {
                    #( #enum_name::#entries => { #docs.into() }),*
                }
            }
        }

        impl workflow_core::enums::Describe for #enum_name {

            fn caption() -> &'static str {
                #caption
            }

            fn iter() -> impl Iterator<Item = &'static Self> {
                #enum_name::iter()
            }

            fn into_iter() -> impl Iterator<Item = Self> {
                #enum_name::into_iter()
            }

            fn describe(&self) -> &'static str {
                self.describe()
            }

            fn rustdoc(&self) -> &'static str {
                self.rustdoc()
            }

            fn as_str(&self) -> &'static str {
                self.as_str()
            }

            fn as_str_ns(&self)->&'static str{
                match self {
                    #( #enum_name::#entries => { #strings_ns.into() }),*
                }
            }

            fn from_str(str:&str)->Option<Self>{
                #enum_name::from_str(str)
            }

            fn from_str_ns(str:&str)->Option<Self>{
                match str {
                    #( #strings_ns => { Some(#enum_name::#entries) }),*
                    _ => None
                }
            }
        }

    };

    quote! {
        #enum_impl
    }
    .into()
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
