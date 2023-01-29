use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
mod task;

#[proc_macro]
#[proc_macro_error]
pub fn task(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as task::Task);
    let ts = quote! {
        workflow_task::Task::new(#result)
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
