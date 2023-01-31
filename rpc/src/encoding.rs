//!
//! Module containing a helper [`Encoding`] enum use in RPC server constructors.
//!

use std::fmt::{Debug, Display, Formatter};
use wasm_bindgen::prelude::*;

/// Helper enum representing the protocol encoding: `Borsh` or `SerdeJson`
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
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
