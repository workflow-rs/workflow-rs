use nw_sys::prelude::*;
use wasm_bindgen::prelude::*;
use workflow_core::channel::oneshot;
use workflow_wasm::callback::Callback;
// use crate::error::Error;
use crate::result::Result;
use workflow_log::*;

pub async fn get_all_async() -> Result<Vec<Window>> {
    let (sender, receiver) = oneshot();
    log_info!("get_all_async starting...");
    let callback = Callback::new(move |windows: JsValue| {
        log_info!("get_all_async callback... A windows: {:?}", windows);

        let array = windows
            .dyn_into::<js_sys::Array>()
            .expect("nw_sys::window::get_all_async() error converting to window array");
        log_info!("get_all_async callback... B");
        let windows = array
            .to_vec()
            .into_iter()
            // .map(|window: JsValue| window.unchecked_into::<Window>())
            .map(|window: JsValue| window.unchecked_into::<Window>())
            // .collect::<std::result::Result<Vec<Window>, JsValue>>();
            .collect::<Vec<Window>>();
        log_info!("get_all_async callback... C");

        sender.try_send(windows).unwrap();
    });
    log_info!("get_all_async callback... X");

    nw_sys::window::get_all(callback.as_ref());
    log_info!("get_all_async callback... Y");
    let result = receiver.recv().await?;
    log_info!("get_all_async callback... Z");
    // let result = result?;
    log_info!("get_all_async callback... ZXZX");

    Ok(result)
}
