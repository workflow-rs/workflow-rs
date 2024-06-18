//!
//! Trait constraints for RPC methods (Ops) and message (Req,Resp,Msg).
//!

use crate::imports::*;

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

pub trait MsgT:
    BorshSerialize + BorshDeserialize + Serialize + DeserializeOwned + Send + Sync + 'static
{
}

impl<T> MsgT for T where
    T: BorshSerialize + BorshDeserialize + Serialize + DeserializeOwned + Send + Sync + 'static
{
}
