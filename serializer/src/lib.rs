//! Declarative helpers for storing and loading values with
//! [Borsh](https://borsh.io/): serialization macros and length-prefixed payload
//! buffers used to persist and exchange data across the workflow-rs crates.

/// Declarative macros for storing and loading items via Borsh and [`serializer::Serializer`].
pub mod macros;
/// Helper `payload::Payload` wrappers that length-prefix serialized data into a byte buffer.
pub mod payload;
/// Type aliases for the standard library I/O error and result types used throughout the crate.
pub mod result;
/// The [`serializer::Serializer`] and [`serializer::Deserializer`] traits and their implementations.
pub mod serializer;
/// Unit tests exercising the serialization traits and macros.
pub mod tests;

/// Re-exports of the most commonly used traits, macros, and Borsh items.
pub mod prelude {
    pub use crate::serializer::{Deserializer, Serializable, Serializer};
    pub use crate::{deserialize, load, payload, reader, serialize, store, version, writer};
    pub use borsh::{BorshDeserialize, BorshSerialize};
}

pub use borsh;
