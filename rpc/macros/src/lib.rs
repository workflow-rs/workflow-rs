use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;
mod method;

#[proc_macro]
#[proc_macro_error]
pub fn server_method(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as method::Method);
    let ts = quote! {
        workflow_rpc::server::Method::new(#result)
    };
    ts.into()
}

#[proc_macro]
#[proc_macro_error]
pub fn server_notification(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as method::Method);
    let ts = quote! {
        workflow_rpc::server::Notification::new(#result)
    };
    ts.into()
}

#[proc_macro]
#[proc_macro_error]
pub fn client_notification(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as method::Method);
    let ts = quote! {
        workflow_rpc::client::Notification::new(#result)
    };
    ts.into()
}
