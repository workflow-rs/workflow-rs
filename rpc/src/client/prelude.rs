//!
//! Convenience module exporting all types required for the client use.
//!
pub use crate::client::{
    notification, result::Result as ClientResult, BorshProtocol, ConnectOptions, ConnectStrategy,
    Interface, Options as RpcClientOptions, RpcClient, SerdeJsonProtocol,
};
pub use crate::encoding::Encoding;
