//! Hex serialization traits

use serde::{Deserialize, Deserializer, Serializer};
use std::str;

/// Trait for types that can render themselves as a hexadecimal string.
pub trait ToHex {
    /// Returns the hexadecimal string representation of `self`.
    fn to_hex(&self) -> String;
}

/// `serde` serialize helper that encodes a [`ToHex`] value as a hex string.
pub fn serialize<S, T>(this: T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToHex,
{
    let hex = this.to_hex();
    serializer.serialize_str(&hex)
}

/// Trait for types that can be parsed from a hexadecimal string.
pub trait FromHex: Sized {
    /// Error type returned when hex decoding fails.
    type Error: std::fmt::Display;
    /// Parses `hex_str` into `Self`, returning an error on invalid hex input.
    fn from_hex(hex_str: &str) -> Result<Self, Self::Error>;
}

/// `serde` deserialize helper that decodes a hex string into a [`FromHex`] value.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromHex,
{
    use serde::de::Error;
    let buff: &[u8] = Deserialize::deserialize(deserializer)?;
    T::from_hex(str::from_utf8(buff).unwrap()).map_err(D::Error::custom)
}

/// Little endian format of full slice content
/// (so string lengths are always even).
impl ToHex for &[u8] {
    fn to_hex(&self) -> String {
        // an empty vector is allowed
        if self.is_empty() {
            return "".to_string();
        }

        let mut hex = vec![0u8; self.len() * 2];
        faster_hex::hex_encode(self, hex.as_mut_slice())
            .expect("The output is exactly twice the size of the input");
        let result = unsafe { str::from_utf8_unchecked(&hex) };
        result.to_string()
    }
}

/// Little endian format of full content
/// (so string lengths are always even).
impl ToHex for Vec<u8> {
    fn to_hex(&self) -> String {
        (&**self).to_hex()
    }
}

/// Little endian format of full content
/// (so string lengths must be even).
impl FromHex for Vec<u8> {
    type Error = faster_hex::Error;
    fn from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        // an empty string is allowed
        if hex_str.is_empty() {
            return Ok(vec![]);
        }

        let mut bytes = vec![0u8; hex_str.len() / 2];
        faster_hex::hex_decode(hex_str.as_bytes(), bytes.as_mut_slice())?;
        Ok(bytes)
    }
}
