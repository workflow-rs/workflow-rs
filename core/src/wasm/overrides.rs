use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{console, window, Blob, BlobPropertyBag, Url, Worker};

#[wasm_bindgen]
pub struct TimerManager {
    worker: Worker, // Used to run the JavaScript code in a separate thread
    next_id: u32, // Used to generate unique IDs for intervals and timeouts
    callbacks: js_sys::Map, // Used to store the callbacks that will be called when the interval/timeout is triggered
}

#[wasm_bindgen]
impl TimerManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TimerManager {
        // The JavaScript code that will run in the worker
        // It listens for messages and sets/clears intervals and timeouts
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

        // Create a blob from the code
        let blob_parts = js_sys::Array::new();

        // Convert the code to a blob
        blob_parts.push(&JsValue::from_str(code));

        // Create a blob with the code
        let blob = Blob::new_with_str_sequence_and_options(
            &blob_parts.into(), // Convert the array to a sequence
            &BlobPropertyBag::new().type_("application/javascript"), // Set the type to JavaScript
        )
        .expect("failed to create blob");

        // Create a URL for the blob
        let url = Url::create_object_url_with_blob(&blob).expect("failed to create URL for blob");

        // Create a worker with the URL of the blob
        let worker = Worker::new(&url).expect("failed to create worker");

        // Create a map to store the callbacks that will be called when the interval/timeout is triggered
        let callbacks = js_sys::Map::new();

        // Clone the worker and callbacks so they can be used in the closure
        let worker_clone = worker.clone();
        let callbacks_clone = callbacks.clone();

        // Create a closure that will be called when the worker sends a message
        let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            // The worker sends an object with the name of the function to call and the ID of the interval/timeout
            if let Ok(id) = js_sys::Reflect::get(&event.data(), &JsValue::from_str("id")) {
                // Get the callback function from the map
                let callback = callbacks_clone.get(&id);
                // Check if the callback is defined
                let undefined = callback.is_undefined();
                if !undefined {
                    // If the callback is defined, call it
                    if let Some(func) = callback.dyn_ref::<js_sys::Function>() {
                        func.call0(&JsValue::NULL).unwrap();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>); // As a Box<dyn FnMut(_)> type which is a function that takes a single argument

        // Set the closure as the onmessage handler for the worker
        worker_clone.set_onmessage(Some(closure.as_ref().unchecked_ref()));

        // Forget the closure so it is not deallocated from memory
        closure.forget();

        TimerManager {
            worker,
            next_id: 1,
            callbacks,
        }
    }

    /// Replacement for setInterval
    #[wasm_bindgen(js_name = "setInterval")]
    pub fn set_interval(&mut self, callback: &JsValue, time: u32) -> u32 {
        // Get the current ID and increment it
        let id = self.next_id;
        self.next_id += 1;

        // Store the callback in the map with the ID as the key
        self.callbacks.set(&JsValue::from(id), callback);

        let request = js_sys::Object::new();
        // Set the name of the function to call which is setInterval
        js_sys::Reflect::set(
            &request,
            &JsValue::from("name"),
            &JsValue::from("setInterval"),
        )
        .unwrap();

        // Set the ID of the interval
        js_sys::Reflect::set(&request, &JsValue::from("id"), &JsValue::from(id)).unwrap();

        // Set the time of the interval
        js_sys::Reflect::set(&request, &JsValue::from("time"), &JsValue::from(time)).unwrap();

        // Post the message to the worker
        self.worker.post_message(&request).unwrap();

        // Return the ID of the interval
        id
    }

    #[wasm_bindgen(js_name = "clearInterval")]
    pub fn clear_interval(&mut self, id: u32) {

        let request = js_sys::Object::new();

        // Set the name of the function to call which is clearInterval
        js_sys::Reflect::set(
            &request,
            &JsValue::from("name"),
            &JsValue::from("clearInterval"),
        )
        .unwrap();

        // Set the ID of the interval
        js_sys::Reflect::set(&request, &JsValue::from("id"), &JsValue::from(id)).unwrap();

        // Post the message to the worker
        self.worker.post_message(&request).unwrap();

        // Delete the callback from the map
        self.callbacks.delete(&JsValue::from(id));
    }

    #[wasm_bindgen(js_name = "setTimeout")]
    pub fn set_timeout(&mut self, callback: &JsValue, time: u32) -> u32 {

        // Since we are adding a new function, we need to increment the ID
        let id = self.next_id;
        self.next_id += 1;

        // Store the callback in the map with the ID as the key
        self.callbacks.set(&JsValue::from(id), callback);
        let request = js_sys::Object::new();

        // Set the name of the function to call which is setTimeout
        js_sys::Reflect::set(
            &request,
            &JsValue::from("name"),
            &JsValue::from("setTimeout"),
        )
        .unwrap();

        // Set the ID of the timeout
        js_sys::Reflect::set(&request, &JsValue::from("id"), &JsValue::from(id)).unwrap();

        // Set the time of the timeout 
        js_sys::Reflect::set(&request, &JsValue::from("time"), &JsValue::from(time)).unwrap();

        // Post the message to the worker
        self.worker.post_message(&request).unwrap();

        id
    }

    #[wasm_bindgen(js_name = "clearTimeout")]
    pub fn clear_timeout(&mut self, id: u32) {

        // Create a new object to send to the worker
        let request = js_sys::Object::new();

        // Set the name of the function to call which is clearTimeout
        js_sys::Reflect::set(
            &request,
            &JsValue::from("name"),
            &JsValue::from("clearTimeout"),
        )
        .unwrap();

        // Set the ID of the timeout
        js_sys::Reflect::set(&request, &JsValue::from("id"), &JsValue::from(id)).unwrap();
        self.worker.post_message(&request).unwrap();

        // Delete the callback from the map 
        self.callbacks.delete(&JsValue::from(id));
    }
}

#[wasm_bindgen]
pub fn init_timer_overrides() -> Result<(), JsValue> {
    // Get the global window object
    let window = window().unwrap();

    // Create a new TimerManager and wrap it in an Rc<RefCell<_>> so 
    // that it can be shared across functions and still share mutable state.
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

            timer_manager.borrow_mut().clear_interval(id);
        }) as Box<dyn FnMut(u32)>)
    };

    // Wrap the setTimeout function so it returns the timer ID
    let set_timeout_closure = {
        let timer_manager = timer_manager.clone();
        Closure::wrap(Box::new(move |callback: JsValue, time: u32| -> JsValue {

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
