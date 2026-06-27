//! [`async-channel`](async_channel) re-exports and shims
use crate::id::Id;
pub use async_channel::{
    Receiver, RecvError, SendError, Sender, TryRecvError, TrySendError, bounded, unbounded,
};
use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};
use thiserror::Error;

/// Errors produced by channel operations in this module.
#[derive(Error, Debug)]
pub enum ChannelError<T> {
    /// The underlying channel send operation failed (the channel is closed).
    #[error(transparent)]
    SendError(#[from] SendError<T>),
    /// The underlying channel receive operation failed (the channel is closed and empty).
    #[error(transparent)]
    RecvError(#[from] RecvError),
    /// Serialization or deserialization to/from a JavaScript value failed.
    #[error(transparent)]
    SerdeWasmBindgen(#[from] serde_wasm_bindgen::Error),
    /// A non-blocking `try_send()` failed while broadcasting through a [`Multiplexer`].
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
    /// Channel carrying request messages of type `T`.
    pub request: Channel<T>,
    /// Channel carrying response messages of type `R`.
    pub response: Channel<R>,
}

impl<T, R> DuplexChannel<T, R> {
    /// Creates a duplex channel whose `request` and `response` channels are both unbounded.
    pub fn unbounded() -> Self {
        Self {
            request: Channel::unbounded(),
            response: Channel::unbounded(),
        }
    }

    /// Creates a duplex channel whose `request` and `response` channels are both oneshot.
    pub fn oneshot() -> Self {
        Self {
            request: Channel::oneshot(),
            response: Channel::oneshot(),
        }
    }

    /// Sends `msg` on the request channel and waits for the matching response,
    /// returning the received response value.
    pub async fn signal(&self, msg: T) -> std::result::Result<R, ChannelError<T>> {
        self.request.sender.send(msg).await?;
        self.response
            .receiver
            .recv()
            .await
            .map_err(|err| err.into())
    }
}

/// [`Channel`] struct that combines [[`async_channel::Sender`]] and
/// [[`async_channel::Receiver`]] into a single struct with `sender`
/// and `receiver` members representing a single channel.
#[derive(Debug, Clone)]
pub struct Channel<T = ()> {
    /// Sender endpoint of the channel.
    pub sender: Sender<T>,
    /// Receiver endpoint of the channel.
    pub receiver: Receiver<T>,
}

impl<T> Channel<T> {
    /// Creates a channel with an unbounded message buffer.
    pub fn unbounded() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }

    /// Creates a channel with a buffer bounded to `cap` messages.
    pub fn bounded(cap: usize) -> Self {
        let (sender, receiver) = bounded(cap);
        Self { sender, receiver }
    }

    /// Creates a oneshot channel (a bounded channel with a capacity of one message).
    pub fn oneshot() -> Self {
        let (sender, receiver) = bounded(1);
        Self { sender, receiver }
    }

    /// Discards all currently-buffered messages from the channel.
    pub fn drain(&self) -> std::result::Result<(), TryRecvError> {
        while !self.receiver.is_empty() {
            self.receiver.try_recv()?;
        }
        Ok(())
    }

    /// Receives a message, waiting asynchronously until one is available.
    pub async fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv().await
    }

    /// Attempts to receive a message without blocking, returning an error if the channel is empty.
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }

    /// Sends a message, waiting asynchronously if the channel is bounded and full.
    pub async fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.sender.send(msg).await
    }

    /// Attempts to send a message without blocking, returning an error if the channel is full or closed.
    pub fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
        self.sender.try_send(msg)
    }

    /// Returns the number of messages currently buffered in the channel.
    pub fn len(&self) -> usize {
        self.receiver.len()
    }

    /// Returns `true` if there are no messages currently buffered in the channel.
    pub fn is_empty(&self) -> bool {
        self.receiver.is_empty()
    }

    /// Returns the number of [`Receiver`] endpoints currently connected to the channel.
    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Returns the number of [`Sender`] endpoints currently connected to the channel.
    pub fn sender_count(&self) -> usize {
        self.sender.sender_count()
    }

    /// Returns an iterator that drains all currently-buffered messages from the channel.
    pub fn iter(&self) -> ChannelIterator<T> {
        ChannelIterator::new(self.receiver.clone())
    }
}

/// Iterator that drains all currently-buffered messages from a [`Channel`]'s
/// receiver, yielding `None` once the channel is momentarily empty.
pub struct ChannelIterator<T> {
    receiver: Receiver<T>,
}

impl<T> ChannelIterator<T> {
    /// Create a new iterator that drains messages from the given receiver.
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
    /// Map of currently registered receiver channels keyed by their unique id.
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
    /// Unique id identifying this channel within the parent [`Multiplexer`].
    pub id: Id,
    /// Sender endpoint, provided for convenient internal relay within this channel.
    pub sender: Sender<T>,
    /// Receiver endpoint from which broadcast events are consumed.
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
