use thiserror::Error;
use workflow_core::channel::{ChannelError, RecvError, SendError, TryRecvError, TrySendError};

/// Errors produced by the service runtime and its channel operations.
#[derive(Error, Debug)]
pub enum Error {
    /// An arbitrary, caller-supplied error message.
    #[error("Error: {0}")]
    Custom(String),

    /// A wrapped standard library I/O error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A channel `send()` operation failed.
    #[error("Channel send() error")]
    SendError,

    /// A channel `recv()` operation failed.
    #[error("Channel recv() error")]
    RecvError,

    /// A channel `try_send()` operation failed.
    #[error("Channel try_send() error")]
    TrySendError,

    /// A channel `try_recv()` operation failed.
    #[error("Channel try_recv() error")]
    TryRecvError,

    /// A generic channel error carrying its description.
    #[error("Channel error: {0}")]
    ChannelError(String),
}

impl Error {
    /// Creates an [`Error::Custom`] from any displayable message.
    pub fn custom<S: std::fmt::Display>(msg: S) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Error::SendError
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(_: TrySendError<T>) -> Self {
        Error::TrySendError
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Error::RecvError
    }
}

impl From<TryRecvError> for Error {
    fn from(_: TryRecvError) -> Self {
        Error::TryRecvError
    }
}

impl<T> From<ChannelError<T>> for Error {
    fn from(err: ChannelError<T>) -> Self {
        Error::ChannelError(err.to_string())
    }
}
