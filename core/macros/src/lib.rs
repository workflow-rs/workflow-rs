use proc_macro::TokenStream;
// mod attribute_cleaner;
mod enums;
mod seal;
mod send;

///
/// Attribute macro for automatic conversion of enums to their string representation
///
/// This macro works only with pure enums (it does not support enums that have
/// values represented as structs)
///
/// This macro implements the following methods:
///
/// ```ignore
/// // returns a Vec of all enum permutations
/// fn list() -> Vec<MyEnum>;
/// // returns the `rustdoc` description of the enum
/// fn descr(&self) -> &'static str;
/// // return the name of the value i.e. `Value`
/// fn as_str(&self) -> &'static str;
/// // return the the namespaced enum value i.e. `MyEnum::Value`
/// fn as_str_ns(&self)->&'static str;
/// // get enum value from the name i.e. `Value`
/// fn from_str(str:&str)->Option<MyEnum>;
/// // get enum value from the namespaced value name i.e. `MyEnum::Value`
/// fn from_str_ns(str:&str)->Option<#enum_name>;
/// ```
///
///
#[proc_macro_derive(Describe, attributes(caption, describe))]
pub fn describe_enum(item: TokenStream) -> TokenStream {
    enums::macro_handler(item)
}

#[proc_macro]
pub fn seal(input: TokenStream) -> TokenStream {
    seal::seal(input)
}

#[proc_macro]
pub fn call_async_no_send(input: TokenStream) -> TokenStream {
    send::call_async_no_send(input)
}

// #[proc_macro_attribute]
// pub fn clean_attributes(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     attribute_cleaner::clean_attributes(_attr, item)
// }
