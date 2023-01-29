#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use clap::*;
    use rpc_example_client_common::client_example;
    use std::time::Duration;
    use workflow_log::log_info;

    #[derive(Debug, Parser)]
    struct Args {
        #[clap(short, long)]
        json: bool,
    }

    let Args { json } = Args::parse();

    let result = client_example(json, Duration::from_millis(1000)).await;
    log_info!("{:#?}", result);
}

// suppress build errors for wasm32
#[cfg(target_arch = "wasm32")]
fn main() {}
