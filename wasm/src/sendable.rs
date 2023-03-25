//!
//! Sendable NewType for automatic Send marker tagging of JS primitives.
//!

///
/// Senable wrapper for JS primitives.
///
/// Wrapping any JS primitive (JsValue, JsString, JsArray, JsObject, etc.) in
/// Sendable<T> wraps the value with the Send marker, making it transportable
/// across "thread boundaries". In reality, this allows JS primitives to be
/// used safely within a single-threaded WASM async environment (browser).
///
#[derive(Clone, Debug)]
pub struct Sendable<T>(pub T)
where
    T: Clone;
unsafe impl<T> Send for Sendable<T> where T: Clone {}

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
