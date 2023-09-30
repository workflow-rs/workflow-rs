use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Error, ExprAsync, ExprAwait};

pub fn call_async_send(input: TokenStream) -> TokenStream {
    let input = match parse_macro_input::parse::<ExprAsync>(input.clone()) {
        Ok(block) => quote! {#block.await},
        Err(_) => match parse_macro_input::parse::<ExprAwait>(input) {
            Ok(block) => quote! {#block},
            Err(err) => {
                return TokenStream::from(
                    Error::new(err.span(), "expected async block or await expression")
                        .to_compile_error(),
                );
            }
        },
    };
    quote! {
        {
            let (__tx__, __rx__) = workflow_core::channel::oneshot();
            workflow_core::task::dispatch(async move {
                let _ = __tx__.send(workflow_core::sendable::Sendable(#input)).await;
            });

            __rx__.recv().await.map_err(|err|err.to_string())?.unwrap()
        }
    }
    .into()
}
