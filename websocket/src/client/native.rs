use super::{
    error::Error, message::Message, result::Result, Ack, ConnectOptions, ConnectResult,
    ConnectStrategy, Handshake, Resolver, WebSocketConfig,
};
use futures::{
    select_biased,
    stream::{SplitSink, SplitStream},
    FutureExt,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_tungstenite::{
    connect_async_with_config, tungstenite::protocol::Message as TsMessage, MaybeTlsStream,
    WebSocketStream,
};
use tungstenite::protocol::WebSocketConfig as TsWebSocketConfig;
pub use workflow_core as core;
use workflow_core::channel::*;
pub use workflow_log::*;

impl From<Message> for tungstenite::Message {
    fn from(message: Message) -> Self {
        match message {
            Message::Text(text) => text.into(),
            Message::Binary(data) => data.into(),
            _ => {
                panic!("From<Message> for tungstenite::Message - invalid message type: {message:?}",)
            }
        }
    }
}

impl From<tungstenite::Message> for Message {
    fn from(message: tungstenite::Message) -> Self {
        match message {
            TsMessage::Text(text) => Message::Text(text),
            TsMessage::Binary(data) => Message::Binary(data),
            TsMessage::Close(_) => Message::Close,
            _ => panic!(
                "TryFrom<tungstenite::Message> for Message - invalid message type: {message:?}",
            ),
        }
    }
}

impl From<WebSocketConfig> for TsWebSocketConfig {
    fn from(config: WebSocketConfig) -> Self {
        TsWebSocketConfig {
            write_buffer_size: config.write_buffer_size,
            max_write_buffer_size: config.max_write_buffer_size,
            max_message_size: config.max_message_size,
            max_frame_size: config.max_frame_size,
            accept_unmasked_frames: config.accept_unmasked_frames,
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct Settings {
    default_url: Option<String>,
    current_url: Option<String>,
}

pub struct WebSocketInterface {
    settings: Mutex<Settings>,
    config: Mutex<WebSocketConfig>,
    reconnect: AtomicBool,
    is_open: AtomicBool,
    receiver_channel: Channel<Message>,
    sender_channel: Channel<(Message, Ack)>,
    shutdown: DuplexChannel<()>,
}

impl WebSocketInterface {
    pub fn new(
        url: Option<&str>,
        config: Option<WebSocketConfig>,
        sender_channel: Channel<(Message, Ack)>,
        receiver_channel: Channel<Message>,
    ) -> Result<WebSocketInterface> {
        let settings = Settings {
            default_url: url.map(String::from),
            ..Default::default()
        };

        let iface = WebSocketInterface {
            settings: Mutex::new(settings),
            config: Mutex::new(config.unwrap_or_default()),
            receiver_channel,
            sender_channel,
            reconnect: AtomicBool::new(true),
            is_open: AtomicBool::new(false),
            shutdown: DuplexChannel::unbounded(),
        };

        Ok(iface)
    }

    pub fn default_url(self: &Arc<Self>) -> Option<String> {
        self.settings.lock().unwrap().default_url.clone()
    }

    pub fn current_url(self: &Arc<Self>) -> Option<String> {
        self.settings.lock().unwrap().current_url.clone()
    }

    pub fn set_default_url(self: &Arc<Self>, url: &str) {
        self.settings
            .lock()
            .unwrap()
            .default_url
            .replace(url.to_string());
    }

    pub fn set_current_url(self: &Arc<Self>, url: &str) {
        self.settings
            .lock()
            .unwrap()
            .current_url
            .replace(url.to_string());
    }

    pub fn is_open(self: &Arc<Self>) -> bool {
        self.is_open.load(Ordering::SeqCst)
    }

    fn resolver(&self) -> Option<Arc<dyn Resolver>> {
        self.config.lock().unwrap().resolver.clone()
    }

    fn handshake(&self) -> Option<Arc<dyn Handshake>> {
        self.config.lock().unwrap().handshake.clone()
    }

    pub fn configure(&self, config: WebSocketConfig) {
        *self.config.lock().unwrap() = config;
    }

    fn config(&self) -> WebSocketConfig {
        self.config.lock().unwrap().clone()
    }

    async fn resolve_url(self: &Arc<Self>, options: &ConnectOptions) -> Result<String> {
        let url = if let Some(url) = options.url.as_ref().or(self.default_url().as_ref()) {
            url.clone()
        } else if let Some(resolver) = self.resolver() {
            resolver.resolve_url().await?
        } else {
            return Err(Error::MissingUrl);
        };
        self.set_current_url(&url);
        Ok(url)
    }

    pub async fn connect(self: &Arc<Self>, options: ConnectOptions) -> ConnectResult<Error> {
        let this = self.clone();

        if self.is_open.load(Ordering::SeqCst) {
            return Err(Error::AlreadyConnected);
        }

        let (connect_trigger, connect_listener) = oneshot::<Result<()>>();
        let mut connect_trigger = Some(connect_trigger);

        this.reconnect.store(true, Ordering::SeqCst);

        let block_async_connect = options.block_async_connect;
        let ts_websocket_config = Some(self.config().into());

        core::task::spawn(async move {
            'outer: loop {
                match this.resolve_url(&options).await {
                    Ok(url) => {
                        let connect_future =
                            connect_async_with_config(&url, ts_websocket_config, false);
                        let timeout_future = timeout(options.connect_timeout(), connect_future);

                        match timeout_future.await {
                            // connect success
                            Ok(Ok(stream)) => {
                                // log_trace!("connected...");

                                this.is_open.store(true, Ordering::SeqCst);
                                let (mut ws_stream, _) = stream;

                                if connect_trigger.is_some() {
                                    connect_trigger.take().unwrap().try_send(Ok(())).ok();
                                }

                                if let Err(err) = this.dispatcher(&mut ws_stream).await {
                                    log_trace!("WebSocket dispatcher error: {}", err);
                                }

                                this.is_open.store(false, Ordering::SeqCst);
                            }
                            // connect error
                            Ok(Err(e)) => {
                                log_trace!("WebSocket failed to connect to {}: {}", url, e);
                                if matches!(options.strategy, ConnectStrategy::Fallback) {
                                    if options.block_async_connect && connect_trigger.is_some() {
                                        connect_trigger
                                            .take()
                                            .unwrap()
                                            .try_send(Err(e.into()))
                                            .ok();
                                    }
                                    break;
                                }
                                workflow_core::task::sleep(options.retry_interval()).await;
                            }
                            // timeout error
                            Err(_) => {
                                log_trace!(
                                    "WebSocket connection timeout while connecting to {}",
                                    url
                                );
                                if matches!(options.strategy, ConnectStrategy::Fallback) {
                                    if options.block_async_connect && connect_trigger.is_some() {
                                        connect_trigger
                                            .take()
                                            .unwrap()
                                            .try_send(Err(Error::ConnectionTimeout))
                                            .ok();
                                    }
                                    break;
                                }
                                workflow_core::task::sleep(options.retry_interval()).await;
                            }
                        };

                        if !this.reconnect.load(Ordering::SeqCst) {
                            break 'outer;
                        };
                    }
                    Err(err) => {
                        log_trace!("WebSocket failed to get session URL: {}", err);
                        if !this.reconnect.load(Ordering::SeqCst) {
                            break 'outer;
                        } else {
                            workflow_core::task::sleep(options.retry_interval()).await;
                        }
                    }
                }
            }
        });

        match block_async_connect {
            true => match connect_listener.recv().await? {
                Ok(_) => Ok(None),
                Err(e) => Err(e),
            },
            false => Ok(Some(connect_listener)),
        }
    }

    async fn handshake_impl(
        self: &Arc<Self>,
        ws_sender: &mut SplitSink<&mut WebSocketStream<MaybeTlsStream<TcpStream>>, TsMessage>,
        ws_receiver: &mut SplitStream<&mut WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) -> Result<()> {
        if let Some(handshake) = self.handshake() {
            let (sender_tx, sender_rx) = unbounded();
            let (receiver_tx, receiver_rx) = unbounded();
            let (accept_tx, accept_rx) = oneshot();

            core::task::spawn(async move {
                accept_tx
                    .send(handshake.handshake(&sender_tx, &receiver_rx).await)
                    .await
                    .unwrap_or_else(|err| {
                        log_trace!("WebSocket handshake unable to send completion: `{}`", err)
                    });
            });

            loop {
                select_biased! {
                    result = accept_rx.recv().fuse() => {
                        return result?;
                    },
                    msg = sender_rx.recv().fuse() => {
                        if let Ok(msg) = msg {
                            ws_sender.send(msg.into()).await?;
                        }
                    },
                    msg = ws_receiver.next().fuse() => {
                        if let Some(Ok(msg)) = msg {
                            receiver_tx.send(msg.into()).await?;
                        } else {
                            return Err(Error::NegotiationFailure);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn dispatcher(
        self: &Arc<Self>,
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> Result<()> {
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        self.handshake_impl(&mut ws_sender, &mut ws_receiver)
            .await?;

        self.receiver_channel.send(Message::Open).await?;

        loop {
            select_biased! {
                dispatch = self.sender_channel.recv().fuse() => {
                    if let Ok((msg,ack)) = dispatch {
                        if let Some(ack_sender) = ack {
                            let result = ws_sender.send(msg.into()).await
                                .map(Arc::new)
                                .map_err(|err|Arc::new(err.into()));
                            ack_sender.send(result).await?;
                        } else {
                            ws_sender.send(msg.into()).await?;
                        }
                    }
                }
                msg = ws_receiver.next().fuse() => {
                    match msg {
                        Some(Ok(msg)) => {
                            match msg {
                                TsMessage::Binary(_) | TsMessage::Text(_) | TsMessage::Close(_) => {
                                    self
                                        .receiver_channel
                                        .send(msg.into())
                                        .await?;
                                }
                                TsMessage::Ping(data) => {
                                    ws_sender.send(TsMessage::Pong(data)).await?;
                                },
                                TsMessage::Pong(_) => { },
                                TsMessage::Frame(_frame) => { },
                            }
                        }
                        Some(Err(e)) => {
                            self.receiver_channel.send(Message::Close).await?;
                            log_trace!("WebSocket error: {}", e);
                            break;
                        }
                        None => {
                            self.receiver_channel.send(Message::Close).await?;
                            log_trace!("WebSocket connection closed");
                            break;
                        }
                    }
                }
                _ = self.shutdown.request.receiver.recv().fuse() => {
                    self.receiver_channel.send(Message::Close).await?;
                    self.shutdown.response.sender.send(()).await?;
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn close(self: &Arc<Self>) -> Result<()> {
        // if self.inner.lock().unwrap().is_some() {
        if self.is_open.load(Ordering::SeqCst) {
            // } self.inner.lock().unwrap().is_some() {
            self.shutdown
                .request
                .sender
                .send(())
                .await
                .unwrap_or_else(|err| {
                    log_error!("Unable to signal WebSocket dispatcher shutdown: {}", err)
                });
            self.shutdown
                .response
                .receiver
                .recv()
                .await
                .unwrap_or_else(|err| {
                    log_error!("Unable to receive WebSocket dispatcher shutdown: {}", err)
                });
        }

        Ok(())
    }

    pub async fn disconnect(self: &Arc<Self>) -> Result<()> {
        self.reconnect.store(false, Ordering::SeqCst);
        self.close().await?;
        Ok(())
    }
}
