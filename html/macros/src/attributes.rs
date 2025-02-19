use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use rand::Rng;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Block, Result, Token,
};
//use std::sync::Arc;

pub type AttributeName = Punctuated<Ident, Token![-]>;

pub trait AttributeNameString {
    fn to_string(&self) -> String;
    fn to_property_name(&self) -> Ident;
}

impl AttributeNameString for AttributeName {
    fn to_property_name(&self) -> Ident {
        Ident::new(&self.to_string().replace('-', "_"), Span::call_site())
    }
    fn to_string(&self) -> String {
        let mut items = self.iter().map(|a| a.to_string());
        let first = items.next().unwrap();
        items.fold(first, |a, b| format!("{a}-{b}"))
    }
}

pub struct Attributes<'a> {
    list: Vec<Attribute<'a>>,
}

impl Attributes<'_> {
    /*
    pub fn get_names(&self)->Vec<String>{
        let mut list = vec![];
        for attr in &self.list{
            list.push(attr.get_name())
        }
        list
    }
    */
    pub fn empty() -> Self {
        Self { list: vec![] }
    }
    pub fn to_properties(&self) -> (Vec<TokenStream>, Vec<TokenStream>) {
        let mut properties = vec![];
        let mut events = vec![];
        //let mut used = vec![];
        let mut append: bool;
        for attr in &self.list {
            append = true;
            let name_str = &attr.name.to_string();
            let name = &attr.name.to_property_name();
            let value = match attr.attr_type {
                AttributeType::Str => {
                    if attr.value.is_some() {
                        let value = attr.get_value();
                        quote!(:String::from(#value).into())
                    } else {
                        quote!(:String::from(#name).into())
                    }
                }
                AttributeType::String => {
                    if attr.value.is_some() {
                        let value = attr.get_value();
                        quote!(:&#value.into())
                    } else {
                        quote!(:&#name.into())
                    }
                }
                AttributeType::Event => {
                    append = false;
                    if attr.value.is_some() {
                        let value = attr.get_value();
                        events.push(quote!(
                            .on(#name_str, Box::new(move |target|{
                                #value
                                Ok(())
                            }))
                        ));
                    }
                    quote!()
                }
                _ => {
                    if attr.value.is_some() {
                        let value = attr.get_value();
                        quote!(:#value)
                    } else {
                        quote!()
                    }
                }
            };
            //used.push(name.to_string());
            if append {
                properties.push(quote!(
                    #name #value
                ));
            }
        }
        /*
        println!("used: {:?} , names:{:?}", used, names);
        for name in names.iter(){
            if !used.contains(name){
                let name_ident = Ident::new(name, Span::call_site());
                properties.push(quote!(
                    #name_ident: None
                ));
            }
        }
        */
        (properties, events)
    }
    pub fn to_token_stream(&self) -> (TokenStream, Vec<TokenStream>) {
        let mut attrs = vec![];
        let mut events = vec![];
        let mut ref_field = quote!(reff: None);
        for attr in &self.list {
            let name = attr.get_name();
            let value = attr.get_value();
            let mut append = true;
            let value = match attr.attr_type {
                AttributeType::Bool => {
                    quote! {workflow_html::AttributeValue::Bool(#value)}
                }
                AttributeType::Str => {
                    quote! {workflow_html::AttributeValue::Str(String::from(#value))}
                }
                AttributeType::String => {
                    quote! {workflow_html::AttributeValue::Str(String::from(#value))}
                }
                AttributeType::Ref => {
                    ref_field = quote! {reff: Some((String::from(#name), String::from(#value)))};
                    append = false;
                    quote!()
                }
                AttributeType::Event => {
                    append = false;
                    if attr.value.is_some() {
                        events.push(quote!(
                            .on(#name, Box::new(move |_event, _target|{
                                #value
                            }))
                        ));
                    }
                    quote!()
                }
            };
            if append {
                attrs.push(quote!(
                    map.insert(String::from(#name), #value);
                ));
            }
        }
        (
            quote! {
                #ref_field,
                attributes:{
                    let mut map = std::collections::BTreeMap::new();
                    #(#attrs)*
                    map
                }
            },
            events,
        )
    }
}

pub enum AttributeValue<'a> {
    Block(Block),
    Literal(Literal),
    Path(syn::punctuated::Punctuated<syn::PathSegment, Token!(::)>),
    _Str(&'a str),
}
pub enum AttributeType {
    Bool,
    Str,
    String,
    Ref,
    Event,
}
pub struct Attribute<'a> {
    pub name: AttributeName,
    pub attr_type: AttributeType,
    pub value: Option<AttributeValue<'a>>,
}

impl<'a> Attribute<'a> {
    pub fn new(
        name: AttributeName,
        attr_type: AttributeType,
        value: Option<AttributeValue<'a>>,
    ) -> Attribute<'a> {
        Self {
            name,
            attr_type,
            value,
        }
    }
    pub fn get_name(&self) -> String {
        let mut items = self.name.iter().map(|a| a.to_string());
        let first = items.next().unwrap();
        items.fold(first, |a, b| format!("{a}-{b}"))
    }

    pub fn get_value(&self) -> TokenStream {
        match &self.value {
            Some(value) => match value {
                AttributeValue::Block(v) => {
                    if v.stmts.len() > 1 {
                        v.to_token_stream()
                    } else {
                        (&v.stmts[0]).into_token_stream()
                    }
                }
                AttributeValue::Literal(v) => quote!(#v).into_token_stream(),
                AttributeValue::_Str(v) => quote!(#v).into_token_stream(),
                AttributeValue::Path(v) => quote!(#v).into_token_stream(),
            },
            None => match self.attr_type {
                AttributeType::Ref => {
                    let mut rng = rand::thread_rng();
                    let code = format!("ref_{}", rng.gen::<u32>());
                    quote!(#code)
                }
                _ => self.name.to_token_stream(),
            },
        }
    }
}

impl Parse for Attribute<'_> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attr_type = AttributeType::Str;
        if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            attr_type = AttributeType::Bool;
        } else if input.peek(Token![&]) {
            input.parse::<Token![&]>()?;
            attr_type = AttributeType::String;
        } else if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            attr_type = AttributeType::Ref;
        } else if input.peek(Token![!]) {
            input.parse::<Token![!]>()?;
            attr_type = AttributeType::Event;
        }

        let name = AttributeName::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let value;
            let parser = Punctuated::<syn::PathSegment, Token![::]>::parse_separated_nonempty;
            if input.peek(syn::token::Brace) {
                value = AttributeValue::Block(input.parse::<Block>()?);
            } else if input.peek(syn::Lit) {
                //value = AttributeValue::Str("");
                //println!("input: {:#?}", input);
                value = AttributeValue::Literal(input.parse::<Literal>()?);
            } else {
                value = AttributeValue::Path(parser(input)?); //AttributeValue::Literal(input.parse::<Literal>()?);
            }
            return Ok(Attribute::new(name, attr_type, Some(value)));
        }
        Ok(Attribute::new(name, attr_type, None))
    }
}

pub fn parse_attributes<'a>(input: ParseStream) -> Result<Attributes<'a>> {
    let mut list = vec![];
    //print!("parse_attributes: {:?}", input);
    while !(input.peek(Token![/]) || input.peek(Token![>])) {
        let attribute = input.parse::<Attribute>()?;
        list.push(attribute);
    }

    Ok(Attributes { list })
}
