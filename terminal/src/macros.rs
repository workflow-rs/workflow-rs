use crate::terminal::Terminal;
use std::sync::Arc;
use workflow_log::style;

#[inline(always)]
pub fn tprintln_impl<T>(term: T, args: &str)
where
    T: Into<Arc<Terminal>>,
{
    term.into().writeln(args);
}

#[inline(always)]
pub fn terrorln_impl<T>(term: T, args: &str)
where
    T: Into<Arc<Terminal>>,
{
    term.into()
        .writeln(style(args.to_string()).red().to_string());
}

#[inline(always)]
pub fn twarnln_impl<T>(term: T, args: &str)
where
    T: Into<Arc<Terminal>>,
{
    term.into()
        .writeln(style(args.to_string()).yellow().to_string());
}

#[inline(always)]
pub fn tpara_impl<T>(term: T, args: &str)
where
    T: Into<Arc<Terminal>>,
{
    term.into().para(args.to_string());
}

#[macro_export]
macro_rules! twarnln {
    ($target:expr) => {
        compile_error!("twarnln! macro requires at least two arguments");
    };

    ($dest:expr, $($arg:tt)*) => {
        $crate::twarnln_impl($dest.deref().clone(), &format_args!($($arg)*).to_string().as_str())
    };
}

#[macro_export]
macro_rules! terrorln {
    ($target:expr) => {
        compile_error!("terrorln! macro requires at least two arguments");
    };

    ($dest:expr, $($arg:tt)*) => {
        $crate::terrorln_impl($dest.deref().clone(), &format_args!($($arg)*).to_string().as_str())
    };
}

#[macro_export]
macro_rules! tprintln {
    ($dest:expr) => {
        $crate::tprintln_impl($dest.as_ref(), &"")
    };

    ($dest:expr, $($arg:tt)*) => {
        $crate::tprintln_impl($dest.deref().clone(), &format_args!($($arg)*).to_string().as_str())
    };
}

#[macro_export]
macro_rules! tpara {
    ($target:expr) => {
        compile_error!("tpara! macro requires at least two arguments");
    };

    ($dest:expr, $($arg:tt)*) => {
        $crate::tpara_impl($dest.as_ref(), &format_args!($($arg)*).to_string().as_str())
    };
}
