//! [`Result`] enum encapsulating the node [`Error`](crate::error::Error) enum

pub type Result<T> = std::result::Result<T, crate::error::Error>;
