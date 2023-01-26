///
/// This enum is a replica of the Level and LevelFilter
/// enums from the log crate : https://crates.io/crates/log
///
/// The original references can be found here:
/// - https://docs.rs/log/latest/log/enum.Level.html
/// - https://docs.rs/log/latest/log/enum.LevelFilter.html
///
/// This is embedded directly into this crate to to allow
/// usage of log levels while retaining the ability to embed
/// the log crate itself as an optional feature. This is done
/// to accomodate reduced bytecode requirements of build
/// targets such as BPF.
///
use std::cmp;

#[repr(usize)]
#[derive(Copy, Eq, Debug)]
pub enum Level {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Clone for Level {
    #[inline]
    fn clone(&self) -> Level {
        *self
    }
}

impl PartialEq for Level {
    #[inline]
    fn eq(&self, other: &Level) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialEq<LevelFilter> for Level {
    #[inline]
    fn eq(&self, other: &LevelFilter) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialOrd for Level {
    #[inline]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Level) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &Level) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &Level) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &Level) -> bool {
        *self as usize >= *other as usize
    }
}

impl PartialOrd<LevelFilter> for Level {
    #[inline]
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some((*self as usize).cmp(&(*other as usize)))
    }

    #[inline]
    fn lt(&self, other: &LevelFilter) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &LevelFilter) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &LevelFilter) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &LevelFilter) -> bool {
        *self as usize >= *other as usize
    }
}

impl Ord for Level {
    #[inline]
    fn cmp(&self, other: &Level) -> cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

#[repr(usize)]
#[derive(Copy, Eq, Debug)]
pub enum LevelFilter {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}

// Deriving generates terrible impls of these traits

impl Clone for LevelFilter {
    #[inline]
    fn clone(&self) -> LevelFilter {
        *self
    }
}

impl PartialEq for LevelFilter {
    #[inline]
    fn eq(&self, other: &LevelFilter) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialEq<Level> for LevelFilter {
    #[inline]
    fn eq(&self, other: &Level) -> bool {
        other.eq(self)
    }
}

impl PartialOrd for LevelFilter {
    #[inline]
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &LevelFilter) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &LevelFilter) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &LevelFilter) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &LevelFilter) -> bool {
        *self as usize >= *other as usize
    }
}

impl PartialOrd<Level> for LevelFilter {
    #[inline]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some((*self as usize).cmp(&(*other as usize)))
    }

    #[inline]
    fn lt(&self, other: &Level) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &Level) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &Level) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &Level) -> bool {
        *self as usize >= *other as usize
    }
}

impl Ord for LevelFilter {
    #[inline]
    fn cmp(&self, other: &LevelFilter) -> cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}
