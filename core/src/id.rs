//!
//! 64-bit random identifier struct [`Id`] that renders its value as a base58 string
//!

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Base58 decode error: {0}")]
    Base58Decode(#[from] bs58::decode::Error),
    #[error("Invalid buffer size")]
    InvalidBufferSize,
    #[error("Unable to decode id: JsValue must be a string")]
    JsValueNotString,
}

/// 64-bit identifier that renders the value as a base58 string.
/// This struct is useful for general-purpose random id generation
/// for use with DOM elements and for other similar purposes.
#[repr(transparent)]
#[derive(
    Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize,
)]
pub struct Id(pub(crate) [u8; 8]);

impl Id {
    pub fn new() -> Id {
        Id::new_from_slice(&rand::random::<[u8; 8]>())
    }

    pub fn new_from_slice(vec: &[u8]) -> Self {
        Self(<[u8; 8]>::try_from(<&[u8]>::clone(&vec)).expect("Error: invalid slice size for id"))
    }

    pub fn to_bytes(self) -> [u8; 8] {
        self.0
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.to_string()
    }
}

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Id {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > std::mem::size_of::<Id>() * 2 {
            return Err(Error::InvalidBufferSize);
        }
        let vec = bs58::decode(s).into_vec()?;
        if vec.len() != std::mem::size_of::<Id>() {
            Err(Error::InvalidBufferSize)
        } else {
            Ok(Id::new_from_slice(&vec))
        }
    }
}

impl TryFrom<&str> for Id {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Id::from_str(s)
    }
}

impl TryFrom<JsValue> for Id {
    type Error = Error;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let value_str = value.as_string().ok_or(Error::JsValueNotString)?;
        FromStr::from_str(&value_str)
    }
}

impl From<Id> for JsValue {
    fn from(id: Id) -> Self {
        JsValue::from_str(&id.to_string())
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <std::string::String as Deserialize>::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}
