use rpc_example_messages::*;
use workflow_core::time::Duration;
use workflow_log::*;
#[allow(unused_imports)]
use workflow_rpc::client::{prelude::*, result::Result};

pub async fn client_example(json: bool, message_delay: Duration) -> Result<()> {
    let encoding = if json {
        Encoding::JSON
    } else {
        Encoding::Borsh
    };

    let mut interface = Interface::<TestOps>::new();
    interface.notification(
        TestOps::Notify,
        notification!(|msg: TestNotify| async move {
            log_info!("received notification: {:?}", msg);
            Ok(())
        }),
    );

    let url = "ws://localhost:9292";
    let rpc = RpcClient::<TestOps>::new_with_encoding(
        encoding,
        interface.into(),
        RpcClientOptions {
            url: Some(url),
            ..RpcClientOptions::default()
        },
        None,
    )?;

    log_info!("Connecting to {url}");
    rpc.connect(ConnectOptions::default()).await?;
    log_info!("Connection ok!");

    let mut seq = 0;
    loop {
        log_info!("calling rpc method");
        let resp: Result<TestResp> = rpc.call(TestOps::EvenOdd, TestReq { v: seq }).await;
        log_info!("call done...");
        match resp {
            Ok(_) => {
                log_info!("call ok - response: {:?}", resp);
            }
            Err(err) => {
                log_info!("call failed: {:?}", err);
            }
        }

        workflow_core::task::sleep(message_delay).await;

        seq += 1;
    }
}
