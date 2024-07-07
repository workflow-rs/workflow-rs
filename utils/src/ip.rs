use crate::imports::*;

pub async fn public() -> Result<String> {
    Ok(http::get("https://api.ipify.org").await?)
}

#[cfg(not(target_arch = "wasm32"))]
pub mod blocking {
    use super::*;

    pub fn public() -> Result<String> {
        Ok(reqwest::blocking::get("https://api.ipify.org")?.text()?)
    }
}
