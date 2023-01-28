//!
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-html.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-html)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--html-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-html)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-html.svg?maxAge=2592000&color=6ac&style=for-the-badge&logo=opensourceinitiative&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! [`workflow-html`](self) crate provides HTML templating macros that return
//! an [`Html`] structure containing a collection of DOM elements as well as retained
//! Rust structures supplied to the template. This ensures the lifetime of Rust
//! structures for the period [`Html`] structure is kept alive. Dropping [`Html`]
//! structure destroys all retained DOM elements as well as Rust structures.
//!
//! By retaining Rust structures this API ensures that elements and callbacks
//! created by Rust-based HTML elements are retained for the duration of the
//! Html litefime.
//!
//! In addition, HTML elements marked with `@name` attributes are collected into
//! a separate `HashMap` allowing client to side-access them for external bindings.
//!
//! This crate works in conjunction with [`workflow-ux`](https://crates.io/crates/workflow-ux)
//! allowing Rust HTML Form binding to HTML.
//!
//!

pub mod escape;
pub mod interface;
pub mod render;
pub mod utils;
pub use interface::{Hooks, Html};

pub use escape::{escape_attr, escape_html};
pub use render::{Render, Renderables, Result, Write};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
pub use utils::{document, Element as WebElement, ElementResult};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
pub use workflow_html_macros::{html, html_str, renderable, tree};

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Bool(bool),
    Str(String),
}

pub type OnClickClosure = Closure<dyn FnMut(web_sys::MouseEvent)>;

#[derive(Debug, Default, Clone)]
pub struct Element<T: Render> {
    pub is_fragment: bool,
    pub tag: String,
    pub attributes: BTreeMap<String, AttributeValue>,
    pub children: Option<T>,
    pub reff: Option<(String, String)>,
    pub onclick: Arc<Mutex<Option<OnClickClosure>>>,
}

impl<T: Render + Clone + 'static> Element<T> {
    pub fn on(self, name: &str, cb: Box<dyn Fn(web_sys::MouseEvent, WebElement)>) -> Self {
        if name.eq("click") {
            let mut onclick = self.onclick.lock().unwrap();
            *onclick = Some(Closure::<dyn FnMut(web_sys::MouseEvent)>::new(Box::new(
                move |event: web_sys::MouseEvent| {
                    let target = event.target().unwrap().dyn_into::<WebElement>().unwrap();
                    cb(event, target)
                },
            )));
        }
        self
    }
    //self_.home_item.element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
}

pub trait ElementDefaults {
    fn _get_attributes(&self) -> String;
    fn _get_children(&self) -> String;

    fn get_attributes(&self) -> String {
        self._get_attributes()
    }
    fn get_children(&self) -> String {
        self._get_children()
    }
}

impl<T: Render + Clone + 'static> Render for Element<T> {
    fn render_node(
        self,
        parent: &mut WebElement,
        map: &mut Hooks,
        renderables: &mut Renderables,
    ) -> ElementResult<()> {
        renderables.push(Arc::new(self.clone()));
        let mut el = document().create_element(&self.tag)?;

        let onclick = self.onclick.lock().unwrap();
        if let Some(onclick) = onclick.as_ref() {
            el.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())?;
        }

        for (key, value) in &self.attributes {
            match value {
                AttributeValue::Bool(v) => {
                    if *v {
                        el.set_attribute(key, "true")?;
                    }
                }
                AttributeValue::Str(v) => {
                    el.set_attribute(key, v)?;
                }
            }
        }
        if let Some((key, value)) = self.reff {
            el.set_attribute("data-ref", &value)?;
            map.insert(key, el.clone());
        }
        if let Some(children) = self.children {
            children.render_node(&mut el, map, renderables)?;
        }

        parent.append_child(&el)?;
        Ok(())
    }
    fn render(&self, w: &mut Vec<String>) -> ElementResult<()> {
        if self.is_fragment {
            if let Some(children) = &self.children {
                children.render(w)?;
            }
        } else {
            w.push(format!("<{}", self.tag));
            for (key, value) in &self.attributes {
                match value {
                    AttributeValue::Bool(v) => {
                        if *v {
                            w.push(format!(" {key}"));
                        }
                    }
                    AttributeValue::Str(v) => {
                        w.push(format!(" {}=\"{}\"", key, (*v)));
                    }
                }
            }
            w.push(">".to_string());
            if let Some(children) = &self.children {
                children.render(w)?;
            }
            w.push(format!("</{}>", self.tag));
        }
        Ok(())
    }

    fn remove_event_listeners(&self) -> ElementResult<()> {
        *self.onclick.lock().unwrap() = None;
        if let Some(children) = &self.children {
            children.remove_event_listeners()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    //cargo test -- --nocapture --test-threads=1
    use crate as workflow_html;
    use crate::tree;
    use crate::Render;
    //use crate::renderable;
    //use crate::ElementDefaults;
    #[test]
    pub fn simple_html() {
        self::print_hr("simple_html");
        let tree = tree! {
            <p>
                <div class="xyz abc active">"some inner html"</div>
                <div class={"abc"}></div>
            </p>
        };
        let result = tree.html();
        println!("tag: {:#?}", tree.tag);
        println!("html: {}", result);
        assert_eq!(
            result,
            "<p><div class=\"xyz abc active\">some inner html</div><div class=\"abc\"></div></p>"
        );
    }
    #[test]
    pub fn custom_elements() {
        self::print_hr("simple_html");
        let tree = tree! {
            <flow-select>
                <flow-menu-item class={"xyz"} />
                <flow-menu-item class={"abc"} />
            </flow-select>
        };
        let result = tree.html();
        println!("tag: {:#?}", tree.tag);
        println!("html: {}", result);
        assert_eq!(result, "<flow-select><flow-menu-item class=\"xyz\"></flow-menu-item><flow-menu-item class=\"abc\"></flow-menu-item></flow-select>");
    }
    #[test]
    pub fn without_root_element() {
        self::print_hr("without_root_element");
        let tree = tree! {
            <div class={"xyz"}></div>
            <div class={"abc"}></div>
        };
        let result = tree.html();
        println!("html: {}", result);
        assert_eq!(result, "<div class=\"xyz\"></div><div class=\"abc\"></div>");
    }
    #[test]
    pub fn complex_html() {
        self::print_hr("complex_html");
        /*let world  = "world";
        let num  = 123;
        let string  = "123".to_string();
        let string2  = "string2 value".to_string();
        let user = "123";
        let active = true;
        let disabled = false;
        let selected = "1";


        #[renderable(flow-select)]
        #[allow(unused_variables)]
        struct FlowSelect{
            #[attr(name="is-active")]
            pub active:bool,
            pub selected:String,
            pub name:String,
            pub children:Option<Vec<std::sync::Arc<dyn Render>>>,
            pub label:Option<String>
        }

        #[renderable(flow-menu-item)]
        struct FlowMenuItem<'a, R:Render>{
            pub text:&'a str,
            pub value:&'a str,
            pub children:Option<R>
        }


        //overries
        /*
        impl<'a> FlowSelect<'a>{

            fn get_attributes(&self)->String{
                format!("class=\"xxxxxxx\" active")
            }
            fn get_children(&self)->String{
                format!("<flow-menu-item value=\"sss\">xyz</flow-menu-item>")
            }
        }
        */
        //let name = "abc".to_string();
        //let selected = "1".to_string();
        let name2 = "aaa".to_string();
        let name3 = "bbb".to_string();
        let tree = tree!{
            <div class={"abc"} ?active ?disabled ?active2={false} user data-user-name={"test-node"} &string2>
                {123} {"hello"} {world} {num} {num} {num} {string} {true}
                {1.2 as f64}
                <h1>{"hello 123"} {num}</h1>
                {"10"}
                {11}
                {12} {13} {14}
                <h3>{"single child"}</h3>
                <FlowSelect active name={name2} selected={"<1&2>\"3"} />
                <div class={"abc"}></div>
                <FlowSelect active name={name3} &selected>
                    <flow text={"abc"} />
                    <FlowMenuItem text={"abc"} value={"abc"} />
                </FlowSelect>
            </div>
        };

        let result = tree.html();
        println!("tag: {:#?}", tree.tag);
        println!("html: {}", result);
        assert_eq!(
            result,
            "<div active class=\"abc\" data-user-name=\"test-node\" string2=\"string2 value\" user=\"123\">123helloworld123123123123true1.2<h1>hello 123123</h1>1011121314<h3>single child</h3><flow-select is-active selected=\"&lt;1&amp;2&gt;&quot;3\" name=\"aaa\"></flow-select><div class=\"abc\"></div><flow-select is-active selected=\"1\" name=\"bbb\"><flow text=\"abc\"></flow><flow-menu-item text=\"abc\" value=\"abc\"></flow-menu-item></flow-select></div>"
        );
        */
    }

    fn print_hr<'a>(_title: &'a str) {
        //println!("\n☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁\n");
        println!("\n☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰\n")
    }
}
