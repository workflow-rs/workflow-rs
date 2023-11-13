pub mod error;
pub mod result;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod wasm;
        pub use wasm::*;
    } else {
        mod native;
        pub use native::*;
    }
}
