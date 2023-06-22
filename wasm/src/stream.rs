//!
//! Conversion from Rust streams into async JavaScript generators.
//!
//! This module allows you to convert any `future::Stream<Item>` where `Item : Into<JsValue>`
//! into an async JavaScript generator.
//!
//! ```no_run
//! let js_value : JsValue = AsyncStream::new(stream).into();
//! ```
//! or
//! ```no_run
//! let js_value = create_async_stream_iterator(stream);
//! ```
//!
//! For example:
//! ```no_run
//! #[wasm_bindgen]
//! fn test() {
//!    let iter = stream::iter(0..30);
//!    AsyncStream::new(iter).into()
//! }
//! ```
//!
//! Then, on JavaScript side, you can can consume it as follows:
//! ```js
//!     let iter = myFn(); // get the generator from Rust
//!     for await (let item of iter) {
//!            console.log("item ->",item);
//!     }
//! ```
//!

use crate::error::Error;
use crate::object::*;
use futures::{Stream, StreamExt};
use js_sys::Object;
use std::pin::Pin;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
struct AsyncStreamProxy(Pin<Box<dyn Stream<Item = JsValue>>>);

impl AsyncStreamProxy {
    pub fn new<T>(source: impl Stream<Item = T> + Send + 'static) -> Self
    where
        T: Into<JsValue> + Send + 'static,
    {
        AsyncStreamProxy(Box::pin(source.map(|item| item.into())))
    }
}

#[wasm_bindgen]
impl AsyncStreamProxy {
    #[allow(dead_code)]
    pub async fn next(&mut self) -> Result<JsValue, Error> {
        let object = Object::new();
        let result = match self.0.next().await {
            Some(value) => {
                object.set("value", &value)?;
                object.into()
            }
            None => {
                object.set("done", &JsValue::from(true))?;
                object.into()
            }
        };
        Ok(result)
    }
}

///
/// `AsyncStream` is a helper that receives a stream that must correspond
/// to the following spec: `Stream<Item = T> where T : Into<JsValue> + Send + 'static`.
/// The stream must be supplied via the `AsyncStream::new` constructor.
///
/// You can then use `into()` to obtain a `JsValue` the represents a
/// JavaScript generator iterating this stream.
///
pub struct AsyncStream(AsyncStreamProxy);

impl AsyncStream {
    pub fn new<T>(source: impl Stream<Item = T> + Send + 'static) -> Self
    where
        T: Into<JsValue> + Send + 'static,
    {
        Self(AsyncStreamProxy::new(source))
    }
}

static mut ASYNC_ITER_PROXY_FN: Option<js_sys::Function> = None;

fn async_iter_proxy_fn() -> &'static js_sys::Function {
    unsafe {
        ASYNC_ITER_PROXY_FN.get_or_insert_with(|| {
            js_sys::Function::new_with_args(
                "iter",
                "return (async function* () {
                        let done = false;
                        let item = await iter.next();
                        while (!item.done) {
                            yield item.value;
                            item = await iter.next();
                        }
                    })();
                ",
            )
        })
    }
}

impl From<AsyncStream> for JsValue {
    fn from(stream: AsyncStream) -> Self {
        let proxy_fn = async_iter_proxy_fn();
        proxy_fn
            .call1(&wasm_bindgen::JsValue::undefined(), &stream.0.into())
            .unwrap_or_else(|err| panic!("create_async_stream_iterator(): {:?}", err))
    }
}

///
/// Helper function that receives a stream and returns a `JsValue` representing
/// the JavaScript generator iterating this stream. The function uses `AsyncStream`
/// internally as follows: `AsyncStream::new(stream).into()`
///
pub fn create_async_stream_iterator<T>(source: impl Stream<Item = T> + Send + 'static) -> JsValue
where
    T: Into<JsValue> + Send + 'static,
{
    AsyncStream::new(source).into()
}
