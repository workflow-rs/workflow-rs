//! Cryptographic primitives for the workflow-rs framework, providing
//! authenticated encryption, password-based key derivation, hashing and a
//! self-zeroizing [`Secret`](crate::secret::Secret) container.

mod imports;

/// Authenticated encryption and decryption using the `XChaCha20Poly1305` cipher.
pub mod chacha20poly1305;
/// Error types produced by this crate.
pub mod error;
/// Hashing and Argon2-based key derivation helpers.
pub mod hash;
/// Crate-specific [`Result`](crate::result::Result) alias.
pub mod result;
/// Self-zeroizing container for sensitive data.
pub mod secret;

/// Commonly used re-exports for convenient glob importing.
pub mod prelude {
    pub use crate::chacha20poly1305;
    pub use crate::hash::*;
    pub use crate::secret::Secret;
}
