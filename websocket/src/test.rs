use crate::client::{ConnectOptions, Message as ClientMessage, WebSocket};
use crate::server::{
    Message as ServerMessage, Result as ServerResult, WebSocketHandler, WebSocketReceiver,
    WebSocketSender, WebSocketServer, WebSocketSink,
};
use async_trait::async_trait;
use std::net::SocketAddr;
use std::sync::Arc;
use thiserror::Error;
use workflow_core::task::spawn;
use workflow_log::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    WebSocketClient(#[from] crate::client::Error),

    #[error(transparent)]
    WebSocketServer(#[from] crate::server::Error),
}

type Result<T> = std::result::Result<T, Error>;

// Struct representing a websocket connection
pub struct MyContext {
    pub peer: SocketAddr,
}

// A simple WebSocket handler struct
pub struct EchoWsHandler;

#[async_trait]
impl WebSocketHandler for EchoWsHandler {
    type Context = Arc<MyContext>;

    // store peer address for each connection into context
    async fn connect(self: &Arc<Self>, _peer: &SocketAddr) -> ServerResult<()> {
        // let ctx = MyContext { peer };
        // Ok(Arc::new(ctx))
        Ok(())
    }

    async fn handshake(
        self: &Arc<Self>,
        peer: &SocketAddr,
        _sender: &mut WebSocketSender,
        _receiver: &mut WebSocketReceiver,
        _sink: &WebSocketSink,
    ) -> ServerResult<Arc<MyContext>> {
        let ctx = MyContext { peer: *peer };
        Ok(Arc::new(ctx))
    }

    // receive and echo text and binary messages
    // while logging the ip address and received data
    async fn message(
        self: &Arc<Self>,
        ctx: &Self::Context,
        msg: ServerMessage,
        sink: &WebSocketSink,
    ) -> ServerResult<()> {
        match &msg {
            ServerMessage::Binary(data) => {
                log_debug!("[SERVER MSG] [{}] {:?}", ctx.peer, data);
                sink.send(msg)?;
            }
            ServerMessage::Text(text) => {
                log_debug!("[SERVER MSG] [{}] {}", ctx.peer, text);
                sink.send(msg)?;
            }
            _ => {
                log_debug!("[SERVER MSG] [{}] {:?}", ctx.peer, msg);
            }
        }
        Ok(())
    }
}

struct Sink;

impl workflow_log::Sink for Sink {
    fn write(&self, _target: Option<&str>, level: Level, args: &std::fmt::Arguments<'_>) -> bool {
        println!("[{level:>5}] {args}");
        let text = format!("{args}");
        if text.contains("WebSocket failed to connect to") {
            panic!("{text}");
        }
        true
    }
}

#[tokio::test]
async fn websocket_test() -> Result<()> {
    let sink = Sink {};
    workflow_log::pipe(Some(Arc::new(sink)));
    workflow_log::set_log_level(workflow_log::LevelFilter::Trace);

    let addr = "127.0.0.1:19111";
    log_debug!("WebSocket server is listening on {}", addr);

    let handler = Arc::new(EchoWsHandler {});
    let ws_server = WebSocketServer::<EchoWsHandler>::new(handler, None);
    let listener = ws_server.bind(addr).await?;
    let ws_server_ = ws_server.clone();
    spawn(async move {
        let result = ws_server_.listen(listener, None).await;
        log_debug!("Server stopped with result: {:?}", result);
    });

    let ws_client = WebSocket::new(Some("ws://localhost:19111"), None)?;
    ws_client
        .connect(ConnectOptions::blocking_fallback())
        .await?;

    let text_out = "Hello, world!";
    ws_client
        .send(ClientMessage::Text(text_out.to_string()))
        .await
        .expect("Error sending message");

    let (sender, receiver) = workflow_core::channel::oneshot::<()>();
    spawn(async move {
        loop {
            let message = ws_client.recv().await.unwrap();
            log_debug!("[CLIENT MSG]: {:?}", message);
            match &message {
                ClientMessage::Open => {}
                ClientMessage::Text(text_in) => {
                    // log_debug!("Client received message: {:?}", text_in);
                    assert_eq!(text_in, "Hello, world!");
                    // log_debug!("Shutting down server...");
                    ws_client.disconnect().await.unwrap();
                    ws_server.stop_and_join().await.unwrap();
                    // log_debug!("Server has been shutdown...");
                }
                ClientMessage::Close => {
                    break;
                }
                _ => panic!("Unexpected message: {:?}", message),
            }
        }

        sender.send(()).await.unwrap();
    });

    receiver.recv().await.unwrap();

    Ok(())
}
