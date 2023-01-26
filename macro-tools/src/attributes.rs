use proc_macro2::{Group, Ident, Span};
use std::convert::Into;
// use proc_macro2::{TokenStream as TokenStream2};
use crate::parse_error;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};
use syn::{Attribute, Error, Lit, LitBool};

#[derive(Debug, Clone)]
pub struct IdentWraper(proc_macro2::Ident);

impl Parse for IdentWraper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.step(|cursor| {
            if let Some((ident, rest)) = cursor.ident() {
                let wraper = IdentWraper(proc_macro2::Ident::new(&ident.to_string(), ident.span()));
                return Ok((wraper, rest));
            }
            Err(cursor.error("expected identifier"))
        })
    }
}

#[derive(Debug, Clone, parse_variants::Parse)]
pub enum Item {
    Identifier(syn::Ident),
    IdentifierWraper(IdentWraper),
    Literal(syn::LitInt),
    String(syn::LitStr),
}

#[derive(Debug, Clone, parse_variants::Parse)]
pub enum EvaluationValue {
    Group(proc_macro2::Group),
    Integer(syn::LitInt),
    String(syn::LitStr),
}

#[derive(Debug, Clone, parse_variants::Parse)]
pub enum AssignmentValue {
    String(syn::LitStr),
    Integer(syn::LitInt),
    Boolean(syn::LitBool),
    Group(proc_macro2::Group),
}

#[derive(Debug, Clone)]
pub enum Value {
    EvaluationValue(EvaluationValue),
    AssignmentValue(AssignmentValue),
}

impl Value {
    pub fn to_token_stream(&self) -> TokenStream {
        match self {
            Value::EvaluationValue(ev) => match ev {
                EvaluationValue::Integer(lit_int) => lit_int.to_token_stream(),
                EvaluationValue::String(lit_str) => lit_str.to_token_stream(),
                EvaluationValue::Group(group) => group.stream(),
            },
            Value::AssignmentValue(av) => match av {
                AssignmentValue::String(lit_str) => lit_str.to_token_stream(),
                AssignmentValue::Integer(lit_int) => lit_int.to_token_stream(),
                AssignmentValue::Boolean(lit_bool) => lit_bool.to_token_stream(),
                AssignmentValue::Group(group) => group.stream(),
            },
        }
    }
}

#[derive(Debug)]
pub struct Args {
    pub map: HashMap<Ident, Option<Value>>,
}

impl Default for Args {
    fn default() -> Self {
        Args::new()
    }
}

impl Args {
    pub fn new() -> Args {
        Args {
            map: HashMap::new(),
        }
    }

    pub fn has(&self, ident: &str) -> bool {
        let ident = Ident::new(ident, Span::call_site());
        self.map.contains_key(&ident)
    }

    pub fn get(&self, ident: &str) -> Option<&Option<Value>> {
        let ident = Ident::new(ident, Span::call_site());
        self.map.get(&ident)
    }

    pub fn get_value_or<T: ToTokens>(
        &self,
        ident: &str,
        field: T,
        msg: &str,
    ) -> Result<Option<Value>, TokenStream> {
        let v = self.get(ident);
        match v {
            None => Ok(None),
            Some(v) => match v {
                Some(v) => Ok(Some(v.clone())),
                None => Err(parse_error(field, msg)),
            },
        }
    }

    pub fn to_string_kv(&self) -> Vec<(String, String)> {
        let mut list: Vec<(String, String)> = Vec::new();
        for (k, v) in self.map.iter() {
            let value = match v {
                Some(value) => {
                    let v = value.to_token_stream();
                    let expr: Expr = syn::parse(v.into()).unwrap();
                    match &expr {
                        Expr::Lit(expr_lit) => match &expr_lit.lit {
                            Lit::Str(lit_str) => lit_str.value(),
                            _ => expr.to_token_stream().to_string(),
                        },
                        _ => expr.to_token_stream().to_string(),
                    }
                }
                None => "".to_string(),
            };
            list.push((k.to_string(), value));
        }

        list
    }

    pub fn allow(&self, list: &[&str]) -> syn::Result<()> {
        for (ident, _) in self.map.iter() {
            let name = ident.to_string();
            if !list.contains(&name.as_str()) {
                return Err(Error::new_spanned(
                    ident,
                    format!(
                        "unsupported attribute: {}, supported attributes are {}",
                        name,
                        list.join(", ")
                    ),
                ));
            }
        }

        Ok(())
    }
}

fn advance_one_step(input: &ParseStream<'_>) {
    let _ = input.step(|cursor| {
        let rest = *cursor;
        if let Some((_tt, next)) = rest.token_tree() {
            Ok(((), next))
        } else {
            Ok(((), rest))
        }
    });
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut map: HashMap<Ident, Option<Value>> = HashMap::new();
        while !input.is_empty() {
            let token_result = input.parse::<Item>();
            if token_result.is_err() {
                advance_one_step(&input);
                if input.peek(Token![=]) {
                    let _: Token![=] = input.parse()?;
                }
            } else {
                let token = token_result.ok().unwrap();
                match token {
                    Item::Identifier(ident) | Item::IdentifierWraper(IdentWraper(ident)) => {
                        if input.peek(Token![,]) {
                            let _: Token![,] = input.parse()?;
                            map.insert(
                                ident,
                                Some(Value::AssignmentValue(AssignmentValue::Boolean(
                                    LitBool::new(true, Span::call_site()),
                                ))),
                            );
                        } else if input.peek(Token![=]) {
                            let _: Token![=] = input.parse()?;
                            let rvalue: AssignmentValue = input.parse()?;
                            map.insert(ident, Some(Value::AssignmentValue(rvalue)));
                        } else {
                            let group: Group = input.parse()?;
                            map.insert(
                                ident,
                                Some(Value::EvaluationValue(EvaluationValue::Group(group))),
                            );
                        }

                        if input.peek(Token![,]) {
                            let _: Token![,] = input.parse()?;
                        }
                    }
                    // Item::Literal(lit) => {
                    //     let title = Ident::new("title", Span::call_site());
                    //     if map.get(&title).is_none() {
                    //         map.insert(title, Some(Value::EvaluationValue(EvaluationValue::Integer(lit))));
                    //     }
                    // },
                    Item::String(lit_str) => {
                        let default = Ident::new("default", Span::call_site());
                        if map.get(&default).is_none() {
                            map.insert(
                                default,
                                Some(Value::EvaluationValue(EvaluationValue::String(lit_str))),
                            );
                        }
                    }
                    _ => {
                        println!("invalid attributes");
                        // TODO check error handling
                        let ident = Ident::new("", Span::call_site());
                        return Err(Error::new_spanned(ident, "invalid attributes".to_string()));
                    }
                }
            }
        }

        Ok(Self { map })
    }
}

pub fn get_attributes(attr: &Attribute) -> Option<Args> {
    let attributes: Option<Args> = attr.parse_args().ok();
    attributes
}
