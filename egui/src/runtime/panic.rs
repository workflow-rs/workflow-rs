use crate::runtime::Runtime;
use std::backtrace::Backtrace;
use std::fs;
use std::panic;

pub fn init_graceful_panic_handler() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let backtrace = Backtrace::capture();
        // println!("panic! \n{:#?}\n{:#?}", panic_info, backtrace);
        let _ = fs::write(
            "application-panic.log",
            format!("{:#?}\n{:#?}", panic_info, backtrace),
        );
        println!("An unexpected condition (panic) has occurred. Additional information has been written to `application-panic.log`");
        default_hook(panic_info);
        Runtime::abort();
    }));
}

pub fn init_ungraceful_panic_handler() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let backtrace = Backtrace::capture();
        let _ = fs::write(
            "service-panic.log",
            format!("{:#?}\n{:#?}", panic_info, backtrace),
        );
        default_hook(panic_info);
        println!("An unexpected condition (panic) has occurred. Additional information has been written to `service-panic.log`");
        println!("Exiting...");
        std::process::exit(1);
    }));
}
