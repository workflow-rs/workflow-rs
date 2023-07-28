use crate::graph::{Graph, GraphDuration};
use crate::imports::*;
use web_sys::{Element, HtmlSelectElement};
use workflow_dom::inject::*;

static mut DOM_INIT: bool = false;

pub struct Container {
    element: Element,
    /// holds references to [Callback](workflow_wasm::callback::Callback)
    pub callbacks: CallbackMap,
    //duration_selector: Arc<Mutex<Option<Element>>>,
}

unsafe impl Sync for Container {}
unsafe impl Send for Container {}

impl Container {
    pub async fn try_init() -> Result<()> {
        if !unsafe { DOM_INIT } {
            let layout_css = include_str!("container.css");
            inject_css(layout_css)?;
            unsafe {
                DOM_INIT = true;
            }
        }

        Ok(())
    }

    pub async fn try_new(window: &web_sys::Window) -> Result<Container> {
        let document = window.document().unwrap();
        let element = document.create_element("div").unwrap();
        element.set_class_name("layout");

        let body = document
            .query_selector("body")
            .unwrap()
            .ok_or_else(|| "Unable to get body element".to_string())?;

        body.append_child(&element).unwrap();

        let layout = Container {
            element,
            callbacks: CallbackMap::new(),
            //duration_selector: Arc::new(Mutex::new(None)),
        };

        Ok(layout)
    }

    pub fn element(&self) -> &Element {
        &self.element
    }

    pub fn init_duration_selector(
        &self,
        window: &web_sys::Window,
        graphs: Vec<Arc<Graph>>,
    ) -> Result<()> {
        let doc = window.document().unwrap();
        let element = doc
            .query_selector("select.duration-selector")
            .unwrap()
            .ok_or_else(|| "Unable to get select.duration-selector element".to_string())?;
        let el = Arc::new(element.dyn_into::<HtmlSelectElement>().unwrap());
        let el_clone = el.clone();
        let on_change = callback!(move || {
            let value = el_clone.value();
            workflow_log::log_info!("duration-selector:change: {value:?}");
            if let Ok(duration) = GraphDuration::parse(value) {
                for graph in &graphs {
                    graph.set_duration(duration);
                }
            }
        });

        el.add_event_listener_with_callback("change", on_change.get_fn())?;
        self.callbacks.retain(on_change)?;
        Ok(())
    }
}
