//! Assorted helper utilities shared across the `workflow-rs` ecosystem,
//! including error/result types, interactive CLI actions, command-line
//! argument helpers, formatting, public IP lookup and semantic version
//! parsing.

/// Crate-wide error type.
pub mod error;
mod imports;
/// Crate-wide [`Result`](result::Result) type alias.
pub mod result;

/// Trait for defining interactive, selectable CLI actions.
pub mod action;
/// Builder for assembling de-duplicated command-line argument lists.
pub mod arglist;
/// Human-readable value formatting helpers.
pub mod format;
/// Helpers for resolving the host's public IP address.
pub mod ip;
/// Semantic version parsing, comparison and crates.io lookup.
pub mod version;

/// Re-exports of the most commonly used modules for convenient glob imports.
pub mod prelude {
    pub use crate::action;
    pub use crate::arglist;
    pub use crate::format;
    pub use crate::ip;
    pub use crate::version;
}
