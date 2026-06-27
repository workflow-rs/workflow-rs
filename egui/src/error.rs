use thiserror::Error;
use workflow_core::channel::{RecvError, SendError, TrySendError};

/// Boxed, thread-safe dynamic error type used for opaque error propagation.
pub type DynError = Box<dyn std::error::Error + Send + Sync>;

/// Errors produced by this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// A custom, free-form error message.
    #[error("{0}")]
    Custom(String),

    /// An error originating from `eframe`, captured as a string.
    #[error("{0}")]
    Eframe(String),

    /// A wrapped dynamic error from another source.
    #[error(transparent)]
    DynError(#[from] DynError),

    /// A channel `send()` operation failed.
    #[error("Channel send() error")]
    SendError,

    /// A channel `try_send()` operation failed.
    #[error("Channel try_send() error")]
    TrySendError,

    /// A channel `recv()` operation failed.
    #[error("Channel recv() error")]
    RecvError,
}

impl Error {
    /// Construct an [`Error::Custom`] from any value convertible into a `String`.
    pub fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Custom(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Custom(err)
    }
}

impl From<eframe::Error> for Error {
    fn from(err: eframe::Error) -> Self {
        Error::Eframe(err.to_string())
    }
}

// impl From<std::io::Error> for Error {
//     fn from(err: std::io::Error) -> Self {
//         Error::Io(err)
//     }
// }

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Error::SendError
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Error::RecvError
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(_: TrySendError<T>) -> Self {
        Error::TrySendError
    }
}
