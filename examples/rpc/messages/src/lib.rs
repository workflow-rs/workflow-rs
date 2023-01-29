use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// RPC operations (methods and notifications)
#[derive(
    Clone, Debug, Eq, PartialEq, Hash, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
pub enum TestOps {
    Notify,
    EvenOdd,
    Increase,
}

/// Request messages
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TestReq {
    pub v: u64,
}

/// Response messages
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum TestResp {
    Even(u64),
    Odd(u64),
    Increase(u64),
}

/// Notification messages
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum TestNotify {
    Seq(u64),
}
