use super::{
    bindings::WebSocket as W3CWebSocket,
    error::Error,
    message::{Ack, Message},
    result::Result,
    ConnectOptions, ConnectResult, Handshake, Options, WebSocketConfig,
};
use futures::{select, select_biased, FutureExt};
use js_sys::{ArrayBuffer, Uint8Array};
use std::ops::Deref;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use wasm_bindgen::JsCast;
use web_sys::{
    CloseEvent as WsCloseEvent, ErrorEvent as WsErrorEvent, MessageEvent as WsMessageEvent,
};
use workflow_core::runtime::*;
use workflow_core::{
    channel::{oneshot, unbounded, Channel, DuplexChannel, Sender},
    task::spawn,
};
use workflow_log::*;
use workflow_wasm::callback::*;
// use workflow_wasm::options::OptionsTrait;

impl TryFrom<WsMessageEvent> for Message {
    type Error = Error;

    fn try_from(event: WsMessageEvent) -> std::result::Result<Self, Self::Error> {
        match event.data() {
            data if data.is_instance_of::<ArrayBuffer>() => {
                let buffer = Uint8Array::new(data.unchecked_ref());
                Ok(Message::Binary(buffer.to_vec()))
            }
            data if data.is_string() => match data.as_string() {
                Some(text) => Ok(Message::Text(text)),
                None => Err(Error::DataEncoding),
            },
            _ => Err(Error::DataType),
        }
    }
}

#[derive(Clone)]
pub struct WebSocket(W3CWebSocket);
unsafe impl Send for WebSocket {}
unsafe impl Sync for WebSocket {}
impl Deref for WebSocket {
    type Target = W3CWebSocket;
    fn deref(&self) -> &W3CWebSocket {
        &self.0
    }
}

impl WebSocket {
    #[allow(dead_code)]
    const CONNECTING: u16 = W3CWebSocket::CONNECTING;
    #[allow(dead_code)]
    const OPEN: u16 = W3CWebSocket::OPEN;
    #[allow(dead_code)]
    const CLOSING: u16 = W3CWebSocket::CLOSING;
    #[allow(dead_code)]
    const CLOSED: u16 = W3CWebSocket::CLOSED;

    #[allow(dead_code)]
    pub fn new(url: &str) -> Result<Self> {
        Ok(WebSocket(W3CWebSocket::new(url)?))
    }

    pub fn new_with_config(url: &str, config: &WebSocketConfig) -> Result<Self> {
        Ok(WebSocket(W3CWebSocket::new_with_config(url, config)?))
    }

    fn cleanup(&self) {
        self.set_onopen(None);
        self.set_onclose(None);
        self.set_onerror(None);
        self.set_onmessage(None);
    }
}

impl From<W3CWebSocket> for WebSocket {
    fn from(ws: W3CWebSocket) -> Self {
        WebSocket(ws)
    }
}

struct Settings {
    url: String,
}

#[allow(dead_code)]
struct Inner {
    ws: WebSocket,
    callbacks: CallbackMap,
}

unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

pub struct WebSocketInterface {
    inner: Arc<Mutex<Option<Inner>>>,
    settings: Arc<Mutex<Settings>>,
    reconnect: AtomicBool,
    is_open: AtomicBool,
    event_channel: Channel<Message>,
    sender_channel: Channel<(Message, Ack)>,
    receiver_channel: Channel<Message>,
    handshake: Option<Arc<dyn Handshake>>,
    dispatcher_shutdown: DuplexChannel,
    config: WebSocketConfig,
}

impl WebSocketInterface {
    pub fn new(
        url: &str,
        sender_channel: Channel<(Message, Ack)>,
        receiver_channel: Channel<Message>,
        options: Options,
        config: Option<WebSocketConfig>,
    ) -> Result<WebSocketInterface> {
        sanity_checks()?;

        let settings = Settings {
            url: url.to_string(),
        };

        let iface = WebSocketInterface {
            inner: Arc::new(Mutex::new(None)),
            settings: Arc::new(Mutex::new(settings)),
            sender_channel,
            receiver_channel,
            event_channel: Channel::unbounded(),
            reconnect: AtomicBool::new(true),
            is_open: AtomicBool::new(false),
            handshake: options.handshake,
            dispatcher_shutdown: DuplexChannel::unbounded(),
            config: config.unwrap_or_default(),
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

    // pub async fn connect(self: &Arc<Self>, block: bool) -> Result<Option<Listener>> {
    pub async fn connect(self: &Arc<Self>, options: ConnectOptions) -> ConnectResult<Error> {
        let (connect_trigger, connect_listener) = oneshot::<Result<()>>();

        if let Some(url) = options.url.as_ref() {
            self.set_url(url);
        }

        self.connect_impl(options.clone(), Some(connect_trigger))?;

        match options.block_async_connect {
            true => match connect_listener.recv().await? {
                Ok(_) => Ok(None),
                Err(e) => Err(e),
            },
            false => Ok(Some(connect_listener)),
        }
    }

    fn connect_impl(
        self: &Arc<Self>,
        options: ConnectOptions,
        connect_trigger: Option<Sender<Result<()>>>,
    ) -> Result<()> {
        let mut inner = self.inner.lock().unwrap();
        if inner.is_some() {
            log_warning!("WebSocket::connect() called while already initialized");

            return Err(Error::AlreadyInitialized);
        }

        let connect_trigger = Arc::new(Mutex::new(connect_trigger));

        self.reconnect.store(true, Ordering::SeqCst);
        // let ws_client_config =
        //     WebSocketClientConfig::new().max_received_frame_size(1024 * 1024 * 2);
        let ws = WebSocket::new_with_config(&self.url(), &self.config)?;
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        // - Message
        let event_sender_ = self.event_channel.sender.clone();
        let onmessage = callback!(move |event: WsMessageEvent| {
            let msg: Message = event.try_into().expect("MessageEvent Error");
            event_sender_.try_send(msg).unwrap_or_else(|err| {
                log_trace!("WebSocket unable to try_send() `message` to event channel: `{err}`")
            });
        });
        ws.set_onmessage(Some(onmessage.as_ref()));

        // - Error
        let onerror = callback!(move |_event: WsErrorEvent| {
            // log_trace!("WS - error event: {:?}", _event);
        });
        ws.set_onerror(Some(onerror.as_ref()));

        // - Open
        let event_sender_ = self.event_channel.sender.clone();
        let onopen = callback!(move || {
            event_sender_.try_send(Message::Open).unwrap_or_else(|err| {
                log_trace!("WebSocket unable to try_send() `open` to event channel: `{err}`")
            });
        });
        ws.set_onopen(Some(onopen.as_ref()));

        // - Close
        let event_sender_ = self.event_channel.sender.clone();
        let onclose = callback!(move |_event: WsCloseEvent| {
            // log_trace!("WS - close event: {:?}", _event);
            event_sender_
                .try_send(Message::Close)
                .unwrap_or_else(|err| {
                    log_trace!("WebSocket unable to try_send() `close` to event channel: `{err}`")
                });
        });
        ws.set_onclose(Some(onclose.as_ref()));

        let callbacks = CallbackMap::new();
        callbacks.retain(onmessage)?;
        callbacks.retain(onerror)?;
        callbacks.retain(onopen)?;
        callbacks.retain(onclose)?;

        *inner = Some(Inner {
            ws: ws.clone(),
            callbacks,
        });

        let self_ = self.clone();
        spawn(async move {
            self_
                .dispatcher_task(&ws, options.clone(), connect_trigger)
                .await
                .unwrap_or_else(|err| log_trace!("WebSocket error: {err}"));
            // if reconnect is true, we sleep for reconnect interval and try to reconnect
            if self_.reconnect.load(Ordering::SeqCst) {
                workflow_core::task::sleep(
                    options
                        .retry_interval
                        .unwrap_or(std::time::Duration::from_millis(1000)),
                )
                .await;
                // check again if reconnect may have been disabled during sleep
                if self_.reconnect.load(Ordering::SeqCst) {
                    self_.reconnect().await.ok();
                }
            }
        });

        Ok(())
    }

    fn ws(self: &Arc<Self>) -> Option<WebSocket> {
        self.inner
            .lock()
            .expect("WebSocket:: inner lock failure")
            .as_ref()
            .map(|inner| inner.ws.clone())
    }

    #[allow(dead_code)]
    pub fn try_send(self: &Arc<Self>, message: &Message) -> Result<()> {
        if let Some(ws) = self.ws() {
            ws.try_send(message)?;
            Ok(())
        } else {
            Err(Error::NotConnected)
        }
    }

    async fn handshake(self: &Arc<Self>, ws: &WebSocket) -> Result<()> {
        if let Some(handshake) = self.handshake.as_ref().cloned() {
            let (sender_tx, sender_rx) = unbounded();
            let (receiver_tx, receiver_rx) = unbounded();
            let (accept_tx, accept_rx) = oneshot();

            spawn(async move {
                accept_tx
                    .send(handshake.handshake(&sender_tx, &receiver_rx).await)
                    .await
                    .unwrap_or_else(|err| {
                        log_trace!("WebSocket handshake unable to send completion: `{}`", err)
                    });
            });

            // let handshake_rx = self.handshake_channel.receiver.clone();
            loop {
                select_biased! {
                    result = accept_rx.recv().fuse() => {
                        return result?;
                    },
                    msg = sender_rx.recv().fuse() => {
                        if let Ok(msg) = msg {
                            ws.try_send(&msg)?;
                            // ws_sender.send(msg.into()).await?;
                        }
                    },
                    msg = self.event_channel.recv().fuse() => {
                        if let Ok(msg) = msg {
                            receiver_tx.send(msg).await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn dispatcher_task(
        self: &Arc<Self>,
        ws: &WebSocket,
        options: ConnectOptions,
        connect_trigger: Arc<Mutex<Option<Sender<Result<()>>>>>,
    ) -> Result<()> {
        'outer: loop {
            select! {
                _ = self.dispatcher_shutdown.request.receiver.recv().fuse() => {
                    break 'outer;
                },
                msg = self.event_channel.recv().fuse() => {
                    match msg {
                        Ok(msg) => {
                            match msg {
                                Message::Binary(_) | Message::Text(_) => {
                                    self.receiver_channel.sender.send(msg).await.unwrap();
                                },
                                Message::Open => {
                                    // log_info!("WebSocket connected to {}",self.url());

                                    // handle handshake failure
                                    if let Err(err) = self.handshake(ws).await {
                                        log_info!("WebSocket handshake negotiation error: {err}");

                                        if options.strategy.is_fallback() {
                                            self.reconnect.store(false, Ordering::SeqCst);
                                        }

                                        let connect_trigger = connect_trigger.lock().unwrap().take();
                                        if let Some(connect_trigger) = connect_trigger {
                                            connect_trigger.send(Err(err)).await.ok();
                                        }

                                        return Err(Error::NegotiationFailure);
                                    }

                                    self.is_open.store(true, Ordering::SeqCst);

                                    let connect_trigger = connect_trigger.lock().unwrap().take();
                                    if let Some(connect_trigger) = connect_trigger {
                                        connect_trigger.send(Ok(())).await.ok();
                                    }

                                    self.receiver_channel.sender.send(msg).await.unwrap();
                                },
                                Message::Close => {
                                    // log_info!("WebSocket disconnecting from {}",self.url());

                                    if let Some(inner) = self.inner.lock().unwrap().take() {
                                        inner.ws.cleanup();
                                    }

                                    if self.is_open.load(Ordering::SeqCst) {
                                        self.is_open.store(false, Ordering::SeqCst);
                                        self.receiver_channel.sender.send(msg).await.unwrap();
                                    } else if options.strategy.is_fallback() && options.block_async_connect {
                                        // if we never connected and receiver Close while
                                        // the strategy is Fallback, we disable reconnect
                                        self.reconnect.store(false, Ordering::SeqCst);

                                        let connect_trigger = connect_trigger.lock().unwrap().take();
                                        if let Some(connect_trigger) = connect_trigger {
                                            connect_trigger.send(Err(Error::Connect(self.url()))).await.ok();
                                        }
                                    }

                                    break 'outer;
                                }
                            }
                        }
                        Err(err) => {
                            log_error!("WebSocket dispatcher channel error: {err}");
                        }
                    }
                },
                msg = self.sender_channel.receiver.recv().fuse() => {

                    if let Ok((msg, ack)) = msg {

                        // if ws.ready_state() != WebSocket::OPEN {
                        //     return Err(Error::NotConnected);
                        // }

                        if let Some(ack) = ack {
                            let result = ws
                                .try_send(&msg)
                                .map(Arc::new)
                                .map_err(Arc::new);
                            ack.send(result).await.unwrap_or_else(|err| {
                                log_trace!("WebSocket error producing message ack {:?}", err)
                            });
                        } else {
                            ws.try_send(&msg).unwrap_or_else(|err| {
                                log_trace!("WebSocket unable to send `raw ws` message: `{err}`")
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn _shutdown(self: &Arc<Self>) -> Result<()> {
        self.dispatcher_shutdown
            .signal(())
            .await
            .map_err(|_| Error::DispatcherSignal)?;

        Ok(())
    }

    pub async fn close(self: &Arc<Self>) -> Result<()> {
        if let Some(inner) = self.inner.lock().unwrap().take() {
            inner.ws.cleanup();
            inner.ws.close()?;
        }

        if self.is_open.load(Ordering::SeqCst) {
            self.event_channel.try_send(Message::Close)?;
        }

        Ok(())
    }

    async fn reconnect(self: &Arc<Self>) -> Result<()> {
        self.close().await?;

        self.connect_impl(ConnectOptions::reconnect_defaults(), None)?;

        Ok(())
    }

    pub async fn disconnect(self: &Arc<Self>) -> Result<()> {
        self.reconnect.store(false, Ordering::SeqCst);
        self.close().await.ok();
        Ok(())
    }
}

impl Drop for WebSocketInterface {
    fn drop(&mut self) {}
}

trait TrySendMessage {
    fn try_send(&self, message: &Message) -> Result<()>;
}

impl TrySendMessage for WebSocket {
    fn try_send(&self, message: &Message) -> Result<()> {
        match message {
            Message::Binary(data) => self.send_with_u8_array(data).map_err(|e| e.into()),
            Message::Text(text) => self.send_with_str(text).map_err(|e| e.into()),
            _ => {
                panic!("WebSocket trying to convert unsupported message type: `{message:?}`");
            }
        }
    }
}

fn w3c_websocket_available() -> Result<bool> {
    Ok(js_sys::Reflect::get(&js_sys::global(), &"WebSocket".into())
        .map(|v| !v.is_falsy())
        .unwrap_or(false))
}

fn sanity_checks() -> Result<()> {
    if !w3c_websocket_available()? {
        if is_node() {
            log_info!("");
            log_info!("+------------------------------------------------------------");
            log_info!("|");
            log_info!("| w3c websocket is not available");
            log_info!("|");
            log_info!("| Please include `WebSocket` module as a project dependency");
            log_info!("| and add the following line to your Node.js script:");
            log_info!("|");
            log_info!("| `globalThis.WebSocket = require(\"websocket\").w3cwebsocket;`");
            log_info!("|");
            log_info!("| (or use any other w3c-compatible module)");
            log_info!("|");
            log_info!("+------------------------------------------------------------");
            log_info!("");
        } else {
            log_info!("");
            log_error!("w3c websocket is not available");
            log_info!("");
        }
        panic!("w3c websocket is not available");
    }
    Ok(())
}
