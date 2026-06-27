//! Procedural macros for `workflow-rpc` that build server and client method and
//! notification handlers from closures or expressions, boxing and pinning their
//! async bodies for registration with the RPC runtime.

use proc_macro::TokenStream;
use proc_macro_error3::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;
mod method;

/// Constructs a server-side `workflow_rpc::server::Method` handler from a
/// closure or expression, wrapping its body so it can serve a request-response
/// RPC method.
#[proc_macro]
#[proc_macro_error]
pub fn server_method(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as method::Method);
    let ts = quote! {
        workflow_rpc::server::Method::new(#result)
    };
    ts.into()
}

/// Constructs a server-side `workflow_rpc::server::Notification` handler from
/// a closure or expression, wrapping its body so it can handle inbound
/// notifications from clients.
#[proc_macro]
#[proc_macro_error]
pub fn server_notification(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as method::Method);
    let ts = quote! {
        workflow_rpc::server::Notification::new(#result)
    };
    ts.into()
}

/// Constructs a client-side `workflow_rpc::client::Notification` handler from
/// a closure or expression, wrapping its body so it can be registered to receive
/// server-initiated notifications.
#[proc_macro]
#[proc_macro_error]
pub fn client_notification(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as method::Method);
    let ts = quote! {
        workflow_rpc::client::Notification::new(#result)
    };
    ts.into()
}
