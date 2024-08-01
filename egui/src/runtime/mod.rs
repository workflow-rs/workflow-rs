cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        pub mod signals;
        pub mod panic;
    } else {
        // ...
    }
}

pub mod channel;
pub mod events;
pub mod payload;
mod repaint;
pub mod service;

#[allow(clippy::module_inception)]
mod runtime;
pub use runtime::*;
