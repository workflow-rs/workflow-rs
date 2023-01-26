use proc_macro2::{Ident, Literal, TokenStream};
//use proc_macro::TokenTree;
use crate::attributes::{parse_attributes, Attributes};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{punctuated::Punctuated, Block, Result, Token};
//use crate::state::get_attributes;

pub type TagName = Punctuated<Ident, Token![-]>;

pub trait TagNameString {
    fn to_string(&self) -> String;
    fn is_custom_element(&self) -> bool;
}

impl TagNameString for TagName {
    fn to_string(&self) -> String {
        let mut items = self.iter().map(|a| a.to_string());
        if items.len() == 0 {
            return "".to_string();
        }
        let first = items.next().unwrap();
        items.fold(first, |a, b| format!("{}-{}", a, b))
    }
    fn is_custom_element(&self) -> bool {
        let name = self.to_string();
        if name.len() == 0 {
            return false;
        }
        let first = name.get(0..1).unwrap();
        first.to_uppercase() == first
    }
}

pub struct Element<'a> {
    pub tag: OpeningTag<'a>,
    pub children: Option<Nodes<'a>>,
}

impl<'a> Parse for Element<'a> {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("================== start: Element parsing #######################");
        let span = input.span();
        let tag = input.parse::<OpeningTag>()?;

        let mut children = None;
        if !tag.self_closing {
            let nodes = input.parse::<Nodes>()?;
            if nodes.list.len() > 0 {
                children = Some(nodes);
            }
            let closing_tag = input.parse::<ClosingTag>()?;
            if closing_tag.name != tag.name {
                abort!(
                    span,
                    format!("Closing tag is missing for '{}'", tag.name.to_string())
                );
            }
        }
        //println!("=================== end: Element parsing ########################");
        //println!("after Element parse, input: {}", input);

        Ok(Element { tag, children })
    }
}

impl<'a> Element<'a> {
    fn is_custom_element(&self) -> bool {
        self.tag.name.is_custom_element()
    }
    fn children_stream(&self) -> TokenStream {
        match &self.children {
            Some(nodes) => {
                let children = nodes.get_tuples();
                quote!(children:Some(#children))
            }
            None => {
                quote!(children: Option::<()>::None)
            }
        }
    }
    fn has_children(&self) -> bool {
        match &self.children {
            Some(nodes) => nodes.len() > 0,
            None => false,
        }
    }
}

impl<'a> ToTokens for Element<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        //let mut properties:Vec<TokenStream> = vec![];
        let children = self.children_stream();
        let el = if self.is_custom_element() {
            let name = &self.tag.name;
            /*
            let names = match get_attributes(name.to_string()){
                Some(names)=>names,
                None=>Arc::new(vec![])
            };
            */
            let (mut properties, events) = self.tag.attributes.to_properties(); //names);
                                                                                //println!("properties: {:?}", properties);
            if self.has_children() {
                properties.push(children);
            }
            if properties.len() == 0 {
                quote!(#name {
                    ..Default::default()
                }#(#events)*)
            } else {
                quote!(#name {
                    #(#properties),*,
                    ..Default::default()
                }#(#events)*)
            }
        } else {
            let (attributes, events) = self.tag.attributes.to_token_stream();
            let tag = self.tag.name.to_string();
            let is_fragment = tag.len() == 0;
            quote! {
                workflow_html::Element {
                    is_fragment:#is_fragment,
                    tag:String::from(#tag),
                    onclick:std::sync::Arc::new(std::sync::Mutex::new(None)),
                    #attributes,
                    #children,
                }#(#events)*
            }
        };

        el.to_tokens(tokens);
    }
}

pub struct OpeningTag<'a> {
    pub name: TagName,
    pub self_closing: bool,
    pub attributes: Attributes<'a>,
}
fn get_fragment_tag_name() -> TagName {
    Punctuated::new() // Ident::new("x", Span::call_site())
}
impl<'a> Parse for OpeningTag<'a> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut self_closing = false;
        let name;
        let attributes;
        input.parse::<Token![<]>()?;
        if input.peek(Token![>]) {
            name = get_fragment_tag_name();
            attributes = Attributes::empty()
        } else {
            name = TagName::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
            attributes = parse_attributes(input)?;
            if input.peek(Token![/]) {
                input.parse::<Token![/]>()?;
                self_closing = true;
            }
        }

        input.parse::<Token![>]>()?;
        Ok(Self {
            name,
            self_closing,
            attributes,
        })
    }
}

pub struct ClosingTag {
    pub name: TagName,
}

impl Parse for ClosingTag {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() || !input.peek(Token![<]) {
            return Ok(Self {
                name: get_fragment_tag_name(),
            });
        }
        input.parse::<Token![<]>()?;
        if input.is_empty() || input.peek2(Token![/]) {
            //abort!(input.span(), format!("Closing tag is missing"));
            return Ok(Self {
                name: get_fragment_tag_name(),
            });
        }
        input.parse::<Token![/]>()?;
        if input.is_empty() {
            return Ok(Self {
                name: get_fragment_tag_name(),
            });
        }
        let name;
        if input.peek(Token![>]) {
            name = get_fragment_tag_name();
        } else {
            name = match TagName::parse_separated_nonempty_with(input, syn::Ident::parse_any) {
                Ok(tag_name) => tag_name,
                Err(_e) => {
                    //for closing tag validation making a empty close tag
                    return Ok(Self {
                        name: get_fragment_tag_name(),
                    });
                }
            };
        }
        if input.is_empty() || !input.peek(Token![>]) {
            return Ok(Self {
                name: get_fragment_tag_name(),
            });
        }
        input.parse::<Token![>]>()?;
        Ok(Self { name })
    }
}

pub struct Nodes<'a> {
    list: Vec<Node<'a>>,
}

impl<'a> Nodes<'a> {
    pub fn len(&self) -> usize {
        self.list.len()
    }
    pub fn get_tuples(&self) -> TokenStream {
        if self.list.len() == 1 {
            let node = &self.list[0];
            quote! {#node}
        } else {
            let mut group = vec![];
            let list: Vec<TokenStream> = self.list.iter().map(|item| quote! {#item}).collect();
            for chunk in list.chunks(10) {
                group.push(quote! { ( #(#chunk),* ) });
                if group.len() == 10 {
                    let combined = quote! { ( #(#group),* ) };
                    group = vec![];
                    group.push(combined);
                }
            }

            let children = quote! {(#(#group),*)};
            quote! {#children}
        }
    }
}

impl<'a> Parse for Nodes<'a> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut list: Vec<Node> = vec![];
        //println!("================== start: Nodes parsing ==================");
        while !input.is_empty() && (!input.peek(Token![<]) || !input.peek2(Token![/])) {
            let node = input.parse::<Node>()?;
            list.push(node);
        }
        //println!("==================== end: Nodes parsing ==================");
        //println!("after nodes parse, input: {:?}", input);

        Ok(Nodes { list })
    }
}
impl<'a> ToTokens for Nodes<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.get_tuples().to_tokens(tokens);
    }
}

pub enum Node<'a> {
    Element(Element<'a>),
    Block(Block),
    //TokenStream(proc_macro2::TokenStream)
    Literal(Literal),
}

impl<'a> Parse for Node<'a> {
    fn parse(input: ParseStream) -> Result<Self> {
        let node = if input.peek(Token![<]) {
            Node::Element(input.parse::<Element>()?)
        } else if input.peek(syn::token::Brace) {
            Node::Block(input.parse::<Block>()?)
        } else {
            /*
            let mut items:Vec<proc_macro2::TokenTree> = vec![];
            input.step(|cursor|{
                let mut rest = *cursor;
                while let Some((tt, next)) = rest.token_tree() {
                    if input.peek(syn::token::Brace){

                    }
                    match &tt {
                        proc_macro2::TokenTree::Punct(a) if a.as_char() == '<' =>{
                            println!("XXXXXXXXXXX");
                            return Ok(((), rest));
                        }
                        proc_macro2::TokenTree::Group(_a)=>{
                            items.push(tt);
                            rest = next;
                        }
                        _ =>{
                            items.push(tt);
                            rest = next;
                        }
                    }
                }
                Ok(((), rest))
            })?;

            Node::TokenStream(proc_macro2::TokenStream::from_iter(items))
            */
            Node::Literal(input.parse::<Literal>()?)
        };

        Ok(node)
    }
}
impl<'a> ToTokens for Node<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Node::Element(el) => {
                el.to_tokens(tokens);
            }
            Node::Literal(el) => {
                el.to_tokens(tokens);
            }
            Node::Block(block) => {
                if block.stmts.len() == 1 {
                    let stm = &block.stmts[0];
                    stm.to_tokens(tokens);
                } else {
                    block.to_tokens(tokens);
                }
            }
        }
    }
}
