use crate::terminal::Terminal;
use std::sync::Arc;
use workflow_log::style;

/// Implementation backing the `tprint!` macro; writes `args` to the
/// terminal without a trailing newline.
#[inline(always)]
pub fn tprint_impl<T>(term: T, args: &str)
where
    T: Into<Arc<Terminal>>,
{
    term.into().write(args);
}

/// Implementation backing the `tprintln!` macro; writes `args` to the
/// terminal followed by a newline.
#[inline(always)]
pub fn tprintln_impl<T>(term: T, args: &str)
where
    T: Into<Arc<Terminal>>,
{
    term.into().writeln(args);
}

/// Implementation backing the `terrorln!` macro; writes `args` to the
/// terminal styled in red followed by a newline.
#[inline(always)]
pub fn terrorln_impl<T>(term: T, args: &str)
where
    T: Into<Arc<Terminal>>,
{
    term.into()
        .writeln(style(args.to_string()).red().to_string());
}

/// Implementation backing the `twarnln!` macro; writes `args` to the
/// terminal styled in yellow followed by a newline.
#[inline(always)]
pub fn twarnln_impl<T>(term: T, args: &str)
where
    T: Into<Arc<Terminal>>,
{
    term.into()
        .writeln(style(args.to_string()).yellow().to_string());
}

/// Implementation backing the `tpara!` macro; writes `args` to the
/// terminal as a word-wrapped paragraph.
#[inline(always)]
pub fn tpara_impl<T>(term: T, args: &str)
where
    T: Into<Arc<Terminal>>,
{
    term.into().para(args.to_string());
}

/// Write a formatted warning line (styled in yellow) to the given terminal.
/// Usage: `twarnln!(term, "format {}", value)`.
#[macro_export]
macro_rules! twarnln {
    ($target:expr_2021) => {
        compile_error!("twarnln! macro requires at least two arguments");
    };

    ($dest:expr_2021, $($arg:tt)*) => {
        $crate::twarnln_impl($dest.deref().clone(), &format_args!($($arg)*).to_string().as_str())
    };
}

/// Write a formatted error line (styled in red) to the given terminal.
/// Usage: `terrorln!(term, "format {}", value)`.
#[macro_export]
macro_rules! terrorln {
    ($target:expr_2021) => {
        compile_error!("terrorln! macro requires at least two arguments");
    };

    ($dest:expr_2021, $($arg:tt)*) => {
        $crate::terrorln_impl($dest.deref().clone(), &format_args!($($arg)*).to_string().as_str())
    };
}

/// Write a formatted line followed by a newline to the given terminal.
/// Usage: `tprintln!(term)` or `tprintln!(term, "format {}", value)`.
#[macro_export]
macro_rules! tprintln {
    ($dest:expr_2021) => {
        $crate::tprintln_impl($dest.as_ref(), &"")
    };

    ($dest:expr_2021, $($arg:tt)*) => {
        $crate::tprintln_impl($dest.deref().clone(), &format_args!($($arg)*).to_string().as_str())
    };
}

/// Write formatted text without a trailing newline to the given terminal.
/// Usage: `tprint!(term)` or `tprint!(term, "format {}", value)`.
#[macro_export]
macro_rules! tprint {
    ($dest:expr_2021) => {
        $crate::tprint_impl($dest.as_ref(), &"")
    };

    ($dest:expr_2021, $($arg:tt)*) => {
        $crate::tprint_impl($dest.deref().clone(), &format_args!($($arg)*).to_string().as_str())
    };
}

/// Write formatted text to the given terminal as a word-wrapped paragraph.
/// Usage: `tpara!(term, "format {}", value)`.
#[macro_export]
macro_rules! tpara {
    ($target:expr_2021) => {
        compile_error!("tpara! macro requires at least two arguments");
    };

    ($dest:expr_2021, $($arg:tt)*) => {
        $crate::tpara_impl($dest.as_ref(), &format_args!($($arg)*).to_string().as_str())
    };
}
