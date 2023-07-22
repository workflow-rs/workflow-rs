use web_sys::MouseEvent;

use crate::utils::*;

pub fn open(url: &str) {
    let document = document();
    // let body = body().expect("open_link(): unable to obtain document body");

    let el = document.create_element("a").unwrap();
    el.set_attribute("href", url).unwrap();
    el.set_attribute("target", "_blank").unwrap();

    let event = MouseEvent::new("click").unwrap();
    el.dispatch_event(&event).unwrap();
}
