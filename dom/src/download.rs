use crate::result::Result;
use crate::utils::*;
use js_sys::{Array, Uint8Array};
use web_sys::{Blob, MouseEvent, Url};

pub fn data(filename: &str, content: &[u8], mime: &str) -> Result<()> {
    let document = document();
    let body = body()?;

    let args = Array::new_with_length(1);
    args.set(0, unsafe { Uint8Array::view(content).into() });
    let mut options = web_sys::BlobPropertyBag::new();
    options.type_(mime);
    let blob = Blob::new_with_u8_array_sequence_and_options(&args, &options)?;
    let url = Url::create_object_url_with_blob(&blob)?;

    let el = document.create_element("a").unwrap();
    el.set_attribute("href", filename).unwrap();
    el.set_attribute("download", &url).unwrap();

    body.append_child(&el).unwrap();

    let event = MouseEvent::new("click").unwrap();
    el.dispatch_event(&event).unwrap();

    body.remove_child(&el).unwrap();

    Url::revoke_object_url(&url)?;

    Ok(())
}

pub fn text(filename: &str, content: &str) -> Result<()> {
    data(filename, content.as_bytes(), "text/plain")
}
