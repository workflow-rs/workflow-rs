#[allow(unused_imports)]
use crate::error::Error;
use crate::result::Result;

pub async fn get(url: impl Into<String>) -> Result<String> {
    let _url = url.into();

    todo!();
}

pub async fn get_json<T: serde::de::DeserializeOwned>(url: impl Into<String>) -> Result<T> {
    let _url = url.into();

    todo!();
}
