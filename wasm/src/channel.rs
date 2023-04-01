//!
//! Subscription-based channel multiplexer - WASM client.
//!

use crate::result::Result;
use futures::{select, FutureExt};
use js_sys::Function;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use workflow_core::channel::{DuplexChannel, Multiplexer};
use workflow_core::task::*;
use crate::sendable::Sendable;
use serde::Serialize;
use serde_wasm_bindgen::*;
use workflow_log::log_error;

///
/// [`MultiplexerClient`] is an object meant to be used in WASM environment to
/// process channel events.
///

#[wasm_bindgen]
pub struct MultiplexerClient {
    callback: Arc<Mutex<Option<Sendable<Function>>>>,
    task_running: AtomicBool,
    task_ctl: DuplexChannel,
}

impl Default for MultiplexerClient {
    fn default() -> Self {
        MultiplexerClient::new()
    }
}

impl MultiplexerClient {

    pub async fn start_notification_task<T>(&self, multiplexer: &Arc<Multiplexer<T>>) -> Result<()>
    where
        T: Clone + Serialize + Send + Sync + 'static,
    {
        if self.task_running.load(Ordering::SeqCst) {
            panic!("ReflectorClient task is already running");
        }
        let ctl_receiver = self.task_ctl.request.receiver.clone();
        let ctl_sender = self.task_ctl.response.sender.clone();
        let callback = self.callback.clone();
        self.task_running.store(true, Ordering::SeqCst);

        let (channel_id, _, receiver) = multiplexer.register_event_channel();

        let multiplexer = multiplexer.clone();
        spawn(async move {
            loop {
                select! {
                    _ = ctl_receiver.recv().fuse() => {
                        break;
                    },
                    msg = receiver.recv().fuse() => {
                        // log_info!("notification: {:?}",msg);
                        if let Ok(notification) = &msg {
                            if let Some(callback) = callback.lock().unwrap().as_ref() {
                                // if let Ok(event) = JsValue::try_from(notification) {
                                if let Ok(event) = to_value(notification) {
                                    if let Err(err) = callback.0.call1(&JsValue::undefined(), &event) {
                                        log_error!("Error while executing notification callback: {:?}", err);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            multiplexer.unregister_event_channel(channel_id);
            ctl_sender.send(()).await.ok();
        });

        Ok(())
    }
}

#[wasm_bindgen]
impl MultiplexerClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MultiplexerClient {
        MultiplexerClient {
            callback: Arc::new(Mutex::new(None)),
            task_running: AtomicBool::new(false),
            task_ctl: DuplexChannel::oneshot(),
        }
    }

    #[wasm_bindgen(js_name = "setHandler")]
    pub fn set_handler(&self, callback: JsValue) -> Result<()> {
        if callback.is_function() {
            let fn_callback: Function = callback.into();
            self.callback.lock().unwrap().replace(fn_callback.into());
        } else {
            self.remove_handler()?;
        }
        Ok(())
    }

    /// `removeHandler` must be called when releasing ReflectorClient
    /// to stop the background event processing task
    #[wasm_bindgen(js_name = "removeHandler")]
    pub fn remove_handler(&self) -> Result<()> {
        *self.callback.lock().unwrap() = None;
        Ok(())
    }

    #[wasm_bindgen(js_name = "stop")]
    pub async fn stop_notification_task(&self) -> Result<()> {
        if self.task_running.load(Ordering::SeqCst) {
            self.task_running.store(false, Ordering::SeqCst);
            self.task_ctl
                .signal(())
                .await
                .map_err(|err| JsValue::from_str(&err.to_string()))?;
        }
        Ok(())
    }
}
