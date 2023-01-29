#[cfg(not(target_arch = "wasm32"))]
mod server;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let result = server::server_example().await;
    workflow_log::log_info!("{result:#?}");
}

// suppress build errors for wasm32
#[cfg(target_arch = "wasm32")]
fn main() {}
