use async_trait::async_trait;
use std::net::SocketAddr;
use std::sync::Arc;
// use tokio::sync::mpsc::*;
// use tungstenite::Message;
use workflow_log::*;
use workflow_websocket::server::{
    Message, Result, WebSocketHandler, WebSocketReceiver, WebSocketSender, WebSocketServer,
    WebSocketSink,
};

// Struct representing a websocket connection
pub struct MyContext {
    pub peer: SocketAddr,
}

// A simple WebSocket handler struct
pub struct MyWsHandler;

#[async_trait]
impl WebSocketHandler for MyWsHandler {
    type Context = Arc<MyContext>;

    // store peer address for each connection into context
    async fn connect(self: &Arc<Self>, _peer: &SocketAddr) -> Result<()> {
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
    ) -> Result<Arc<MyContext>> {
        let ctx = MyContext { peer: *peer };
        Ok(Arc::new(ctx))
    }

    // receive and echo text and binary messages
    // while logging the ip address and received data
    async fn message(
        self: &Arc<Self>,
        ctx: &Self::Context,
        msg: Message,
        sink: &WebSocketSink,
    ) -> Result<()> {
        match &msg {
            Message::Binary(data) => {
                log_trace!("[{}] {:?}", ctx.peer, data);
                sink.send(msg)?;
            }
            Message::Text(text) => {
                log_trace!("[{}] {}", ctx.peer, text);
                sink.send(msg)?;
            }
            _ => {}
        }
        Ok(())
    }
}

struct Sink;

impl workflow_log::Sink for Sink {
    fn write(&self, _target: Option<&str>, level: Level, args: &std::fmt::Arguments<'_>) -> bool {
        println!("[{level}] {args}");
        true
    }
}

pub async fn server_example() -> Result<()> {
    let sink = Sink {};
    workflow_log::pipe(Some(Arc::new(sink)));
    workflow_log::set_log_level(workflow_log::LevelFilter::Trace);

    let addr = "127.0.0.1:9090";
    log_info!("WebSocket server is listening on {}", addr);

    // create our handler instance
    let handler = Arc::new(MyWsHandler {});
    // create websocket server and install our handler in it
    let ws = WebSocketServer::<MyWsHandler>::new(handler, None);
    // listen for incoming connections
    let listener = ws.bind(addr).await?;
    ws.listen(listener, None).await?;

    Ok(())
}
