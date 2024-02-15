//!
//! Module containing a helper [`Encoding`] enum use in RPC server constructors.
//!

use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};
use wasm_bindgen::{convert::TryFromJsValue, prelude::*};

/// wRPC protocol encoding: `Borsh` or `JSON`
/// @category Transport
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Encoding {
    Borsh = 0,
    #[serde(rename = "json")]
    JSON = 1,
}

impl Display for Encoding {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Encoding::Borsh => "borsh",
            Encoding::JSON => "json",
        };
        f.write_str(s)
    }
}

impl FromStr for Encoding {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "borsh" => Ok(Encoding::Borsh),
            "json" => Ok(Encoding::JSON),
            "serde-json" => Ok(Encoding::JSON),
            _ => Err(Error::Encoding(
                "invalid encoding: {s} (must be: 'borsh' or 'json')".to_string(),
            )),
        }
    }
}

impl TryFrom<u8> for Encoding {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Encoding::Borsh),
            1 => Ok(Encoding::JSON),
            _ => Err(Error::Encoding(
                "invalid encoding: {value} (must be: Encoding.Borsh (0) or Encoding.JSON (1))"
                    .to_string(),
            )),
        }
    }
}

impl TryFrom<JsValue> for Encoding {
    type Error = Error;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        if let Ok(encoding) = Encoding::try_from_js_value(value.clone()) {
            Ok(encoding)
        } else if let Some(v) = value.as_f64() {
            Ok(Encoding::try_from(v as u8)?)
        } else if let Some(string) = value.as_string() {
            Encoding::from_str(&string)
        } else {
            Err(Error::Encoding(
                "invalid encoding value: {value:?}".to_string(),
            ))
        }
    }
}

const ENCODING: [Encoding; 2] = [Encoding::Borsh, Encoding::JSON];

impl Encoding {
    pub fn iter() -> impl Iterator<Item = &'static Encoding> {
        ENCODING.iter()
    }
}
