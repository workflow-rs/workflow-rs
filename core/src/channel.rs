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

/// [`DuplexChannel`] contains 2 channels `request` and `response`
/// meant to provide for a request/response pattern. This is useful
/// for any type of signaling, but especially during task termination,
/// where you can request a task to terminate and wait for a response
/// confirming its termination.
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
/// and `receiver` members representing a single channel.
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

/// A simple MPMC (one to many) channel Multiplexer that broadcasts to
/// multiple registered receivers.  [`Multiplexer<T>`] itself can be
/// cloned and used to broadcast using [`Multiplexer::broadcast()`]
/// or [`Multiplexer::try_broadcast()`].  To create a receiving channel,
/// you can call [`MultiplexerChannel<T>::from()`] and supply the
/// desired Multiplexer instance, or  simply call [`Multiplexer::channel()`]
/// to create a new [`MultiplexerChannel`] instance.  The receiving channel
/// gets unregistered when [`MultiplexerChannel`] is dropped or the
/// underlying [`Receiver`] is closed.
#[derive(Clone)]
pub struct Multiplexer<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub channels: Arc<Mutex<HashMap<Id, Arc<Sender<T>>>>>,
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
    /// Create a new Multiplexer instance
    pub fn new() -> Multiplexer<T> {
        Multiplexer {
            channels: Arc::new(Mutex::new(HashMap::default())),
            t: PhantomData,
        }
    }

    /// Create a new multiplexer receiving channel
    pub fn channel(&self) -> MultiplexerChannel<T> {
        MultiplexerChannel::from(self)
    }

    fn register_event_channel(&self) -> (Id, Sender<T>, Receiver<T>) {
        let (sender, receiver) = unbounded();
        let id = Id::new();
        self.channels
            .lock()
            .unwrap()
            .insert(id, Arc::new(sender.clone()));
        (id, sender, receiver)
    }

    fn unregister_event_channel(&self, id: Id) {
        self.channels.lock().unwrap().remove(&id);
    }

    /// Async [`Multiplexer::broadcast`] function that calls [`Sender::send()`] on all registered [`MultiplexerChannel`] instances.
    pub async fn broadcast(&self, event: T) -> Result<(), ChannelError<T>> {
        let mut removed = vec![];
        let channels = self
            .channels
            .lock()
            .unwrap()
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect::<Vec<_>>();
        for (id, sender) in channels.iter() {
            match sender.send(event.clone()).await {
                Ok(_) => {}
                Err(_err) => {
                    removed.push(*id);
                }
            }
        }
        if !removed.is_empty() {
            let mut channels = self.channels.lock().unwrap();
            for id in removed.iter() {
                channels.remove(id);
            }
        }

        Ok(())
    }

    /// A synchronous [`Multiplexer::try_broadcast`] function that calls [`Sender::try_send()`] on all registered [`MultiplexerChannel`] instances.
    /// This function holds a mutex for the duration of the broadcast.
    pub fn try_broadcast(&self, event: T) -> Result<(), ChannelError<T>> {
        let mut removed = vec![];
        let mut channels = self.channels.lock().unwrap();
        for (id, sender) in channels.iter() {
            match sender.try_send(event.clone()) {
                Ok(_) => {}
                Err(_err) => {
                    removed.push(*id);
                }
            }
        }
        if !removed.is_empty() {
            for id in removed.iter() {
                channels.remove(id);
            }
        }

        Ok(())
    }
}

/// Receiving channel endpoint for the [`Multiplexer`].  [`MultiplexerChannel<T>`] holds a [`Sender`] and the [`Receiver`] channel endpoints.
/// The [`Sender`] is provided for convenience, allowing internal relay within this channel instance.
/// To process events, simply iterate over [`MultiplexerChannel::recv()`] by calling `channel.recv().await`.
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
    /// Close the receiving channel.  This will unregister the channel from the [`Multiplexer`].
    pub fn close(&self) {
        self.multiplexer.unregister_event_channel(self.id);
    }

    /// Receive an event from the channel.  This is a blocking async call.
    pub async fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv().await
    }

    /// Receive an event from the channel.  This is a non-blocking sync call that
    /// follows [`Receiver::try_recv`] semantics.
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }
}

/// Create a [`MultiplexerChannel`] from [`Multiplexer`] by reference.
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
