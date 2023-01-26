//!
//! Convenience module exporting all types required for the client use.
//!
pub use crate::client::{
    notification, BorshProtocol, Interface, Options as RpcClientOptions, RpcClient,
    SerdeJsonProtocol,
};
pub use crate::Encoding;
