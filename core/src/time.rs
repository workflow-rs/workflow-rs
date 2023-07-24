//!
//! `time` module provides re-export of WASM32-compatible `Instant` and provides
//! platform neutral implementations for [`unixtime_as_millis_u128()`] and
//! [`unixtime_as_millis_f64()`].
//!

use cfg_if::cfg_if;
use js_sys::Date;

/// re-export of [`instant`] crate supporting native and WASM implementations
pub use instant::*;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        #[inline(always)]
        pub fn unixtime_as_millis_u128() -> u128 {
            (Date::now() * 1000.0) as u128
        }

        #[inline(always)]
        pub fn unixtime_as_millis_f64() -> f64 {
            Date::now()
        }

    } else {
        #[inline(always)]
        pub fn unixtime_as_millis_u128() -> u128 {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("unixtime_as_millis_u64").as_millis()
        }

        #[inline(always)]
        pub fn unixtime_as_millis_f64() -> f64 {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("unixtime_as_millis_u64").as_millis() as f64
        }
    }
}
