//! Procedural macros backing the `workflow-core` crate, providing the
//! `Describe` derive for enum string conversion, the
//! [`seal!`](macro@seal) macro for guarding code blocks against accidental
//! changes, and the [`call_async_no_send!`](macro@call_async_no_send) macro for
//! awaiting `!Send` futures across the worker dispatch boundary.

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

/// Wraps a code block with a content hash seal, taking a seal id and a code
/// block (`seal!(0x1234, { ... })`). The macro emits the block plus a `SEAL`
/// constant and fails to compile if the block's content no longer matches the
/// supplied seal id, drawing attention to changes in security-sensitive code.
#[proc_macro]
pub fn seal(input: TokenStream) -> TokenStream {
    seal::seal(input)
}

/// Awaits an `async` block or `await` expression whose future is not `Send` by
/// dispatching it onto a local task and relaying the result back through a
/// oneshot channel wrapped in `Sendable`.
/// Accepts either an async block or an await expression and propagates errors
/// with `?`.
#[proc_macro]
pub fn call_async_no_send(input: TokenStream) -> TokenStream {
    send::call_async_no_send(input)
}

// #[proc_macro_attribute]
// pub fn clean_attributes(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     attribute_cleaner::clean_attributes(_attr, item)
// }
