//! Umbrella crate that re-exports the individual `workflow-rs` crates as
//! feature-gated submodules, letting consumers depend on a single crate and
//! enable only the subsystems they need.

#[cfg(feature = "core")]
/// Core async runtime, time, and utility primitives, re-exported from the
/// `workflow-core` crate.
pub mod core {
    pub use workflow_core::*;
}

#[cfg(feature = "dom")]
pub mod dom {
    pub use workflow_dom::*;
}

#[cfg(feature = "html")]
pub mod html {
    pub use workflow_html::*;
}

#[cfg(feature = "i18n")]
pub mod i18n {
    pub use workflow_i18n::*;
}

#[cfg(feature = "log")]
/// Logging facilities, re-exported from the `workflow-log` crate.
pub mod log {
    pub use workflow_log::*;
}

#[cfg(feature = "node")]
pub mod node {
    pub use workflow_node::*;
}

#[cfg(feature = "nw")]
pub mod nw {
    pub use workflow_nw::*;
}

#[cfg(feature = "panic-hook")]
pub mod panic_hook {
    pub use workflow_panic_hook::*;
}

#[cfg(feature = "rpc")]
pub mod rpc {
    pub use workflow_rpc::*;
}

#[cfg(feature = "store")]
pub mod store {
    pub use workflow_store::*;
}

#[cfg(feature = "terminal")]
pub mod terminal {
    pub use workflow_terminal::*;
}

#[cfg(feature = "wasm")]
pub mod wasm {
    pub use workflow_wasm::*;
}

#[cfg(feature = "websocket")]
pub mod websocket {
    pub use workflow_websocket::*;
}
