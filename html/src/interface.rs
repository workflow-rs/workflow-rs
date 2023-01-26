use crate::render::{Render, Renderables};
pub use crate::utils::{document, Element, ElementResult};
use crate::WebElement;
use std::collections::BTreeMap;
pub type Hooks = BTreeMap<String, Element>;
//use workflow_log::log_trace;

#[derive(Clone)]
pub struct Html {
    pub roots: Vec<Element>,
    pub hooks: Hooks,
    pub renderables: Renderables,
}

impl Html {
    pub fn new(roots: Vec<Element>, hooks: Hooks, renderables: Renderables) -> ElementResult<Html> {
        let html = Html {
            roots,
            hooks,
            renderables,
        };
        Ok(html)
    }

    pub fn roots(&self) -> &Vec<Element> {
        &self.roots
    }

    pub fn hooks(&self) -> &Hooks {
        &self.hooks
    }

    pub fn inject_into(&self, element: &Element) -> ElementResult<()> {
        for root in self.roots.iter() {
            element.append_child(root)?;
        }
        Ok(())
    }
    pub fn remove_event_listeners(&self) -> ElementResult<()> {
        for root in &self.renderables {
            root.remove_event_listeners()?;
        }
        Ok(())
    }
}

impl Render for Html {
    fn render_node(
        mut self,
        parent: &mut WebElement,
        map: &mut Hooks,
        renderables: &mut Renderables,
    ) -> ElementResult<()> {
        renderables.append(self.renderables.as_mut());
        let mut hooks = self.hooks().clone();
        map.append(&mut hooks);
        self.inject_into(parent)?;
        Ok(())
    }

    fn render(&self, _w: &mut Vec<String>) -> ElementResult<()> {
        Ok(())
    }

    fn remove_event_listeners(&self) -> ElementResult<()> {
        for root in &self.renderables {
            root.remove_event_listeners()?;
        }
        Ok(())
    }
}

/*
impl Drop for Html{
    fn drop(&mut self) {
        log_trace!("HTML Drop: {:?}", self.roots[0].get_attribute("class"));
    }
}
*/
