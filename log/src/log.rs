use cfg_if::cfg_if;
use std::fmt;

cfg_if! {
    if #[cfg(target_os = "solana")] {
        pub use workflow_log::levels::{ Level, LevelFilter };
    } else {
        use std::sync::Arc;
        pub use log::{ Level, LevelFilter };
        use downcast::{ downcast_sync, AnySync };
        pub use hexplay::{self, HexViewBuilder};
        pub use termcolor::Buffer;
        //use core::ops::Range;

        pub struct ColorHexView<'a>{
            pub builder: HexViewBuilder<'a>,
            pub color_start: usize
        }
        impl<'a> ColorHexView<'a>{
            pub fn new(builder:HexViewBuilder<'a>, colors:Vec<(&'a str, usize)>)->Self{
                Self{
                    builder,
                    color_start:0
                }.add_colors(colors)
            }

            pub fn add_colors(mut self, colors:Vec<(&'a str, usize)>)->Self{
                let mut builder = self.builder;
                for (color, len) in colors{
                    let end = self.color_start+len;
                    let range = self.color_start..end;
                    self.color_start = end;
                    builder = builder.add_color(color, range);
                }
                self.builder = builder;
                self
            }

            pub fn add_colors_with_range(mut self, colors:Vec<(&'a str, std::ops::Range<usize>)>)->Self{
                let mut builder = self.builder;
                for (color, range) in colors{
                    builder = builder.add_color(color, range);
                }
                self.builder = builder;
                self
            }

            pub fn try_print(self)->std::result::Result<(), String>{
                let mut buf = Buffer::ansi();
                match self.builder.finish().fmt(&mut buf){
                    Ok(()) => {
                        match String::from_utf8(buf.as_slice().to_vec()){
                            Ok(str)=>{
                                log_trace!("{}", str);
                            }
                            Err(_)=>{
                                return Err("Unable to convert HexView to string".to_string());
                            }
                        }
                    },
                    Err(_) => {
                        return Err("Unable to format HexView".to_string());
                    }
                }
                Ok(())
            }
        }

        /// A log sink trait that can be installed into the log subsystem using the [`pipe`]
        /// function and will receive all log messages.
        pub trait Sink : AnySync {
            fn write(&self, target: Option<&str>, level : Level, args : &fmt::Arguments<'_>) -> bool;
        }

        struct SinkHandler {
            // #[allow(dead_code)]
            sink : Arc<dyn Sink>, // + Send + Sync + 'static>,
        }

        downcast_sync!(dyn Sink);
    }
}

cfg_if! {
    if #[cfg(target_os = "solana")] {
        #[inline(always)]
        pub fn log_level_enabled(_level: Level) -> bool {
            true
        }
    } else if #[cfg(target_arch = "wasm32")] {
        static mut LEVEL_FILTER : LevelFilter = LevelFilter::Trace;
        #[inline(always)]
        pub fn log_level_enabled(level: Level) -> bool {
            unsafe { LEVEL_FILTER >= level }
        }
        pub fn set_log_level(level: LevelFilter) {
            unsafe { LEVEL_FILTER = level };
        }
        cfg_if! {
            if #[cfg(feature = "sink")] {
                static mut SINK : Option<SinkHandler> = None;
                // pub fn pipe(sink : Arc<dyn Sink + Send + Sync + 'static>) {
                pub fn pipe(sink : Option<Arc<dyn Sink>>) {
                    match sink {
                        Some(sink) => { unsafe { SINK = Some(SinkHandler { sink }); } },
                        None => { unsafe { SINK = None; } }
                    }
                }
                #[inline(always)]
                fn to_sink(target: Option<&str>, level : Level, args : &fmt::Arguments<'_>) -> bool {
                    let sink = unsafe { &SINK };
                    match sink {
                        Some(handler) => {
                            handler.sink.write(target, level, args)
                        },
                        None => { false }
                    }
                }
            }
        }

    } else {
        use std::sync::Mutex;

        lazy_static::lazy_static! {
            static ref LEVEL_FILTER : Mutex<LevelFilter> = Mutex::new(LevelFilter::Trace);
        }
        #[inline(always)]
        /// Returns true if the current log level is below the
        /// currently set [`LevelFilter`]
        pub fn log_level_enabled(level: Level) -> bool {
            *LEVEL_FILTER.lock().unwrap() >= level
        }
        /// Enable filtering of log messages using the [`LevelFilter`]
        pub fn set_log_level(level: LevelFilter) {
            *LEVEL_FILTER.lock().unwrap() = level;
        }
        cfg_if! {
            if #[cfg(feature = "sink")] {
                lazy_static::lazy_static! {
                    static ref SINK : Mutex<Option<SinkHandler>> = Mutex::new(None);
                }
                // pub fn pipe(sink : Option<Arc<dyn Sink + Send + Sync + 'static>>) {
                /// Receives an Option with an `Arc`ed [`Sink`] trait reference
                /// and installs it as a log sink / receiver.
                /// The sink can be later disabled by invoking `pipe(None)`
                pub fn pipe(sink : Option<Arc<dyn Sink>>) {
                    match sink {
                        Some(sink) => { *SINK.lock().unwrap() = Some(SinkHandler { sink }); },
                        None => { *SINK.lock().unwrap() = None; }
                    }

                }
                #[inline(always)]
                fn to_sink(target : Option<&str>, level : Level, args : &fmt::Arguments<'_>) -> bool {
                    match SINK.lock().unwrap().as_ref() {
                        Some(handler) => {
                            handler.sink.write(target, level, args)
                        },
                        None => { false }
                    }
                }
            }
        }

        #[cfg(feature = "external-logger")]
        mod workflow_logger {
            use log::{ Level, LevelFilter, Record, Metadata, SetLoggerError };

            pub struct WorkflowLogger;

            impl log::Log for WorkflowLogger {
                fn enabled(&self, metadata: &Metadata) -> bool {
                    super::log_level_enabled(metadata.level())
                }

                fn log(&self, record: &Record) {
                    if self.enabled(record.metadata()) {
                        match record.metadata().level() {
                            Level::Error => { super::error_impl(record.args()); },
                            Level::Warn => { super::warn_impl(record.args()); },
                            Level::Info => { super::info_impl(record.args()); },
                            Level::Debug => { super::debug_impl(record.args()); },
                            Level::Trace => { super::trace_impl(record.args()); },
                        }
                    }
                }

                fn flush(&self) {}
            }

            static LOGGER: WorkflowLogger = WorkflowLogger;

            pub fn init() -> Result<(), SetLoggerError> {
                log::set_logger(&LOGGER)
                    .map(|()| log::set_max_level(LevelFilter::Trace))
            }
        }

        #[cfg(feature = "external-logger")]
        pub fn init() -> Result<(), log::SetLoggerError> {
            workflow_logger::init()
        }

    }
}

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use wasm_bindgen::prelude::*;
    // use super::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
        #[wasm_bindgen(js_namespace = console)]
        pub fn warn(s: &str);
        #[wasm_bindgen(js_namespace = console)]
        pub fn error(s: &str);
    }
}

pub mod impls {
    use super::*;

    #[inline(always)]
    pub fn error_impl(target: Option<&str>, args: &fmt::Arguments<'_>) {
        if log_level_enabled(Level::Error) {
            #[cfg(all(not(target_os = "solana"), feature = "sink"))]
            {
                if to_sink(target, Level::Error, args) {
                    return;
                }
            }
            cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    workflow_log::wasm::error(&args.to_string());
                } else if #[cfg(target_os = "solana")] {
                    solana_program::log::sol_log(&args.to_string());
                } else {
                    println!("{args}");
                }
            }
        }
    }

    #[inline(always)]
    pub fn warn_impl(target: Option<&str>, args: &fmt::Arguments<'_>) {
        if log_level_enabled(Level::Warn) {
            #[cfg(all(not(target_os = "solana"), feature = "sink"))]
            {
                if to_sink(target, Level::Warn, args) {
                    return;
                }
            }
            cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    workflow_log::wasm::warn(&args.to_string());
                } else if #[cfg(target_os = "solana")] {
                    solana_program::log::sol_log(&args.to_string());
                } else {
                    println!("{args}");
                }
            }
        }
    }

    #[inline(always)]
    pub fn info_impl(target: Option<&str>, args: &fmt::Arguments<'_>) {
        if log_level_enabled(Level::Info) {
            #[cfg(all(not(target_os = "solana"), feature = "sink"))]
            {
                if to_sink(target, Level::Info, args) {
                    return;
                }
            }
            cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    workflow_log::wasm::log(&args.to_string());
                } else if #[cfg(target_os = "solana")] {
                    solana_program::log::sol_log(&args.to_string());
                } else {
                    println!("{args}");
                }
            }
        }
    }

    #[inline(always)]
    pub fn debug_impl(target: Option<&str>, args: &fmt::Arguments<'_>) {
        if log_level_enabled(Level::Debug) {
            #[cfg(all(not(target_os = "solana"), feature = "sink"))]
            {
                if to_sink(target, Level::Debug, args) {
                    return;
                }
            }
            cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    workflow_log::wasm::log(&args.to_string());
                } else if #[cfg(target_os = "solana")] {
                    solana_program::log::sol_log(&args.to_string());
                } else {
                    println!("{args}");
                }
            }
        }
    }

    #[inline(always)]
    pub fn trace_impl(target: Option<&str>, args: &fmt::Arguments<'_>) {
        if log_level_enabled(Level::Trace) {
            #[cfg(all(not(target_os = "solana"), feature = "sink"))]
            {
                if to_sink(target, Level::Trace, args) {
                    return;
                }
            }
            cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    workflow_log::wasm::log(&args.to_string());
                } else if #[cfg(target_os = "solana")] {
                    solana_program::log::sol_log(&args.to_string());
                } else {
                    println!("{args}");
                }
            }
        }
    }
}

/// Format and log message with [`Level::Error`]
#[macro_export]
macro_rules! log_error {
    (target: $target:expr, $($arg:tt)+) => (
        workflow_log::impls::error_impl(Some($target),&format_args!($($t)*))
    );

    ($($t:tt)*) => (
        workflow_log::impls::error_impl(None,&format_args!($($t)*))
    )
}

/// Format and log message with [`Level::Warn`]
#[macro_export]
macro_rules! log_warning {
    (target: $target:expr, $($arg:tt)+) => (
        workflow_log::impls::warn_impl(Some($target),&format_args!($($t)*))
    );

    ($($t:tt)*) => (
        workflow_log::impls::warn_impl(None,&format_args!($($t)*))
    )
}

/// Format and log message with [`Level::Info`]
#[macro_export]
macro_rules! log_info {
    (target: $target:expr, $($arg:tt)+) => (
        workflow_log::impls::info_impl(Some($target),&format_args!($($t)*))
    );

    ($($t:tt)*) => (
        workflow_log::impls::info_impl(None,&format_args!($($t)*))
    )
}

/// Format and log message with [`Level::Debug`]
#[macro_export]
macro_rules! log_debug {
    (target: $target:expr, $($arg:tt)+) => (
        workflow_log::impls::debug_impl(Some($target),&format_args!($($t)*))
    );

    ($($t:tt)*) => (
        workflow_log::impls::debug_impl(None,&format_args!($($t)*))
    )
}

/// Format and log message with [`Level::Trace`]
#[macro_export]
macro_rules! log_trace {
    (target: $target:expr, $($arg:tt)+) => (
        workflow_log::impls::trace_impl(Some($target),&format_args!($($t)*))
    );

    ($($t:tt)*) => (
        workflow_log::impls::trace_impl(None,&format_args!($($t)*))
    )
}

use log_debug;
use log_error;
use log_info;
use log_trace;
use log_warning;

/// Prints (using [`log_trace`]) a data slice
/// formatted as a hex data dump.
#[cfg(not(target_os = "solana"))]
pub fn trace_hex(data: &[u8]) {
    let hex = format_hex(data);
    log_trace!("{}", hex);
}

/// Returns a string formatted as a hex data dump
/// of the supplied slice argument.
#[cfg(not(target_os = "solana"))]
pub fn format_hex(data: &[u8]) -> String {
    let view = hexplay::HexViewBuilder::new(data)
        .address_offset(0)
        .row_width(16)
        .finish();

    format!("{view}")
}

/// Formats a hex data dump to contain color ranges
#[cfg(not(target_os = "solana"))]
pub fn format_hex_with_colors<'a>(
    data: &'a [u8],
    colors: Vec<(&'a str, usize)>,
) -> ColorHexView<'a> {
    let view_builder = hexplay::HexViewBuilder::new(data)
        .address_offset(0)
        .row_width(16);

    ColorHexView::new(view_builder, colors)
}
#[cfg(not(target_os = "solana"))]
pub mod color_log {
    pub use super::*;
    pub type Index = usize;
    pub type Length = usize;
    pub type Color<'a> = &'a str;
    type Result<T> = std::result::Result<T, String>;
    pub trait ColoLogTrace {
        fn log_data(&self) -> Vec<u8>;
        fn log_index_length_color(&self) -> Option<Vec<(Index, Length, Color)>> {
            None
        }

        fn log_trace(&self) -> Result<bool> {
            let data_vec = self.log_data();
            let mut view = format_hex_with_colors(&data_vec, vec![]);
            if let Some(index_length_color) = self.log_index_length_color() {
                let mut colors = Vec::new();
                for (index, length, color) in index_length_color {
                    colors.push((color, index..index + length));
                }
                view = view.add_colors_with_range(colors);
            }

            if view.try_print().is_err() {
                trace_hex(&data_vec);
                return Ok(false);
            }
            Ok(true)
        }
    }
}

#[cfg(not(target_os = "solana"))]
pub use color_log::*;
