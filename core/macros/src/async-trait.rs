#[cfg(not(target_arch = "wasm32"))]
#[proc_macro_attribute]
pub fn async_trait_with_send(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: proc_macro2::TokenStream = item.into();
    let output = quote!{
        #[async_trait(?Send)]
        #item
    };
    output.into()
}