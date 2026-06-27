use crate::imports::*;

/// Asynchronously resolves the host's public IP address via the ipify service.
pub async fn public() -> Result<String> {
    Ok(http::get("https://api.ipify.org").await?)
}

/// Blocking (non-`wasm32`) variants of the IP lookup helpers.
#[cfg(not(target_arch = "wasm32"))]
pub mod blocking {
    use super::*;

    /// Synchronously resolves the host's public IP address via the ipify service.
    pub fn public() -> Result<String> {
        Ok(reqwest::blocking::get("https://api.ipify.org")?.text()?)
    }
}
