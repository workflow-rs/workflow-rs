//!
//! Result enum encapsulating [`super::error::Error`]
//! enum common to client and server
//!

pub type Result<T> = std::result::Result<T, super::error::Error>;

// use super::messages::borsh::ServerError;
pub type ServerResult<T> = std::result::Result<T, super::error::ServerError>;
