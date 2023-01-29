#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use terminal_example_cli::example_terminal;

    let result = example_terminal().await;
    if let Err(err) = result {
        println!("{err}");
    }
}

// suppress build errors for wasm32
#[cfg(target_arch = "wasm32")]
fn main() {}
