//! Building blocks for [`egui`]/[`eframe`] applications: device/orientation
//! detection, application framing and lifecycle, font registration, async
//! runtime integration and shared error/result types.

/// Device and screen-orientation detection used to adapt layout between
/// mobile/portrait and desktop/landscape presentations.
pub mod device;
/// Crate error type and result alias.
pub mod error;
/// Helpers for registering static (embedded) fonts with egui.
pub mod fonts;
/// Application framing: window/canvas options and the eframe entry point.
pub mod frame;
/// Commonly used internal imports re-exported for use across the crate.
pub mod imports;
/// Pluggable application modules selectable at runtime.
pub mod module;
/// Re-exports of the crate's most commonly used items.
pub mod prelude;
/// Crate result type alias.
pub mod result;
/// Async runtime and event-channel integration for egui applications.
pub mod runtime;

pub use ahash;
pub use eframe;
pub use egui;
