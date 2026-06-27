//! Rust bindings and helpers for building [D3.js](https://d3js.org/) visualizations
//! in WASM browser environments, including scale wrappers and a time-series graph.

/// Commonly used re-exports shared across the crate's modules.
pub mod imports;

/// DOM container that hosts a D3 visualization and injects its layout CSS.
pub mod container;
/// Thin `wasm_bindgen` bindings to the D3 JavaScript library (scales, areas, etc.).
pub mod d3;
/// Error types returned by this crate.
pub mod error;
/// Time-series graph rendering built on top of the D3 bindings.
pub mod graph;
/// Crate-wide [`Result`](result::Result) type alias.
pub mod result;
mod script;

pub use d3::D3;
pub use script::load;
