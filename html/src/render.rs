use crate::interface::Hooks;
use crate::utils::{document, Element, ElementResult};
use crate::Html;
use std::collections::BTreeMap;
pub use std::fmt::{Result, Write};
pub use std::sync::Arc;

pub type Renderables = Vec<Arc<dyn Render>>;

//pub type RenderPtr = Arc<dyn Render>;

pub trait Render {
    //type Type;
    //fn on(&mut self, _event:&str, _cb: Box<dyn Fn(dyn Render) -> ElementResult<()>>){

    //}
    fn html(&self) -> String {
        let mut buf = vec![];
        self.render(&mut buf).unwrap();
        buf.join("")
    }
    // fn render_tree(self)->ElementResult<(Vec<Element>, BTreeMap<String, Element>)>{
    fn render_tree(self) -> ElementResult<Html>
    where
        Self: Sized,
    {
        let mut parent = document().create_element("div").unwrap();
        //parent.set_attribute("class", "temp-root")?;
        let mut renderable = vec![];
        //renderable.push((*self).clone());
        let map = self.render_tree_into(&mut parent, &mut renderable)?;
        let mut list = vec![];
        let children = parent.children();
        let len = children.length();
        for index in 0..len {
            if let Some(child) = children.get_with_index(index) {
                list.push(child);
            }
        }

        Html::new(list, map, renderable)
    }
    fn render_tree_into(
        self,
        parent: &mut Element,
        renderables: &mut Renderables,
    ) -> ElementResult<BTreeMap<String, Element>>
    where
        Self: Sized,
    {
        let mut map = BTreeMap::new();
        self.render_node(parent, &mut map, renderables)?;
        Ok(map)
    }

    fn render_node(
        self,
        _parent: &mut Element,
        _map: &mut Hooks,
        _renderables: &mut Renderables,
    ) -> ElementResult<()>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn render(&self, _w: &mut Vec<String>) -> ElementResult<()>;

    fn remove_event_listeners(&self) -> ElementResult<()> {
        Ok(())
    }
}

//impl Render for () {}

impl Render for () {
    fn render(&self, _w: &mut Vec<String>) -> ElementResult<()> {
        Ok(())
    }
}

impl Render for &str {
    fn render(&self, w: &mut Vec<String>) -> ElementResult<()> {
        w.push(self.to_string());
        Ok(())
    }
    fn render_node(
        self,
        parent: &mut Element,
        _map: &mut Hooks,
        _renderables: &mut Renderables,
    ) -> ElementResult<()> {
        let el = document().create_text_node(self);
        parent.append_child(&el)?;
        Ok(())
    }
}

impl<T: Render + Clone> Render for Vec<T> {
    fn render(&self, list: &mut Vec<std::string::String>) -> ElementResult<()> {
        for item in self {
            item.render(list)?;
        }
        Ok(())
    }

    fn render_node(
        self,
        parent: &mut Element,
        map: &mut Hooks,
        renderables: &mut Renderables,
    ) -> ElementResult<()> {
        for item in self {
            item.render_node(parent, map, renderables)?;
        }
        Ok(())
    }
}
/*
impl<T: Render + Clone> Render for Arc<Vec<T>> {
    fn render(&self, list: &mut Vec<std::string::String>) -> ElementResult<()> {
        for item in self.iter() {
            item.render(list)?;
        }
        Ok(())
    }

    fn render_node(
        self,
        parent: &mut Element,
        map: &mut Hooks,
        renderables: &mut Renderables,
    ) -> ElementResult<()> {
        for item in self.iter() {
            item.clone().render_node(parent, map, renderables)?;
        }
        Ok(())
    }
}
*/

impl<T: Render + Clone> Render for Option<T> {
    fn render_node(
        self,
        parent: &mut Element,
        map: &mut Hooks,
        renderables: &mut Renderables,
    ) -> ElementResult<()> {
        if let Some(h) = self {
            h.render_node(parent, map, renderables)?;
        }
        Ok(())
    }

    fn render(&self, w: &mut Vec<String>) -> ElementResult<()> {
        if let Some(h) = self {
            h.render(w)?;
        }
        Ok(())
    }
}

macro_rules! impl_tuple {
    ($($ident:ident)+) => {
        impl<$($ident: Render,)+> Render for ($($ident,)+) {
            #[inline]
            #[allow(non_snake_case)]
            fn render(&self, w: &mut Vec<String>)->ElementResult<()>{
                let ($($ident,)+) = self;
                $($ident.render(w)?;)+
                Ok(())
            }
            #[allow(non_snake_case)]
            fn render_node(
                self,
                parent:&mut Element,
                map:&mut Hooks,
                renderables:&mut Renderables
            )->ElementResult<()>{
                let ($($ident,)+) = self;
                $($ident.render_node(parent, map, renderables)?;)+
                Ok(())
            }
        }
    }
}

macro_rules! impl_types {
    ($($ident:ident)+) => {
        $(
            impl Render for $ident {
                fn render(&self, w: &mut Vec<String>)->ElementResult<()>{
                    w.push(format!("{}", self));
                    Ok(())
                }
                fn render_node(
                    self,
                    parent:&mut Element,
                    _map:&mut Hooks,
                    _renderables:&mut Renderables
                )->ElementResult<()>{
                    let el = document().create_text_node(&format!("{}", self));
                    parent.append_child(&el)?;
                    Ok(())
                }
            }
        )+
    }
}

impl_types! {f32 f64 u128 u64 u32 u16 u8 i8 i16 i32 i64 i128 bool String usize}

impl_tuple! {A B}
impl_tuple! {A B C}
impl_tuple! {A B C D}
impl_tuple! {A B C D E}
impl_tuple! {A B C D F G}
impl_tuple! {A B C D F G H}
impl_tuple! {A B C D F G H I}
impl_tuple! {A B C D F G H I J}
impl_tuple! {A B C D F G H I J K}
