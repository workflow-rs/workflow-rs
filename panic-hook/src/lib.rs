//! # `console_error_panic_hook`
//! 
//! [<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-panic-hook.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-panic-hook)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--panic--hook-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/workflow-panic-hook)
//! <img alt="license" src="https://img.shields.io/crates/l/workflow-panic-hook.svg?maxAge=2592000&color=6ac&style=for-the-badge&logo=opensourceinitiative&logoColor=fff" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
//! <img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">
//!
//! This crate is based on [console_error_panic_hook](https://crates.io/crates/console_error_panic_hook) but
//! provides two configuration modes - console output and full-page output, where the panic will create
//! a full-screen DIV element dumping the stack info in it.  This is useful when debugging on devices
//! without access to console output.
//! 
//! This crate lets you debug panics on `wasm32-unknown-unknown` by providing a
//! panic hook that forwards panic messages to
//! [`console.error`](https://developer.mozilla.org/en-US/docs/Web/API/Console/error).
//!
//! When an error is reported with `console.error`, browser devtools and node.js
//! will typically capture a stack trace and display it with the logged error
//! message.
//!
//! Without `console_error_panic_hook` you just get something like *RuntimeError: Unreachable executed*
//!
//! Browser:
//! ![Console without panic hook](without_panic_hook.png)
//!
//! Node:
//! ![Node console without panic hook](without_panic_hook_node.png)
//!
//! With this panic hook installed you will see the panic message
//!
//! Browser:
//! ![Console with panic hook set up](with_panic_hook.png)
//!
//! Node:
//! ![Node console with panic hook set up](with_panic_hook_node.png)
//!
//! ## Usage
//!
//! There are two ways to install this panic hook.
//!
//! First, you can set the hook yourself by calling `std::panic::set_hook` in
//! some initialization function:
//!
//! ```
//! extern crate console_error_panic_hook;
//! use std::panic;
//!
//! fn my_init_function() {
//!     panic::set_hook(Box::new(console_error_panic_hook::hook));
//!
//!     // ...
//! }
//! ```
//!
//! Alternatively, use `set_once` on some common code path to ensure that
//! `set_hook` is called, but only the one time. Under the hood, this uses
//! `std::sync::Once`.
//!
//! ```
//! extern crate console_error_panic_hook;
//!
//! struct MyBigThing;
//!
//! impl MyBigThing {
//!     pub fn new() -> MyBigThing {
//!         console_error_panic_hook::set_once();
//!
//!         MyBigThing
//!     }
//! }
//! ```
//!
//! ## Error.stackTraceLimit
//!
//! Many browsers only capture the top 10 frames of a stack trace. In rust programs this is less likely to be enough. To see more frames, you can set the non-standard value `Error.stackTraceLimit`. For more information see the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Microsoft_Extensions/Error.stackTraceLimit) or [v8 docs](https://v8.dev/docs/stack-trace-api).
//!

#[macro_use]
extern crate cfg_if;

use std::panic;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate wasm_bindgen;
        use wasm_bindgen::prelude::*;
        mod logger;

        #[wasm_bindgen]
        extern {
            #[wasm_bindgen(js_namespace = console, js_name="error")]
            fn console_error(msg: String);

            type Error;

            #[wasm_bindgen(constructor)]
            fn new() -> Error;

            #[wasm_bindgen(structural, method, getter)]
            fn stack(error: &Error) -> String;
        }

        fn process(info: &panic::PanicInfo) -> String{
            let mut msg = info.to_string();

            // Add the error stack to our message.
            //
            // This ensures that even if the `console` implementation doesn't
            // include stacks for `console.error`, the stack is still available
            // for the user. Additionally, Firefox's console tries to clean up
            // stack traces, and ruins Rust symbols in the process
            // (https://bugzilla.mozilla.org/show_bug.cgi?id=1519569) but since
            // it only touches the logged message's associated stack, and not
            // the message's contents, by including the stack in the message
            // contents we make sure it is available to the user.
            msg.push_str("\n\nStack:\n\n");
            let e = Error::new();
            let stack = e.stack();
            msg.push_str(&stack);

            // Safari's devtools, on the other hand, _do_ mess with logged
            // messages' contents, so we attempt to break their heuristics for
            // doing that by appending some whitespace.
            // https://github.com/rustwasm/console_error_panic_hook/issues/7
            msg.push_str("\n\n");

            msg
        }


        fn console_hook(info: &panic::PanicInfo){
            // Finally, log the panic with `console.error`!
            console_error(process(info));
        }
        fn popup_hook(info: &panic::PanicInfo){
            // Finally, log the panic with `logger::error`!
            logger::error(process(info));
        }

        fn init(logger_type:Type){
            match logger_type {
                Type::Console=>{
                    panic::set_hook(Box::new(console_hook));
                }

                Type::Popup=>{
                    logger::init_logger();
                    panic::set_hook(Box::new(popup_hook));
                }
                Type::Native=>{
                    panic!("Native logger not supported under wasm");
                }
            }

        }
        pub use logger::show_logs;
    } else {
        use std::io::{self, Write};

        fn hook(info: &panic::PanicInfo) {
            let _ = writeln!(io::stderr(), "{info}");
        }

        fn init(_logger_type:Type){
            panic::set_hook(Box::new(hook));
        }

        pub fn show_logs(){
            panic!("Native (non-WASM) platform build doesn't support panic logs");
        }
    }
}

pub enum Type {
    Console,
    Popup,
    Native,
}
/// Set the `console.error` panic hook the first time this is called. Subsequent
/// invocations do nothing.
#[inline]
pub fn set_once(logger_type: Type) {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| init(logger_type));
}
