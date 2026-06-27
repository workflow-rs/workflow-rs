//! Errors produced by the the [`node`](super) crate
use thiserror::Error;
use wasm_bindgen::prelude::*;
use workflow_core::channel::{RecvError, SendError, TryRecvError};
use workflow_wasm::printable::Printable;

/// Errors produced by the [`node`](crate) crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A process or task was started while it was already running.
    #[error("Already running")]
    AlreadyRunning,
    /// An operation requiring a running task was attempted while it was stopped.
    #[error("The task is not running")]
    NotRunning,
    /// The underlying child process reference is missing.
    #[error("Child process reference is absent")]
    ProcIsAbsent,
    /// Failure delivering a value over a channel.
    #[error("{0:?}")]
    Send(String),
    /// Failure receiving a value from a channel.
    #[error("{0:?}")]
    Recv(#[from] RecvError),
    /// Failure on a non-blocking channel receive attempt.
    #[error("{0:?}")]
    TryRecv(#[from] TryRecvError),
    /// Error propagated from the [`workflow_task`] task framework.
    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),
    /// Error propagated from a WASM [`callback`](workflow_wasm::callback).
    #[error(transparent)]
    Callback(#[from] workflow_wasm::callback::CallbackError),
    /// A JavaScript value thrown from interop, wrapped for display.
    #[error("{0}")]
    JsValue(Printable),
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Self {
        Error::Send(err.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Error::JsValue(Printable::new(err))
    }
}
