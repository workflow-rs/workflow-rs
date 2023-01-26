//! [`async_std::channel`] re-exports and shims
pub use async_std::channel::*;
use thiserror::Error;

#[derive(Error)]
pub enum ChannelError<T> {
    #[error(transparent)]
    SendError(#[from] SendError<T>),
    #[error(transparent)]
    RecvError(#[from] RecvError),
}

/// Creates a oneshot channel (bounded channel with a limit of 1 message)
pub fn oneshot<T>() -> (Sender<T>, Receiver<T>) {
    bounded(1)
}

#[derive(Debug, Clone)]
pub struct DuplexChannel<T = (), R = ()> {
    pub request: Channel<T>,
    pub response: Channel<R>,
}

impl<T, R> DuplexChannel<T, R> {
    pub fn unbounded() -> Self {
        Self {
            request: Channel::unbounded(),
            response: Channel::unbounded(),
        }
    }

    pub fn oneshot() -> Self {
        Self {
            request: Channel::oneshot(),
            response: Channel::oneshot(),
        }
    }

    pub async fn signal(&self, msg: T) -> std::result::Result<R, ChannelError<T>> {
        self.request.sender.send(msg).await?;
        self.response
            .receiver
            .recv()
            .await
            .map_err(|err| err.into())
    }
}

/// [`Channel`] struct that combines [`async_std::channel::Sender`] and
/// [`async_std::channel::Receiver`] into a single struct with `sender`
/// and `receiver` members.
#[derive(Debug, Clone)]
pub struct Channel<T = ()> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn unbounded() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }

    pub fn bounded(cap: usize) -> Self {
        let (sender, receiver) = bounded(cap);
        Self { sender, receiver }
    }

    pub fn oneshot() -> Self {
        let (sender, receiver) = bounded(1);
        Self { sender, receiver }
    }

    pub fn drain(&self) -> std::result::Result<(), TryRecvError> {
        while !self.receiver.is_empty() {
            self.receiver.try_recv()?;
        }
        Ok(())
    }

    pub async fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv().await
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }

    pub async fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.sender.send(msg).await
    }

    pub fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
        self.sender.try_send(msg)
    }

    pub fn len(&self) -> usize {
        self.receiver.len()
    }

    pub fn is_empty(&self) -> bool {
        self.receiver.is_empty()
    }

    pub fn iter(&self) -> ChannelIterator<T> {
        ChannelIterator::new(self.receiver.clone())
    }
}

pub struct ChannelIterator<T> {
    receiver: Receiver<T>,
}

impl<T> ChannelIterator<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        ChannelIterator { receiver }
    }
}

impl<T> Iterator for ChannelIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.receiver.is_empty() {
            None
        } else {
            self.receiver.try_recv().ok()
        }
    }
}
