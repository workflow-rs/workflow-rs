//!
//! `time` module provides re-export of WASM32-compatible `Instant` and provides
//! platform neutral implementations for [`unixtime_as_millis_u128()`] and
//! [`unixtime_as_millis_f64()`].
//!

use cfg_if::cfg_if;

/// re-export of [`instant`] crate supporting native and WASM implementations
pub use web_time::*;

/// Number of milliseconds in one second.
pub const SECONDS: u64 = 1000;
/// Number of milliseconds in one minute.
pub const MINUTES: u64 = SECONDS * 60;
/// Number of milliseconds in one hour.
pub const HOURS: u64 = MINUTES * 60;
/// Number of milliseconds in one day.
pub const DAYS: u64 = HOURS * 24;

/// Selects how unix timestamps are rendered to locale strings.
pub enum TimeFormat {
    /// 24-hour clock format (`%Y-%m-%d %H:%M:%S`).
    Time24,
    /// 12-hour clock format with AM/PM (`%Y-%m-%d %I:%M:%S %p`).
    Time12,
    /// The platform/system locale's default date-time representation.
    Locale,
    /// A custom `chrono`-style format string.
    Custom(String),
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use js_sys::{Date,Intl,Reflect};
        use wasm_bindgen::prelude::JsValue;

        /// Returns the current unix time in milliseconds as a `u128`.
        #[inline(always)]
        pub fn unixtime_as_millis_u128() -> u128 {
            Date::now() as u128
        }

        /// Returns the current unix time in milliseconds as an `f64`.
        #[inline(always)]
        pub fn unixtime_as_millis_f64() -> f64 {
            Date::now()
        }

        /// Returns the current unix time in milliseconds as a `u64`.
        #[inline(always)]
        pub fn unixtime_as_millis_u64() -> u64 {
            Date::now() as u64
        }

        /// Formats a unix time in milliseconds as a human-readable locale string.
        #[inline(always)]
        pub fn unixtime_to_locale_string(unixtime : u64) -> String {
            let date = Date::new(&JsValue::from(unixtime as f64));
            date.to_locale_string(default_locale().as_str(), &JsValue::UNDEFINED).as_string().unwrap()
        }

        fn default_locale() -> String {
            static mut LOCALE: Option<String> = None;
            let locale_ptr = &raw mut LOCALE;
            unsafe {
                (*locale_ptr).get_or_insert_with(|| {
                    let date_time_format = Intl::DateTimeFormat::default();
                    let resolved_options = date_time_format.resolved_options();
                    let locale = Reflect::get(&resolved_options, &JsValue::from("locale")).expect("Intl::DateTimeFormat().resolvedOptions().locale is not defined");
                    locale.as_string().expect("Intl::DateTimeFormat().resolvedOptions().locale()")
                }).clone()
            }
        }

        /// Sets the desired time format used by [`unixtime_to_locale_string`].
        /// On WASM the argument is ignored and the browser's locale is used instead.
        pub fn init_desired_time_format(_time_format : TimeFormat) {
            // time format is ignored in WASM and
            // the browser's locale is used instead
        }

    } else {
        use chrono::{Local, TimeZone};

        /// Returns the current unix time in milliseconds as a `u128`.
        #[inline(always)]
        pub fn unixtime_as_millis_u128() -> u128 {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("unixtime_as_millis_u64").as_millis()
        }

        /// Returns the current unix time in milliseconds as an `f64`.
        #[inline(always)]
        pub fn unixtime_as_millis_f64() -> f64 {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("unixtime_as_millis_u64").as_millis() as f64
        }

        /// Returns the current unix time in milliseconds as a `u64`.
        #[inline(always)]
        pub fn unixtime_as_millis_u64() -> u64 {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("unixtime_as_millis_u64").as_millis() as u64
        }

        static mut TIME_FORMAT: Option<String> = None;

        #[inline(always)]
        fn time_format() -> &'static str {
            let time_format_ptr = &raw mut TIME_FORMAT;
            unsafe {
                (*time_format_ptr).get_or_insert_with(|| {
                    "%Y-%m-%d %H:%M:%S".to_string()
                }).as_str()
            }
        }

        /// Sets the [`TimeFormat`] used by [`unixtime_to_locale_string`] for
        /// subsequent native timestamp formatting.
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

        /// Formats a unix time in milliseconds as a local-time string using the
        /// format configured via [`init_desired_time_format`].
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
