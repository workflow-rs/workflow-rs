use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
mod enums;
mod seal;
mod task;

///
/// Attribute macro for automatic conversion of enums to their string representation
///
/// This macro works only with pure enums (it does not support enums that have
/// values represented as structs)
///
/// This macro implements the following methods:
///
/// ```rust
///     // returns a Vec of all enum permutations
///     fn list() -> Vec<enum>;
///     // returns the `rustdoc` description of the enum
///     fn descr(&self) -> &'static str;
///     // return the name of the value i.e. `Value`
///     fn as_str(&self) -> &'static str;
///     // return the the namespaced enum value i.e. `Enum::Value`
///     fn as_str_ns(&self)->&'static str;
///     // get enum value from the name i.e. `Value`
///     fn from_str(str:&str)->Option<enum>;
///     // get enum value from the namespaced value name i.e. `Enum::Value`
///     fn from_str_ns(str:&str)->Option<#enum_name>;
///```
///
// #[proc_macro_attribute]
#[proc_macro_derive(Describe, attributes(descr, describe))]
// pub fn describe_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
pub fn describe_enum(item: TokenStream) -> TokenStream {
    // enums::macro_handler(attr, item)
    enums::macro_handler(item)
}

#[proc_macro]
pub fn seal(input: TokenStream) -> TokenStream {
    seal::seal(input)
}

#[proc_macro]
#[proc_macro_error]
pub fn task(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as task::Task);
    let ts = quote! {
        workflow_core::task::Task::new(#result)
    };
    ts.into()
}

#[proc_macro]
#[proc_macro_error]
pub fn set_task(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as task::SetTask);
    let ts = result.to_token_stream();
    ts.into()
}
