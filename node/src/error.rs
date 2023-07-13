//! Errors produced by the the [`node`](super) crate
use thiserror::Error;
use wasm_bindgen::prelude::*;
use workflow_core::channel::{RecvError, SendError, TryRecvError};
use workflow_wasm::printable::Printable;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Already running")]
    AlreadyRunning,
    #[error("The task is not running")]
    NotRunning,
    #[error("Child process reference is absent")]
    ProcIsAbsent,
    #[error("{0:?}")]
    Send(String),
    #[error("{0:?}")]
    Recv(#[from] RecvError),
    #[error("{0:?}")]
    TryRecv(#[from] TryRecvError),
    #[error(transparent)]
    Task(#[from] workflow_task::TaskError),
    #[error(transparent)]
    Callback(#[from] workflow_wasm::callback::CallbackError),
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
