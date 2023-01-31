//!
//! Module containing a helper [`Encoding`] enum use in RPC server constructors.
//!

use std::{fmt::{Debug, Display, Formatter}, str::FromStr};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

/// RPC protocol encoding: `Borsh` or `SerdeJson`
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Encoding {
    Borsh,
    SerdeJson,
}

impl Display for Encoding {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Encoding::Borsh => "Borsh",
            Encoding::SerdeJson => "SerdeJson",
        };
        f.write_str(s)
    }
}

impl FromStr for Encoding {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "borsh" => Ok(Encoding::Borsh),
            "json" => Ok(Encoding::SerdeJson),
            "serde-json" => Ok(Encoding::SerdeJson),
            _ => Err(format!("invalid encoding: {} (must be: 'borsh' or 'json')", s)),
        }
    }
}