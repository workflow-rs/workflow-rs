#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use std::time::Duration;
    use websocket_example_client_common::client_example;
    use workflow_log::log_info;

    let result = client_example(Duration::from_millis(1000)).await;
    log_info!("{:#?}", result);
}

// suppress build errors for wasm32
#[cfg(target_arch = "wasm32")]
fn main() {}
