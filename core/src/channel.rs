//! [`async_std::channel`] re-exports and shims
use crate::id::Id;
pub use async_channel::{
    bounded, unbounded, Receiver, RecvError, SendError, Sender, TryRecvError, TrySendError,
};
use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChannelError<T> {
    #[error(transparent)]
    SendError(#[from] SendError<T>),
    #[error(transparent)]
    RecvError(#[from] RecvError),
    #[error(transparent)]
    SerdeWasmBindgen(#[from] serde_wasm_bindgen::Error),
    #[error("try_send() error during multiplexer broadcast")]
    BroadcastTrySendError,
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

    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }

    pub fn sender_count(&self) -> usize {
        self.sender.sender_count()
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

/// A simple channel Multiplexer that broadcasts to multiple registered receivers.
#[derive(Clone)]
pub struct Multiplexer<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub channels: Arc<Mutex<HashMap<Id, Sender<T>>>>,
    t: PhantomData<T>,
}

impl<T> Default for Multiplexer<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Multiplexer<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Multiplexer<T> {
        Multiplexer {
            channels: Arc::new(Mutex::new(HashMap::default())),
            t: PhantomData,
        }
    }

    pub fn create_channel(&self) -> MultiplexerChannel<T> {
        MultiplexerChannel::from(self)
    }

    fn register_event_channel(&self) -> (Id, Sender<T>, Receiver<T>) {
        let (sender, receiver) = unbounded();
        let id = Id::new();
        self.channels.lock().unwrap().insert(id, sender.clone());
        (id, sender, receiver)
    }

    fn unregister_event_channel(&self, id: Id) {
        self.channels.lock().unwrap().remove(&id);
    }

    pub async fn broadcast(&self, event: T) -> Result<(), ChannelError<T>> {
        let channels = self.channels.lock().unwrap();
        for (_, sender) in channels.iter() {
            match sender.try_send(event.clone()) {
                Ok(_) => {}
                Err(_err) => {
                    // log_error!(
                    //     "Multiplexer: error multiplexing the event {:?}: {:?}",
                    //     event.clone(),
                    //     err,
                    // );
                    return Err(ChannelError::BroadcastTrySendError);
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct MultiplexerChannel<T>
where
    T: Clone + Send + Sync + 'static,
{
    multiplexer: Multiplexer<T>,
    pub id: Id,
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> MultiplexerChannel<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn close(&self) {
        self.multiplexer.unregister_event_channel(self.id);
    }
}

impl<T> From<&Multiplexer<T>> for MultiplexerChannel<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn from(multiplexer: &Multiplexer<T>) -> Self {
        let (id, sender, receiver) = multiplexer.register_event_channel();
        MultiplexerChannel {
            multiplexer: multiplexer.clone(),
            id,
            sender,
            receiver,
        }
    }
}

impl<T> Drop for MultiplexerChannel<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn drop(&mut self) {
        self.multiplexer.unregister_event_channel(self.id);
    }
}
