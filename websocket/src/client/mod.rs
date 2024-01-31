//!
//! async WebSocket client functionality (requires a browser (WASM) or tokio (native) executors)
//!

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod wasm;
        use wasm::WebSocketInterface;
    } else {
        mod native;
        use native::WebSocketInterface;
    }
}

pub mod config;
pub mod error;
pub mod message;
pub mod options;
pub mod result;
pub mod websocket;

pub use config::WebSocketConfig;
pub use error::Error;
use futures::Future;
pub use message::*;
pub use options::{ConnectOptions, ConnectStrategy, Options};
pub use result::Result;

use async_trait::async_trait;
use std::pin::Pin;
use std::sync::Arc;
use workflow_core::channel::{oneshot, Channel, Receiver, Sender};
pub type ConnectResult<E> = std::result::Result<Option<Receiver<Result<()>>>, E>;

pub type HandshakeFn = Arc<
    Box<dyn Send + Sync + Fn(&Sender<Message>, &Receiver<Message>) -> HandshakeFnReturn + 'static>,
>;
pub type HandshakeFnReturn = Pin<Box<(dyn Send + Sync + 'static + Future<Output = Result<()>>)>>;

#[async_trait]
pub trait Handshake: Send + Sync + 'static {
    async fn handshake(&self, sender: &Sender<Message>, receiver: &Receiver<Message>)
        -> Result<()>;
}

struct Inner {
    client: Arc<WebSocketInterface>,
    sender_channel: Channel<(Message, Ack)>,
    receiver_channel: Channel<Message>,
}

impl Inner {
    pub fn new(
        client: Arc<WebSocketInterface>,
        sender_channel: Channel<(Message, Ack)>,
        receiver_channel: Channel<Message>,
    ) -> Self {
        Self {
            client,
            sender_channel,
            receiver_channel,
        }
    }
}

/// An async WebSocket implementation capable of operating
/// uniformly under a browser-backed executor in WASM and under
/// native tokio-runtime.
#[derive(Clone)]
pub struct WebSocket {
    inner: Arc<Inner>,
}

impl WebSocket {
    /// Create a new WebSocket instance connecting to the given URL.
    pub fn new(url: &str, options: Options, config: Option<WebSocketConfig>) -> Result<WebSocket> {
        if !url.starts_with("ws://") && !url.starts_with("wss://") {
            return Err(Error::AddressSchema(url.to_string()));
        }

        let receiver_channel = if let Some(cap) = options.receiver_channel_cap {
            Channel::bounded(cap)
        } else {
            Channel::<Message>::unbounded()
        };

        let sender_channel = if let Some(cap) = options.sender_channel_cap {
            Channel::bounded(cap)
        } else {
            Channel::<(Message, Ack)>::unbounded()
        };

        let client = Arc::new(WebSocketInterface::new(
            url,
            sender_channel.clone(),
            receiver_channel.clone(),
            // receiver_tx,
            // sender_tx_rx,
            options,
            config,
        )?);

        let websocket = WebSocket {
            inner: Arc::new(Inner::new(client, sender_channel, receiver_channel)),
        };

        Ok(websocket)
    }

    /// Get current websocket connection URL
    pub fn url(&self) -> String {
        self.inner.client.url()
    }

    /// Changes WebSocket connection URL.
    /// Following this call, you must invoke
    /// `WebSocket::reconnect().await` manually
    pub fn set_url(&self, url: &str) {
        self.inner.client.set_url(url);
    }

    /// Returns the reference to the Sender channel
    pub fn sender_tx(&self) -> &Sender<(Message, Ack)> {
        &self.inner.sender_channel.sender
    }

    /// Returns the reference to the Receiver channel
    pub fn receiver_rx(&self) -> &Receiver<Message> {
        &self.inner.receiver_channel.receiver
    }

    /// Returns true if websocket is connected, false otherwise
    pub fn is_open(&self) -> bool {
        self.inner.client.is_open()
    }

    /// Connects the websocket to the destination URL.
    /// Optionally accepts `block_until_connected` argument
    /// that will block the async execution until the websocket
    /// is connected.
    ///
    /// Once invoked, connection task will run in the background
    /// and will attempt to repeatedly reconnect if the websocket
    /// connection is closed.
    ///
    /// To suspend reconnection, you have to call `disconnect()`
    /// method explicitly.
    ///
    pub async fn connect(&self, options: ConnectOptions) -> ConnectResult<Error> {
        self.inner.client.connect(options).await
    }

    /// Disconnects the websocket from the destination server.
    pub async fn disconnect(&self) -> Result<()> {
        self.inner.client.disconnect().await
    }

    /// Trigger WebSocket to reconnect.  This method
    /// closes the underlying WebSocket connection
    /// causing the WebSocket implementation to
    /// re-initiate connection.
    pub async fn reconnect(&self) -> Result<()> {
        self.inner.client.close().await
    }

    /// Sends a message to the destination server. This function
    /// will queue the message on the relay channel and return
    /// successfully if the message has been queued.
    /// This function enforces async yield in order to prevent
    /// potential blockage of the executor if it is being executed
    /// in tight loops.
    pub async fn post(&self, message: Message) -> Result<&Self> {
        if !self.inner.client.is_open() {
            return Err(Error::NotConnected);
        }

        let result = Ok(self
            .inner
            .sender_channel
            .sender
            .send((message, None))
            .await?);
        workflow_core::task::yield_now().await;
        result.map(|_| self)
    }

    /// Sends a message to the destination server. This function
    /// will block until until the message was relayed to the
    /// underlying websocket implementation.
    pub async fn send(&self, message: Message) -> std::result::Result<&Self, Arc<Error>> {
        if !self.inner.client.is_open() {
            return Err(Arc::new(Error::NotConnected));
        }

        let (ack_sender, ack_receiver) = oneshot();
        self.inner
            .sender_channel
            .send((message, Some(ack_sender)))
            .await
            .map_err(|err| Arc::new(err.into()))?;

        ack_receiver
            .recv()
            .await
            .map_err(|_| Arc::new(Error::DispatchChannelAck))?
            .map(|_| self)
    }

    /// Receives message from the websocket. Blocks until a message is
    /// received from the underlying websocket connection.
    pub async fn recv(&self) -> Result<Message> {
        Ok(self.inner.receiver_channel.receiver.recv().await?)
    }
}
