use async_trait::async_trait;
use clap::*;
use rpc_example_messages::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use workflow_core::task::*;
use workflow_log::*;
use workflow_rpc::server::prelude::*;
use workflow_rpc::server::result::Result as RpcServerResult;
use workflow_task::*;

#[derive(Debug)]
pub struct ConnectionContext {
    pub peer: SocketAddr,
    pub messenger: Arc<Messenger>,
}

struct ExampleRpcHandler {
    pub sockets: Mutex<HashMap<SocketAddr, Arc<ConnectionContext>>>,
}

impl ExampleRpcHandler {
    pub fn new() -> ExampleRpcHandler {
        ExampleRpcHandler {
            sockets: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl RpcHandler for ExampleRpcHandler {
    type Context = Arc<ConnectionContext>;

    async fn connect(self: Arc<Self>, _peer: &SocketAddr) -> WebSocketResult<()> {
        Ok(())
    }

    async fn handshake(
        self: Arc<Self>,
        peer: &SocketAddr,
        _sender: &mut WebSocketSender,
        _receiver: &mut WebSocketReceiver,
        messenger: Arc<Messenger>,
    ) -> WebSocketResult<Arc<ConnectionContext>> {
        let ctx = Arc::new(ConnectionContext {
            peer: *peer,
            messenger,
        });
        self.sockets.lock().unwrap().insert(*peer, ctx.clone());
        Ok(ctx)
    }

    async fn disconnect(self: Arc<Self>, ctx: Self::Context, _result: WebSocketResult<()>) {
        self.sockets.lock().unwrap().remove(&ctx.peer);
    }
}

pub struct ServerContext;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    json: bool,
}

pub async fn server_main() -> RpcServerResult<()> {
    let Args { json } = Args::parse();

    let addr = "127.0.0.1:9292";
    log_info!("wRPC server is listening on {}", addr);

    let server_ctx = Arc::new(ServerContext);

    let mut interface =
        Interface::<Arc<ServerContext>, Arc<ConnectionContext>, TestOps>::new(server_ctx);

    interface.method(
        TestOps::EvenOdd,
        method!(|_connection_ctx, _server_ctx, req: TestReq| async move {
            if req.v & 1 == 0 {
                Ok(TestResp::Even(req.v))
            } else {
                Ok(TestResp::Odd(req.v))
            }
        }),
    );

    interface.method(
        TestOps::Increase,
        method!(|_connection_ctx, _server_ctx, req: TestReq| async move {
            Ok(TestResp::Increase(req.v + 100))
        }),
    );

    interface.notification(
        TestOps::Notify,
        notification!(
            |_connection_ctx, _server_ctx, _req: TestNotify| async move {
                // Ok(TestResp::Increase(req.v + 100))
                Ok(())
            }
        ),
    );

    let interface = Arc::new(interface);

    let handler = Arc::new(ExampleRpcHandler::new());
    let encoding = if json {
        Encoding::SerdeJson
    } else {
        Encoding::Borsh
    };
    let rpc = RpcServer::new_with_encoding::<
        Arc<ServerContext>,
        Arc<ConnectionContext>,
        TestOps,
        Id64,
    >(encoding, handler.clone(), interface.clone(), None);

    let task = task!(|handler: Arc<ExampleRpcHandler>, stop| async move {
        let mut seq = 0;
        loop {
            if !stop.is_empty() {
                break;
            }

            log_info!("notify bcast");
            let sockets = handler
                .sockets
                .lock()
                .unwrap()
                .values()
                .cloned()
                .collect::<Vec<_>>();
            for ctx in sockets.iter() {
                ctx.messenger
                    .notify(TestOps::Notify, TestNotify::Seq(seq))
                    .await
                    .unwrap_or_else(|err| {
                        log_error!("error posting notification: {err}");
                    });

                if seq % 5 == 0 {
                    ctx.messenger
                        .close()
                        .unwrap_or_else(|err| log_error!("Error closing connection: {err}"));
                }
            }
            // handler.notify();

            sleep(Duration::from_millis(1000)).await;
            seq += 1;
        }
    });

    task.run(handler)?;

    rpc.listen(addr, None).await?;

    task.stop_and_join().await?;

    Ok(())
}
