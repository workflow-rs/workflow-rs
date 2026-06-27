use proc_macro2::{Group, Ident, Span};
use std::convert::Into;
// use proc_macro2::{TokenStream as TokenStream2};
use crate::parse_error;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{Attribute, Error, Lit, LitBool};
use syn::{
    Expr, Token,
    parse::{Parse, ParseStream},
};

/// A newtype wrapper around an identifier that implements [`Parse`] to consume
/// an identifier from the input stream regardless of keyword status.
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

/// A single lexical item encountered while parsing attribute arguments.
#[derive(Debug, Clone, parse_variants::Parse)]
pub enum Item {
    /// A plain identifier, typically an argument name.
    Identifier(syn::Ident),
    /// An identifier captured through [`IdentWraper`].
    IdentifierWraper(IdentWraper),
    /// A bare integer literal.
    Literal(syn::LitInt),
    /// A bare string literal, treated as the `default` argument.
    String(syn::LitStr),
}

/// A value appearing in evaluation position, e.g. inside a delimited group.
#[derive(Debug, Clone, parse_variants::Parse)]
pub enum EvaluationValue {
    /// A delimited token group value.
    Group(proc_macro2::Group),
    /// An integer literal value.
    Integer(syn::LitInt),
    /// A string literal value.
    String(syn::LitStr),
}

/// A value appearing on the right-hand side of an `=` assignment.
#[derive(Debug, Clone, parse_variants::Parse)]
pub enum AssignmentValue {
    /// A string literal value.
    String(syn::LitStr),
    /// An integer literal value.
    Integer(syn::LitInt),
    /// A boolean literal value, also used for bare flag arguments.
    Boolean(syn::LitBool),
    /// A delimited token group value.
    Group(proc_macro2::Group),
}

/// The value associated with an argument, distinguishing values parsed in
/// evaluation position from those parsed on the right-hand side of an `=`.
#[derive(Debug, Clone)]
pub enum Value {
    /// A value supplied in evaluation position, e.g. `name(group)`.
    EvaluationValue(EvaluationValue),
    /// A value supplied via assignment, e.g. `name = value`.
    AssignmentValue(AssignmentValue),
}

impl Value {
    /// Renders the underlying literal or group back into its token stream.
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

/// A parsed set of macro attribute arguments, keyed by argument name.
#[derive(Debug)]
pub struct Args {
    /// Maps each argument identifier to its optional value (`None` when the
    /// argument was supplied as a bare flag without a value).
    pub map: HashMap<Ident, Option<Value>>,
}

impl Default for Args {
    fn default() -> Self {
        Args::new()
    }
}

impl Args {
    /// Creates an empty set of arguments.
    pub fn new() -> Args {
        Args {
            map: HashMap::new(),
        }
    }

    /// Returns `true` if an argument with the given name is present.
    pub fn has(&self, ident: &str) -> bool {
        let ident = Ident::new(ident, Span::call_site());
        self.map.contains_key(&ident)
    }

    /// Looks up an argument by name, returning its optional value, or `None`
    /// if the argument is not present.
    pub fn get(&self, ident: &str) -> Option<&Option<Value>> {
        let ident = Ident::new(ident, Span::call_site());
        self.map.get(&ident)
    }

    /// Returns the value of `ident`: `Ok(None)` if the argument is absent,
    /// `Ok(Some(_))` if it has a value, or an error spanned over `field` with
    /// `msg` if the argument is present but carries no value.
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

    /// Returns the arguments as a list of name/value string pairs, with
    /// string literals unquoted and valueless arguments mapped to an empty string.
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

    /// Validates that every argument name present is contained in `list`,
    /// returning an error spanned over the first unsupported argument.
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
        match rest.token_tree() {
            Some((_tt, next)) => Ok(((), next)),
            _ => Ok(((), rest)),
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
                        map.entry(default).or_insert(Some(Value::EvaluationValue(
                            EvaluationValue::String(lit_str),
                        )));
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

/// Parses the contents of the given attribute into [`Args`],
/// returning `None` if the attribute arguments fail to parse.
pub fn get_attributes(attr: &Attribute) -> Option<Args> {
    let attributes: Option<Args> = attr.parse_args().ok();
    attributes
}
