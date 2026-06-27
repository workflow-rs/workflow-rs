// pub mod broadcast;
/// Error types for the IPC subsystem.
pub mod error;
pub mod id;
/// Common imports shared across the IPC modules.
pub mod imports;
/// Core IPC dispatcher and message handling.
#[allow(clippy::module_inception)]
pub mod ipc;
/// Wire message framing and (de)serialization.
pub mod messages;
pub mod method;
pub mod notification;
/// `Result` and `ResponseResult` type aliases for the IPC subsystem.
pub mod result;
/// IPC target window handles used to address messages.
pub mod target;
/// Shared trait bounds and type definitions for IPC operations.
pub mod types;

pub use error::ResponseError;
pub use ipc::{Ipc, IpcDispatch, get_ipc_target};
pub use method::Method;
pub use notification::Notification;
pub use result::ResponseResult;
pub use target::*;
