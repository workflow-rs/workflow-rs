//!
//! `time` module provides re-export of WASM32-compatible `Instant` and provides
//! platform neutral implementations for [`unixtime_as_millis_u128()`] and
//! [`unixtime_as_millis_f64()`].
//!

use cfg_if::cfg_if;

/// re-export of [`instant`] crate supporting native and WASM implementations
pub use instant::*;

pub const SECONDS: u64 = 1000;
pub const MINUTES: u64 = SECONDS * 60;
pub const HOURS: u64 = MINUTES * 60;
pub const DAYS: u64 = HOURS * 24;

pub enum TimeFormat {
    Time24,
    Time12,
    Locale,
    Custom(String),
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use js_sys::{Date,Intl,Reflect};
        use wasm_bindgen::prelude::JsValue;

        #[inline(always)]
        pub fn unixtime_as_millis_u128() -> u128 {
            Date::now() as u128
        }

        #[inline(always)]
        pub fn unixtime_as_millis_f64() -> f64 {
            Date::now()
        }

        #[inline(always)]
        pub fn unixtime_as_millis_u64() -> u64 {
            Date::now() as u64
        }

        #[inline(always)]
        pub fn unixtime_to_locale_string(unixtime : u64) -> String {
            let date = Date::new(&JsValue::from(unixtime as f64));
            date.to_locale_string(default_locale().as_str(), &JsValue::UNDEFINED).as_string().unwrap()
        }

        fn default_locale() -> String {
            static mut LOCALE: Option<String> = None;
            unsafe {
                LOCALE.get_or_insert_with(|| {
                    let date_time_format = Intl::DateTimeFormat::default();
                    let resolved_options = date_time_format.resolved_options();
                    let locale = Reflect::get(&resolved_options, &JsValue::from("locale")).expect("Intl::DateTimeFormat().resolvedOptions().locale is not defined");
                    locale.as_string().expect("Intl::DateTimeFormat().resolvedOptions().locale()")
                }).clone()
            }
        }

        pub fn init_desired_time_format(_time_format : TimeFormat) {
            // time format is ignored in WASM and
            // the browser's locale is used instead
        }

    } else {
        // use time::OffsetDateTime;
        use chrono::{Local, TimeZone};

        #[inline(always)]
        pub fn unixtime_as_millis_u128() -> u128 {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("unixtime_as_millis_u64").as_millis()
        }

        #[inline(always)]
        pub fn unixtime_as_millis_f64() -> f64 {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("unixtime_as_millis_u64").as_millis() as f64
        }

        #[inline(always)]
        pub fn unixtime_as_millis_u64() -> u64 {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("unixtime_as_millis_u64").as_millis() as u64
        }

        static mut TIME_FORMAT: Option<String> = None;

        #[inline(always)]
        fn time_format() -> &'static str {
            unsafe {
                TIME_FORMAT.get_or_insert_with(|| {
                    "%Y-%m-%d %H:%M:%S".to_string()
                }).as_str()
            }
        }

        pub fn init_desired_time_format(time_format : TimeFormat) {
            unsafe {
                match time_format {
                    TimeFormat::Time24 => {
                        TIME_FORMAT = Some("%Y-%m-%d %H:%M:%S".to_string());
                    },
                    TimeFormat::Time12 => {
                        TIME_FORMAT = Some("%Y-%m-%d %I:%M:%S %p".to_string());
                    },
                    TimeFormat::Locale => {
                        TIME_FORMAT = Some("%c".to_string());
                    },
                    TimeFormat::Custom(format) => {
                        TIME_FORMAT = Some(format);
                    }
                }
            }
        }

        #[inline(always)]
        pub fn unixtime_to_locale_string(unixtime : u64) -> String {
            let local = Local.timestamp_millis_opt(unixtime as i64).unwrap();
            local.format(time_format()).to_string()
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unixtime_to_locale_string() {
        let now = unixtime_as_millis_u64();
        let locale_string = unixtime_to_locale_string(now);
        println!("locale_string: {}", locale_string);
    }
}
*/
