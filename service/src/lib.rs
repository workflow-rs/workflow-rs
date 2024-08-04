cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        pub mod debug;
        pub mod error;
        mod imports;
        pub mod prelude;
        pub mod result;
        pub mod runtime;
        pub mod service;
        pub mod signals;

    } else {
        pub mod prelude { }
    }
}
