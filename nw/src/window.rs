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

// this can be used to test...
// pub async fn get_all_async() -> Result<Vec<Window>> {
//     let (sender, receiver) = oneshot();
//     let callback = Callback::new_with_args_2(move |windows: JsValue, v2: JsValue| {
//         log_info!("windows: {:?}", windows);
//         log_info!("v2: {:?}", v2);

//         sender.try_send(Sendable(windows)).unwrap();
//     });

//     nw_sys::window::get_all(callback.as_ref());
//     let windows = receiver.recv().await?;
//     let length = Reflect::get(&windows, &"length".into())?;
//     log_info!("length: {:?}", length);
//     let length = length.as_f64().unwrap() as usize;
//     let result = (0..length)
//         .into_iter()
//         .map(|index| {
//             Reflect::get(&windows, &index.into())
//                 .expect("failed to get nw window")
//                 .unchecked_into::<Window>()
//         })
//         .collect::<Vec<_>>();

//     Ok(result)
// }
