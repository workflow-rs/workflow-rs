use crate::ipc::imports::*;

/// Marker trait, automatically implemented for any type usable as an IPC
/// operation identifier, bundling the hashing, equality and (de)serialization
/// bounds required to key and transmit operations.
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

/// Marker trait, automatically implemented for any type usable as an IPC
/// message payload (request, response or notification body), bundling the
/// (de)serialization and thread-safety bounds required to transmit it.
pub trait MsgT:
    Debug + BorshSerialize + BorshDeserialize + Serialize + DeserializeOwned + Send + Sync + 'static
{
}

impl<T> MsgT for T where
    T: Debug
        + BorshSerialize
        + BorshDeserialize
        + Serialize
        + DeserializeOwned
        + Send
        + Sync
        + 'static
{
}
