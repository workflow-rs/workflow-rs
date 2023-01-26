//!
//! DOM manipulation utilities.
//!
//! Provides DOM injection functionality allowing injecting
//! buffer slices into DOM as [`Blob`](https://developer.mozilla.org/en-US/docs/Web/API/Blob)
//! objects and loading them as `<script>` or `<style>` elements.
//! This crate is useful for embedding JavaScript sources and
//! stylesheets directly into WASM files or loading JavaSctipt code
//! from external resources such as a WebSocket.
//!
//! Example:
//!
//! ```rust
//! let DATA: &[u8] = include_bytes!("source.js");
//! ...
//! inject_blob(Content::Script(DATA))?;
//! ```

pub mod error;
pub mod inject;
pub mod loader;
pub mod result;
pub mod utils;
