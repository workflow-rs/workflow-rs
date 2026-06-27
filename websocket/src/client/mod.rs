//!
//! async WebSocket client functionality (requires a browser (WASM) or tokio (native) executors)
//!
//! # TLS crypto provider (native)
//!
//! Native secure (`wss://`) connections use `tungstenite` with
//! [`rustls`](https://docs.rs/rustls). Since rustls 0.23, a process-level crypto
//! provider must be installed before the first secure connection — typically by
//! the application or a higher-level SDK at startup:
//!
//! ```ignore
//! rustls::crypto::ring::default_provider().install_default().unwrap();
//! ```
//!
//! Without it, opening a `wss://` connection fails with *"Could not automatically
//! determine the process-level CryptoProvider"*. In the browser/WASM environment
//! TLS is handled by the host `WebSocket`, so no provider is required.
//!

use cfg_if::cfg_if;

mod wasm;
pub use wasm::WebSocketInterface as _WSI;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm::WebSocketInterface;
    } else {
        mod native;
        use native::WebSocketInterface;
    }
}

/// Low-level bindings to the underlying browser/Node.js WebSocket interface.
pub mod bindings;
pub mod config;
/// WebSocket client error type and its conversions.
pub mod error;
/// WebSocket client message types exchanged with the server.
pub mod message;
/// Connection options controlling connect and reconnect behavior.
pub mod options;
/// Result type alias used throughout the WebSocket client.
pub mod result;

pub use config::WebSocketConfig;
pub use error::Error;
use futures::Future;
pub use message::*;
pub use options::{ConnectOptions, ConnectStrategy};
pub use result::Result;

use async_trait::async_trait;
use std::pin::Pin;
use std::sync::Arc;
use workflow_core::channel::{Channel, Receiver, Sender, oneshot};
/// Result of a connect attempt. On success yields `Some(receiver)` when the
/// caller did not block on the connection (the receiver fires once connected),
/// or `None` when the connect call blocked until the connection was established.
pub type ConnectResult<E> = std::result::Result<Option<Receiver<Result<()>>>, E>;

/// Shared closure invoked to perform a custom handshake negotiation using the
/// supplied send/receive channels before the connection is considered ready.
pub type HandshakeFn = Arc<
    Box<dyn Send + Sync + Fn(&Sender<Message>, &Receiver<Message>) -> HandshakeFnReturn + 'static>,
>;
/// The boxed future returned by a [`HandshakeFn`], resolving once the
/// handshake completes (or fails).
pub type HandshakeFnReturn = Pin<Box<dyn Send + Sync + 'static + Future<Output = Result<()>>>>;

/// Trait implemented by custom handshake handlers that negotiate with the
/// server immediately after the socket opens and before it is marked connected.
#[async_trait]
pub trait Handshake: Send + Sync + 'static {
    /// Perform the handshake using the given send and receive channels,
    /// returning once negotiation has succeeded or failed.
    async fn handshake(&self, sender: &Sender<Message>, receiver: &Receiver<Message>)
    -> Result<()>;
}

/// Trait implemented by URL resolvers that supply the destination URL
/// dynamically when no explicit URL has been configured.
#[async_trait]
pub trait Resolver: Send + Sync + 'static {
    /// Resolve and return the WebSocket URL to connect to.
    async fn resolve_url(&self) -> ResolverResult;
}
/// Result of a [`Resolver::resolve_url`] call, yielding the destination URL.
pub type ResolverResult = Result<String>;
/// Alias for the WebSocket client [`Error`] type.
pub type WebSocketError = Error;

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
    pub fn new(url: Option<&str>, config: Option<WebSocketConfig>) -> Result<WebSocket> {
        if let Some(url) = url
            && !url.starts_with("ws://")
            && !url.starts_with("wss://")
        {
            return Err(Error::AddressSchema(url.to_string()));
        }

        let config = config.unwrap_or_default();

        let receiver_channel = if let Some(cap) = config.receiver_channel_cap {
            Channel::bounded(cap)
        } else {
            Channel::<Message>::unbounded()
        };

        let sender_channel = if let Some(cap) = config.sender_channel_cap {
            Channel::bounded(cap)
        } else {
            Channel::<(Message, Ack)>::unbounded()
        };

        let client = Arc::new(WebSocketInterface::new(
            url,
            Some(config),
            sender_channel.clone(),
            receiver_channel.clone(),
        )?);

        let websocket = WebSocket {
            inner: Arc::new(Inner::new(client, sender_channel, receiver_channel)),
        };

        Ok(websocket)
    }

    /// Get current websocket connection URL
    pub fn url(&self) -> Option<String> {
        self.inner.client.current_url()
    }

    /// Changes WebSocket connection URL.
    /// Following this call, you must invoke
    /// `WebSocket::reconnect().await` manually
    pub fn set_url(&self, url: &str) {
        self.inner.client.set_default_url(url);
    }

    /// Configure WebSocket connection settings
    /// Can be supplied after the WebSocket has been
    /// has been created to alter the configuration
    /// for the next connection.
    pub fn configure(&self, config: WebSocketConfig) {
        self.inner.client.configure(config);
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
    pub fn is_connected(&self) -> bool {
        self.inner.client.is_connected()
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
        if !self.inner.client.is_connected() {
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
        if !self.inner.client.is_connected() {
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

    /// Triggers a disconnection on the underlying WebSocket.
    /// This is intended for debug purposes only.
    /// Can be used to test application reconnection logic.
    pub fn trigger_abort(&self) -> Result<()> {
        self.inner.client.trigger_abort()
    }
}
