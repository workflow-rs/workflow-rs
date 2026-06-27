//! Minimal asynchronous client for the OpenAI chat completions API,
//! providing simple text query and translation helpers.

/// Error types returned by this crate.
pub mod error;
/// Chat client and supported model definitions.
pub mod gpt;
mod imports;
/// Crate-wide `Result` type alias.
pub mod result;

/// Re-exports of the most commonly used items.
pub mod prelude {
    pub use crate::gpt::ChatGPT;
}
