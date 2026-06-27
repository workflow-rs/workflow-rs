//! Dedicated wasm-bindgen test crate for the workflow-wasm `Cast` family
//! (the [`CastFromJs`] derive + [`TryCastFromJs`]).
//!
//! This mirrors how rusty-kaspa types such as `PrivateKey`/`PublicKey`
//! (kaspa-wallet-keys) cast JS values into Rust objects: a `#[wasm_bindgen]`
//! struct deriving `CastFromJs` and implementing `TryCastFromJs::try_cast_from`
//! via `Self::resolve(value, || ...)`.
//!
//! The struct is instantiated and exposed to JS as a class instance, then passed
//! back into Rust where the cast must resolve it by reference — exercising the
//! `WasmPtr<WasmRefCell<T>>` reference ABI used by `try_ref_from_abi_safe`.
//!
//! Run with:  `wasm-pack test --node tests/wasm32-cast`

use wasm_bindgen::prelude::*;
use workflow_wasm::convert::{Cast, CastFromJs, TryCastFromJs};
use workflow_wasm::error::Error;

/// Minimal wasm-bindgen-exported struct used to exercise the Cast machinery.
#[derive(Clone, Debug, CastFromJs)]
#[wasm_bindgen]
pub struct CastTarget {
    value: u32,
}

#[wasm_bindgen]
impl CastTarget {
    /// Construct a new `CastTarget` (also the JS `new CastTarget(value)`).
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> CastTarget {
        CastTarget { value }
    }

    /// The wrapped value (JS getter).
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u32 {
        self.value
    }
}

impl TryCastFromJs for CastTarget {
    type Error = Error;

    fn try_cast_from<'a, R>(value: &'a R) -> Result<Cast<'a, Self>, Self::Error>
    where
        R: AsRef<JsValue> + 'a,
    {
        // Resolve a JS reference to an existing `CastTarget`; otherwise build one
        // from a numeric JsValue (as `PrivateKey` builds from a hex string/array).
        Self::resolve(value, || {
            value
                .as_ref()
                .as_f64()
                .map(|n| CastTarget::new(n as u32))
                .ok_or_else(|| Error::custom("not a CastTarget or number"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    /// A `CastTarget` created in Rust and handed to JS as a class instance casts
    /// back to the same Rust object *by reference* — the `WasmPtr<WasmRefCell<T>>`
    /// reference ABI path.
    #[wasm_bindgen_test]
    fn cast_from_instance_resolves_by_reference() {
        let original = CastTarget::new(42);
        let js: JsValue = original.into(); // -> JS class instance
        let cast = CastTarget::try_cast_from(&js).expect("instance should cast");
        assert!(
            matches!(cast, Cast::Ref { .. }),
            "expected a borrowed reference"
        );
        assert_eq!(cast.into_owned().value(), 42);
    }

    /// A non-instance JsValue (a number) falls through to the user `create`
    /// closure and is constructed into an owned value.
    #[wasm_bindgen_test]
    fn cast_from_number_constructs_value() {
        let js = JsValue::from_f64(7.0);
        let cast = CastTarget::try_cast_from(&js).expect("number should construct");
        assert!(
            matches!(cast, Cast::Value { .. }),
            "expected an owned value"
        );
        assert_eq!(cast.into_owned().value(), 7);
    }

    /// `try_owned_from` consumes the cast and yields an owned `CastTarget` for both
    /// a real instance and the constructed-from-number fallback.
    #[wasm_bindgen_test]
    fn try_owned_from_yields_owned_value() {
        let instance: JsValue = CastTarget::new(11).into();
        assert_eq!(CastTarget::try_owned_from(&instance).unwrap().value(), 11);
        assert_eq!(
            CastTarget::try_owned_from(&JsValue::from_f64(5.0))
                .unwrap()
                .value(),
            5
        );
    }

    /// A JsValue that is neither an instance nor convertible must fail *safely* —
    /// return `Err`, never panic (the safe ABI read is guarded by a class check).
    #[wasm_bindgen_test]
    fn cast_from_invalid_fails_safely() {
        let js = JsValue::from_str("not a cast target");
        assert!(CastTarget::try_cast_from(&js).is_err());
        assert!(CastTarget::try_owned_from(&js).is_err());

        // `null`/`undefined` must also fail rather than panic.
        assert!(CastTarget::try_owned_from(&JsValue::NULL).is_err());
        assert!(CastTarget::try_owned_from(&JsValue::UNDEFINED).is_err());
    }
}
