use super::{
    error::Error, message::Message, result::Result, Ack, ConnectOptions, ConnectResult,
    ConnectStrategy, Handshake, Options, WebSocketConfig,
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
use std::time::Duration;
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
            max_send_queue: config.max_send_queue,
            max_message_size: config.max_message_size,
            max_frame_size: config.max_frame_size,
            accept_unmasked_frames: config.accept_unmasked_frames,
        }
    }
}

struct Settings {
    url: String,
}

// #[allow(dead_code)]
// struct Inner {
//     ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
// }

pub struct WebSocketInterface {
    // inner: Arc<Mutex<Option<Inner>>>,
    settings: Arc<Mutex<Settings>>,
    config: Option<WebSocketConfig>,
    // reconnect : Arc<Mutex<bool>>,
    reconnect: AtomicBool,
    is_open: AtomicBool,
    receiver_channel: Channel<Message>,
    sender_channel: Channel<(Message, Ack)>,
    shutdown: DuplexChannel<()>,
    handshake: Option<Arc<dyn Handshake>>,
}

impl WebSocketInterface {
    pub fn new(
        url: &str,
        sender_channel: Channel<(Message, Ack)>,
        receiver_channel: Channel<Message>,
        options: Options,
        config: Option<WebSocketConfig>,
    ) -> Result<WebSocketInterface> {
        let settings = Settings {
            url: url.to_string(),
        };

        let iface = WebSocketInterface {
            settings: Arc::new(Mutex::new(settings)),
            config,
            receiver_channel,
            sender_channel,
            reconnect: AtomicBool::new(true),
            is_open: AtomicBool::new(false),
            shutdown: DuplexChannel::unbounded(),
            handshake: options.handshake,
        };

        Ok(iface)
    }

    pub fn url(self: &Arc<Self>) -> String {
        self.settings.lock().unwrap().url.clone()
    }

    pub fn set_url(self: &Arc<Self>, url: &str) {
        self.settings.lock().unwrap().url = url.into();
    }

    pub fn is_open(self: &Arc<Self>) -> bool {
        self.is_open.load(Ordering::SeqCst)
    }

    pub async fn connect(self: &Arc<Self>, options: ConnectOptions) -> ConnectResult<Error> {
        let this = self.clone();

        if self.is_open.load(Ordering::SeqCst) {
            return Err(Error::AlreadyConnected);
        }

        let (connect_trigger, connect_listener) = oneshot::<Result<()>>();
        let mut connect_trigger = Some(connect_trigger);

        this.reconnect.store(true, Ordering::SeqCst);

        let options_ = options.clone();
        if let Some(url) = options.url.as_ref() {
            self.set_url(url);
        }

        let ts_websocket_config = self.config.clone().map(|config| config.into());

        core::task::spawn(async move {
            loop {
                let url = this.url().clone();
                let connect_future = connect_async_with_config(&url, ts_websocket_config, false);
                let timeout_future = timeout(options_.timeout(), connect_future);

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
                        log_trace!("WebSocket failed to connect to {}: {}", this.url(), e);
                        if matches!(options_.strategy, ConnectStrategy::Fallback) {
                            if options.block_async_connect && connect_trigger.is_some() {
                                connect_trigger.take().unwrap().try_send(Err(e.into())).ok();
                            }
                            break;
                        }
                        workflow_core::task::sleep(Duration::from_millis(1000)).await;
                    }
                    // timeout error
                    Err(_) => {
                        log_trace!(
                            "WebSocket connection timeout while connecting to {}",
                            this.url()
                        );
                        if matches!(options_.strategy, ConnectStrategy::Fallback) {
                            if options.block_async_connect && connect_trigger.is_some() {
                                connect_trigger
                                    .take()
                                    .unwrap()
                                    .try_send(Err(Error::ConnectionTimeout))
                                    .ok();
                            }
                            break;
                        }
                        workflow_core::task::sleep(Duration::from_millis(1000)).await;
                    }
                };

                if !this.reconnect.load(Ordering::SeqCst) {
                    break;
                };
            }
        });

        match options.block_async_connect {
            true => match connect_listener.recv().await? {
                Ok(_) => Ok(None),
                Err(e) => Err(e),
            },
            false => Ok(Some(connect_listener)),
        }
    }

    async fn handshake(
        self: &Arc<Self>,
        ws_sender: &mut SplitSink<&mut WebSocketStream<MaybeTlsStream<TcpStream>>, TsMessage>,
        ws_receiver: &mut SplitStream<&mut WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) -> Result<()> {
        if let Some(handshake) = self.handshake.as_ref().cloned() {
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

        self.handshake(&mut ws_sender, &mut ws_receiver).await?;

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
                                TsMessage::Ping(_) => { },
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

        // *self.inner.lock().unwrap() = None;

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
