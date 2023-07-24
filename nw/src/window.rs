use crate::result::Result;
use nw_sys::prelude::*;
use wasm_bindgen::prelude::*;
use workflow_core::channel::oneshot;
use workflow_wasm::callback::Callback;

pub async fn get_all_async() -> Result<Vec<Window>> {
    let (sender, receiver) = oneshot();
    let callback = Callback::new(move |windows: JsValue| {
        let array = windows
            .dyn_into::<js_sys::Array>()
            .expect("nw_sys::window::get_all_async() error converting to window array");
        let windows = array
            .to_vec()
            .into_iter()
            .map(|window: JsValue| window.unchecked_into::<Window>())
            .collect::<Vec<Window>>();

        sender.try_send(windows).unwrap();
    });

    nw_sys::window::get_all(callback.as_ref());
    let result = receiver.recv().await?;

    Ok(result)
}
