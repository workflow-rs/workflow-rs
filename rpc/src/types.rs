//!
//! Trait constraints for RPC methods (Ops) and message (Req,Resp,Msg).
//!

use crate::imports::*;

/// Marker trait bundling the bounds required of an RPC operation (method) id
/// type: it must be hashable and comparable for dispatch, serializable via both
/// Borsh and Serde, and shareable across threads.
pub trait OpsT:
    Debug
    + Clone
    + Eq
    + Hash
    + BorshSerialize
    + BorshDeserialize
    + Serialize
    + DeserializeOwned
    + Send
    + Sync
    + 'static
{
}
impl<T> OpsT for T where
    T: Debug
        + Clone
        + Eq
        + Hash
        + BorshSerialize
        + BorshDeserialize
        + Serialize
        + DeserializeOwned
        + Send
        + Sync
        + 'static
{
}

/// Marker trait bundling the bounds required of an RPC message payload type
/// (request, response or notification): it must be serializable via both Borsh
/// and Serde and shareable across threads.
pub trait MsgT:
    BorshSerialize + BorshDeserialize + Serialize + DeserializeOwned + Send + Sync + 'static
{
}

impl<T> MsgT for T where
    T: BorshSerialize + BorshDeserialize + Serialize + DeserializeOwned + Send + Sync + 'static
{
}
