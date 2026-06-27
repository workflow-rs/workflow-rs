use crate::runtime::runtime;
use workflow_core::channel::{
    Receiver, RecvError, SendError, Sender as ChannelSender, TryRecvError, TrySendError, unbounded,
};

#[derive(Debug, Clone)]
/// A channel sender that requests a UI repaint after each successful send so
/// that messages produced by async tasks are promptly reflected in the egui UI.
pub struct Sender<T> {
    sender: ChannelSender<T>,
}

impl<T> Sender<T> {
    /// Wraps an underlying channel sender in a repaint-aware sender.
    pub fn new(sender: ChannelSender<T>) -> Self {
        Self { sender }
    }

    /// Sends a message, awaiting capacity if needed, and requests a UI repaint.
    pub async fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.sender.send(msg).await?;
        runtime().request_repaint();
        Ok(())
    }

    /// Sends a message without blocking and requests a UI repaint.
    pub fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
        self.sender.try_send(msg)?;
        runtime().request_repaint();

        Ok(())
    }

    /// Returns the number of receivers currently associated with the channel.
    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Returns the number of senders currently associated with the channel.
    pub fn sender_count(&self) -> usize {
        self.sender.sender_count()
    }
}

#[derive(Debug, Clone)]
/// A bundled sender/receiver pair whose sends request a UI repaint, used for
/// communication between async tasks and the egui event loop.
pub struct Channel<T = ()> {
    /// The repaint-aware sending half of the channel.
    pub sender: Sender<T>,
    /// The receiving half of the channel.
    pub receiver: Receiver<T>,
}

impl<T> Channel<T> {
    /// Creates a new channel with an unbounded message capacity.
    pub fn unbounded() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            sender: Sender::new(sender),
            receiver,
        }
    }

    /// Discards all currently-queued messages from the channel.
    pub fn drain(&self) -> std::result::Result<(), TryRecvError> {
        while !self.receiver.is_empty() {
            self.receiver.try_recv()?;
        }
        Ok(())
    }

    /// Awaits and returns the next message from the channel.
    pub async fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv().await
    }

    /// Receives a message without blocking, failing if the channel is empty.
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }

    /// Sends a message, awaiting capacity if the channel is bounded and full.
    pub async fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.sender.send(msg).await
    }

    /// Sends a message without blocking, failing if the channel is full or closed.
    pub fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
        self.sender.try_send(msg)
    }

    /// Returns the number of messages currently queued in the channel.
    pub fn len(&self) -> usize {
        self.receiver.len()
    }

    /// Returns `true` if there are no messages currently queued.
    pub fn is_empty(&self) -> bool {
        self.receiver.is_empty()
    }

    /// Returns the number of receivers currently associated with the channel.
    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Returns the number of senders currently associated with the channel.
    pub fn sender_count(&self) -> usize {
        self.sender.sender_count()
    }

    /// Returns a non-blocking iterator over the currently-queued messages.
    pub fn iter(&self) -> ChannelIterator<T> {
        ChannelIterator::new(self.receiver.clone())
    }
}

/// Non-blocking iterator that yields currently-queued messages from a channel
/// receiver, stopping once the channel is momentarily empty.
pub struct ChannelIterator<T> {
    receiver: Receiver<T>,
}

impl<T> ChannelIterator<T> {
    /// Creates an iterator that drains messages from the given receiver.
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
