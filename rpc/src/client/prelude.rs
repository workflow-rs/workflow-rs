//!
//! Convenience module exporting all types required for the client use.
//!
pub use crate::client::{
    BorshProtocol, ConnectOptions, ConnectStrategy, Interface, JsonProtocol,
    Options as RpcClientOptions, RpcClient, notification, result::Result as ClientResult,
};
pub use crate::encoding::Encoding;
