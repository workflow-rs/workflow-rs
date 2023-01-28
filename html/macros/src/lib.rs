use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Meta, NestedMeta,
};
mod element;
//mod state;
mod attributes;
use element::Nodes;
//use state::set_attributes;
use attributes::{AttributeName, AttributeNameString};
use proc_macro_error::proc_macro_error;

#[proc_macro]
#[proc_macro_error]
pub fn tree(input: TokenStream) -> TokenStream {
    let nodes = parse_macro_input!(input as Nodes);
    let ts = quote! {#nodes};
    //println!("\n===========> Nodes Object tree <===========\n{}\n", ts.to_string());
    ts.into()
}

#[proc_macro]
#[proc_macro_error]
pub fn html(input: TokenStream) -> TokenStream {
    let nodes = parse_macro_input!(input as Nodes);
    let ts = quote! {#nodes};
    //println!("\n===========> Nodes Object tree <===========\n{}\n", ts.to_string());
    quote!({
        let elements = #ts;

        elements.render_tree()
    })
    .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn html_str(input: TokenStream) -> TokenStream {
    let nodes = parse_macro_input!(input as Nodes);
    let ts = quote! {#nodes};
    //println!("\n===========> Nodes Object tree <===========\n{}\n", ts.to_string());
    quote!({
        let elements = #ts;

        elements.html()
    })
    .into()
}

struct RenderableAttributes {
    pub tag_name: String,
}

impl Parse for RenderableAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag_name = AttributeName::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
        Ok(RenderableAttributes {
            tag_name: tag_name.to_string(),
        })
    }
}

#[proc_macro_attribute]
//#[proc_macro_derive(Renderable)]
#[proc_macro_error]
pub fn renderable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let renderable_attr = parse_macro_input!(attr as RenderableAttributes);
    let tag_name = renderable_attr.tag_name;
    let format_str = format!("<{tag_name} {{}}>{{}}</{tag_name}>");
    //println!("renderable_attr: {:?}", tag_name);
    //let def:proc_macro2::TokenStream = item.clone().into();
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    let struct_params = &ast.generics;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();
    /*let generics_only = ast.generics.clone();
    let where_clause = match generics_only.where_clause.clone() {
        Some(where_clause) => quote!{ #where_clause },
        None => quote!{}
    };
    */

    //println!("struct_params: {:#?}", struct_params);

    let mut field_visibility_vec = vec![];
    let mut field_ident_vec = vec![];
    let mut field_type_vec = vec![];
    let mut attrs_ts_vec = vec![];
    let mut field_names: Vec<String> = vec![];

    //let mut children_field_ts = quote!();
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        //let mut has_children_field = false;
        for field in fields.named.iter() {
            let field_name: syn::Ident = field.ident.as_ref().unwrap().clone();
            field_ident_vec.push(&field.ident);
            field_visibility_vec.push(&field.vis);
            field_type_vec.push(&field.ty);
            let mut attr_name = field_name.to_string();
            if attr_name.eq("children") {
                //has_children_field = true;
                continue;
            }
            field_names.push(attr_name.clone());
            //let name: String = field_name.to_string();
            //println!("\n\n----->name: {}, \ntype: {:?}, \nattrs: {:?}", field_name, field.ty, field.attrs);
            //println!("\n\n----->name: {}, \ntype: {:?}", field_name, field.ty);
            let mut attrs: Vec<_> = field.attrs.iter().collect();

            if !attrs.is_empty() {
                let attr = attrs.remove(0);
                let meta = attr.parse_meta().unwrap();
                if let Meta::List(list) = meta {
                    //println!("meta-list: {:#?}", list);
                    //println!("meta-list.path: {:#?}", list.path.get_ident().unwrap().to_string());
                    //println!("nested: {:?}", list.nested);
                    for item in list.nested.iter() {
                        if let NestedMeta::Meta(Meta::NameValue(name_value)) = item {
                            let key = name_value.path.get_ident().unwrap().to_string();
                            let value: String = match &name_value.lit {
                                syn::Lit::Int(v) => v.to_string(),
                                syn::Lit::Str(v) => v.value(),
                                syn::Lit::Bool(v) => v.value().to_string(),
                                _ => "".to_string(),
                            };
                            //println!("key: {}, value: {}", key, value);
                            if key.eq("name") {
                                attr_name = value;
                            }
                        }
                    }
                }
            }

            let field_type = match &field.ty {
                syn::Type::Path(a) => match a.path.get_ident() {
                    Some(a) => a.to_string(),
                    None => {
                        if !a.path.segments.is_empty() {
                            a.path.segments[0].ident.to_string()
                        } else {
                            "".to_string()
                        }
                    }
                },
                syn::Type::Reference(a) => match a.elem.as_ref() {
                    syn::Type::Path(a) => match a.path.get_ident() {
                        Some(a) => a.to_string(),
                        None => {
                            if !a.path.segments.is_empty() {
                                a.path.segments[0].ident.to_string()
                            } else {
                                "".to_string()
                            }
                        }
                    },
                    _ => "".to_string(),
                },
                _ => "".to_string(),
            };

            if field_type.eq("bool") {
                let fmt_str = attr_name.to_string();
                attrs_ts_vec.push(quote!(
                    if self.#field_name{
                        attrs.push(#fmt_str.to_string());
                    }
                ));
            } else {
                let fmt_str = format!("{attr_name}=\"{{}}\"");
                let mut borrow = quote!();
                if field_type.eq("String") {
                    borrow = quote!(&);
                }
                if field_type.eq("Option") {
                    attrs_ts_vec.push(quote!(
                        match &self.#field_name{
                            Some(value)=>{
                                attrs.push(format!(#fmt_str, workflow_html::escape_attr(value)));
                            }
                            None=>{

                            }
                        }
                    ));
                } else {
                    attrs_ts_vec.push(quote!(
                        attrs.push(format!(#fmt_str, workflow_html::escape_attr(#borrow self.#field_name)));
                    ));
                }
            }
        }
        //if !has_children_field{
        //    children_field_ts = quote!(
        //children:Option<R>
        //    );
        //}
    }

    //set_attributes(struct_name.to_string(), field_names);
    let ts = quote!(
        #[derive(Debug, Clone, Default)]
        pub struct #struct_name #struct_params #where_clause {
            #( #field_visibility_vec #field_ident_vec : #field_type_vec ),*,
            //#children_field_ts
        }

        impl #impl_generics workflow_html::Render for #struct_name #type_generics #where_clause {
            fn render(&self, w:&mut Vec<String>)->workflow_html::ElementResult<()>{
                let attr = self.get_attributes();
                let children = self.get_children();
                w.push(format!(#format_str, attr, children));
                Ok(())
            }
        }
        impl #impl_generics workflow_html::ElementDefaults for #struct_name #type_generics #where_clause {
            fn _get_attributes(&self)->String{
                let mut attrs:Vec<String> = vec![];
                #(#attrs_ts_vec)*
                attrs.join(" ")
            }
            fn _get_children(&self)->String{
                match &self.children{
                    Some(children)=>{
                        children.html()
                    }
                    None=>{
                        "".to_string()
                    }
                }
            }
        }
    );
    //println!("\n===========> element({}) ts: <===========\n{}", struct_name, ts);
    ts.into()
}
