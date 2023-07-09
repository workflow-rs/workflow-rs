//!
//! Sendable NewType for automatic Send marker tagging of JS primitives.
//!

use std::fmt::Display;
use wasm_bindgen::JsValue;

///
/// Senable wrapper for JS primitives.
///
/// Wrapping any JS primitive (JsValue, JsString, JsArray, JsObject, etc.) in
/// `Sendable<T>` wraps the value with the Send marker, making it transportable
/// across "thread boundaries". In reality, this allows JS primitives to be
/// used safely within a single-threaded WASM async environment (browser).
///
#[derive(Clone, Debug)]
pub struct Sendable<T>(pub T)
where
    T: Clone;
unsafe impl<T> Send for Sendable<T> where T: Clone {}
unsafe impl<T> Sync for Sendable<T> where T: Clone {}

impl<T> Sendable<T>
where
    T: Clone,
{
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn unwrap(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for Sendable<T>
where
    T: Clone,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> AsRef<T> for Sendable<T>
where
    T: Clone,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Sendable<T>
where
    T: Clone,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> From<T> for Sendable<T>
where
    T: Clone,
{
    fn from(t: T) -> Self {
        Sendable(t)
    }
}

impl<T> Display for Sendable<T>
where
    T: Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<Sendable<T>> for JsValue
where
    T: Clone + Into<JsValue>,
{
    fn from(s: Sendable<T>) -> Self {
        s.0.into()
    }
}

#[derive(Clone, Debug)]
pub struct SendableFuture<T>(pub T);
unsafe impl<T> Send for SendableFuture<T> {}
unsafe impl<T> Sync for SendableFuture<T> {}
