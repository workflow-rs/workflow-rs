use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

pub fn derive_cast_from_js(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let expanded = match data {
        Data::Struct(_) => {
            quote! {
                impl ::workflow_wasm::convert::CastFromJs for #name {
                    fn try_ref_from_js_value(js: impl AsRef<::wasm_bindgen::JsValue>) -> std::result::Result<<Self as wasm_bindgen::convert::RefFromWasmAbi>::Anchor, ::workflow_wasm::error::Error> {
                        ::workflow_wasm::convert::try_ref_from_abi_safe::<Self>(::core::stringify!(#name), js)
                    }
                    fn try_long_ref_from_js_value(js: impl AsRef<::wasm_bindgen::JsValue>) -> std::result::Result<<Self as wasm_bindgen::convert::RefFromWasmAbi>::Anchor, ::workflow_wasm::error::Error> {
                        ::workflow_wasm::convert::try_long_ref_from_abi_safe::<Self>(::core::stringify!(#name), js)
                    }
                }
            }
        }
        Data::Enum(_) => {
            quote! {
                impl #name {
                    // pub fn try_from_js_value(js: &::wasm_bindgen::JsValue) -> std::result::Result<Self, ::workflow_wasm::error::Error> {
                    pub fn try_cast_from(js: &::wasm_bindgen::JsValue) -> std::result::Result<Self, ::workflow_wasm::error::Error> {
                        <Self as wasm_bindgen::convert::TryFromJsValue>::try_from_js_value(js.clone())
                            .map_err(|err| ::workflow_wasm::error::Error::from(err))
                    }
                }
            }
        }
        _ => return derive_error!("TryFromJsValue may only be derived on structs"),
    };

    TokenStream::from(expanded)
}
