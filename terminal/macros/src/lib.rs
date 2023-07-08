use proc_macro::TokenStream;
mod handlers;
mod register;

#[proc_macro]
pub fn declare_handler(input: TokenStream) -> TokenStream {
    handlers::declare_handler(input)
}

#[proc_macro_derive(Handler, attributes(help))]
pub fn declare_handler_derive(input: TokenStream) -> TokenStream {
    handlers::declare_handler_derive(input)
}

#[proc_macro]
pub fn register_handlers(input: TokenStream) -> TokenStream {
    register::register_handlers(input)
}
