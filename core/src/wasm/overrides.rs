use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{console, window, Blob, BlobPropertyBag, Url, Worker};

#[wasm_bindgen]
pub struct TimerManager {
    worker: Worker,
    next_id: u32,
    callbacks: js_sys::Map,
}

#[wasm_bindgen]
impl TimerManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TimerManager {
        let code = r#"
        var idMap = {};
        self.addEventListener('message', function(event) {
            let { name, id, time } = event.data;
            switch (name) {
                case 'setInterval':
                    idMap[id] = setInterval(function () {
                        postMessage({ id });
                    }, time);
                    break;
                case 'clearInterval':
                    if (idMap.hasOwnProperty(id)) {
                        clearInterval(idMap[id]);
                        delete idMap[id];
                    }
                    break;
                case 'setTimeout':
                    idMap[id] = setTimeout(function () {
                        postMessage({ id });
                        if (idMap.hasOwnProperty(id)) {
                            delete idMap[id];
                        }
                    }, time);
                    break;
                case 'clearTimeout':
                    if (idMap.hasOwnProperty(id)) {
                        clearTimeout(idMap[id]);
                        delete idMap[id];
                    }
                    break;
            }
        });
        "#;

        let blob_parts = js_sys::Array::new();
        blob_parts.push(&JsValue::from_str(code));
        let blob = Blob::new_with_str_sequence_and_options(
            &blob_parts.into(),
            &BlobPropertyBag::new().type_("application/javascript"),
        )
        .expect("failed to create blob");

        let url = Url::create_object_url_with_blob(&blob).expect("failed to create URL for blob");
        let worker = Worker::new(&url).expect("failed to create worker");

        let callbacks = js_sys::Map::new();

        let worker_clone = worker.clone();
        let callbacks_clone = callbacks.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            if let Ok(id) = js_sys::Reflect::get(&event.data(), &JsValue::from_str("id")) {
                let callback = callbacks_clone.get(&id);
                let exists = callback.is_undefined();
                if exists {
                    if let Some(func) = callback.dyn_ref::<js_sys::Function>() {
                        func.call0(&JsValue::NULL).unwrap();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        worker_clone.set_onmessage(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        TimerManager {
            worker,
            next_id: 1,
            callbacks,
        }
    }

    #[wasm_bindgen(js_name = "setInterval")]
    pub fn set_interval(&mut self, callback: &JsValue, time: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.callbacks.set(&JsValue::from(id), callback);
        let request = js_sys::Object::new();
        js_sys::Reflect::set(
            &request,
            &JsValue::from("name"),
            &JsValue::from("setInterval"),
        )
        .unwrap();
        js_sys::Reflect::set(&request, &JsValue::from("id"), &JsValue::from(id)).unwrap();
        js_sys::Reflect::set(&request, &JsValue::from("time"), &JsValue::from(time)).unwrap();
        self.worker.post_message(&request).unwrap();

        id
    }

    #[wasm_bindgen(js_name = "clearInterval")]
    pub fn clear_interval(&mut self, id: u32) {
        let request = js_sys::Object::new();
        js_sys::Reflect::set(
            &request,
            &JsValue::from("name"),
            &JsValue::from("clearInterval"),
        )
        .unwrap();
        js_sys::Reflect::set(&request, &JsValue::from("id"), &JsValue::from(id)).unwrap();
        self.worker.post_message(&request).unwrap();

        self.callbacks.delete(&JsValue::from(id));
    }

    #[wasm_bindgen(js_name = "setTimeout")]
    pub fn set_timeout(&mut self, callback: &JsValue, time: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.callbacks.set(&JsValue::from(id), callback);
        let request = js_sys::Object::new();
        js_sys::Reflect::set(
            &request,
            &JsValue::from("name"),
            &JsValue::from("setTimeout"),
        )
        .unwrap();
        js_sys::Reflect::set(&request, &JsValue::from("id"), &JsValue::from(id)).unwrap();
        js_sys::Reflect::set(&request, &JsValue::from("time"), &JsValue::from(time)).unwrap();
        self.worker.post_message(&request).unwrap();

        id
    }

    #[wasm_bindgen(js_name = "clearTimeout")]
    pub fn clear_timeout(&mut self, id: u32) {
        let request = js_sys::Object::new();
        js_sys::Reflect::set(
            &request,
            &JsValue::from("name"),
            &JsValue::from("clearTimeout"),
        )
        .unwrap();
        js_sys::Reflect::set(&request, &JsValue::from("id"), &JsValue::from(id)).unwrap();
        self.worker.post_message(&request).unwrap();

        self.callbacks.delete(&JsValue::from(id));
    }
}

#[wasm_bindgen]
pub fn init_timer_overrides() -> Result<(), JsValue> {
    let window = window().unwrap();
    let timer_manager = Rc::new(RefCell::new(TimerManager::new()));

    // Wrap the setInterval function so it returns the timer ID
    let set_interval_closure = {
        let timer_manager = timer_manager.clone();
        Closure::wrap(Box::new(move |callback: JsValue, time: u32| -> JsValue {
            let id = timer_manager.borrow_mut().set_interval(&callback, time);
            JsValue::from_f64(id as f64) // Convert the ID to JsValue and return
        }) as Box<dyn FnMut(JsValue, u32) -> JsValue>)
    };

    // Wrap the clearInterval function
    let clear_interval_closure = {
        let timer_manager = timer_manager.clone();
        Closure::wrap(Box::new(move |id: u32| {
            console::log_1(&JsValue::from_str("clearinterval"));

            timer_manager.borrow_mut().clear_interval(id);
        }) as Box<dyn FnMut(u32)>)
    };

    // Wrap the setTimeout function so it returns the timer ID
    let set_timeout_closure = {
        let timer_manager = timer_manager.clone();
        Closure::wrap(Box::new(move |callback: JsValue, time: u32| -> JsValue {
            console::log_1(&JsValue::from_str("settimeout"));

            let id = timer_manager.borrow_mut().set_timeout(&callback, time);
            JsValue::from_f64(id as f64) // Convert the ID to JsValue and return
        }) as Box<dyn FnMut(JsValue, u32) -> JsValue>)
    };

    // Wrap the clearTimeout function
    let clear_timeout_closure = {
        let timer_manager = timer_manager.clone();
        Closure::wrap(Box::new(move |id: u32| {
            console::log_1(&JsValue::from_str("cleartimeout"));
            timer_manager.borrow_mut().clear_timeout(id);
        }) as Box<dyn FnMut(u32)>)
    };

    // Set the new functions on the window object
    js_sys::Reflect::set(
        &window,
        &JsValue::from("setInterval"),
        set_interval_closure.as_ref().unchecked_ref(),
    )?;
    js_sys::Reflect::set(
        &window,
        &JsValue::from("clearInterval"),
        clear_interval_closure.as_ref().unchecked_ref(),
    )?;
    js_sys::Reflect::set(
        &window,
        &JsValue::from("setTimeout"),
        set_timeout_closure.as_ref().unchecked_ref(),
    )?;
    js_sys::Reflect::set(
        &window,
        &JsValue::from("clearTimeout"),
        clear_timeout_closure.as_ref().unchecked_ref(),
    )?;

    // Ensure Closures are kept alive long enough
    set_interval_closure.forget();
    clear_interval_closure.forget();
    set_timeout_closure.forget();
    clear_timeout_closure.forget();

    Ok(())
}
