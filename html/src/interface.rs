use crate::WebElement;
use crate::render::{Render, Renderables};
pub use crate::utils::{Element, ElementResult, document};
use std::collections::BTreeMap;
/// Map of `@name`-tagged element bindings keyed by their declared name.
pub type Hooks = BTreeMap<String, Element>;
//use workflow_log::log_trace;

/// The result of rendering an HTML template: the live DOM root elements, the
/// map of `@name`-tagged hook elements, and the retained Rust renderables that
/// keep elements and callbacks alive for the lifetime of this structure.
#[derive(Clone)]
pub struct Html {
    /// The top-level DOM elements produced by the template.
    pub roots: Vec<Element>,
    /// Map of `@name`-tagged elements available for external binding.
    pub hooks: Hooks,
    /// Retained renderables that keep DOM elements and Rust state alive.
    pub renderables: Renderables,
}

impl Html {
    /// Constructs an [`Html`] from its rendered root elements, hooks map and
    /// retained renderables.
    pub fn new(roots: Vec<Element>, hooks: Hooks, renderables: Renderables) -> ElementResult<Html> {
        let html = Html {
            roots,
            hooks,
            renderables,
        };
        Ok(html)
    }

    /// Returns the top-level DOM elements produced by the template.
    pub fn roots(&self) -> &Vec<Element> {
        &self.roots
    }

    /// Returns the map of `@name`-tagged elements available for external binding.
    pub fn hooks(&self) -> &Hooks {
        &self.hooks
    }

    /// Appends each root element into the given parent DOM `element`.
    pub fn inject_into(&self, element: &Element) -> ElementResult<()> {
        for root in self.roots.iter() {
            element.append_child(root)?;
        }
        Ok(())
    }
    /// Removes all event listeners from every retained renderable, releasing
    /// their closures.
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
