use crate::error::Error;
use crate::result::Result;

pub async fn get(url: impl Into<String>) -> Result<String> {
    let url = url.into();
    let resp = reqwest::get(&url).await?;
    let status = resp.status();
    let text = resp.text().await?;
    if status.is_success() {
        Ok(text)
    } else {
        Err(Error::Custom(format!("{}: {}", status, text)))
    }
}

pub async fn get_json<T: serde::de::DeserializeOwned>(url: impl Into<String>) -> Result<T> {
    let url = url.into();
    let resp = reqwest::get(&url).await?;
    let status = resp.status();
    let text = resp.text().await?;
    if status.is_success() {
        Ok(serde_json::from_str(&text)?)
    } else {
        Err(Error::Custom(format!("{}: {}", status, text)))
    }
}

// pub async fn post(url : impl Into<String>, body : impl Into<String>) -> Result<String> {
//     let url = url.into();
//     let body = body.into();
//     let resp = reqwest::Client::new().post(&url).body(body).send().await?;
//     let status = resp.status();
//     let text = resp.text().await?;
//     if status.is_success() {
//         Ok(text)
//     } else {
//         Err(Error::Custom(format!("{}: {}", status, text)))
//     }
// }

// pub async fn post_json(url : impl Into<String>, body : impl Into<String>) -> Result<String> {
//     let url = url.into();
//     let body = body.into();
//     let resp = reqwest::Client::new().post(&url).body(body).header("Content-Type", "application/json").send().await?;
//     let status = resp.status();
//     let text = resp.text().await?;
//     if status.is_success() {
//         Ok(text)
//     } else {
//         Err(Error::Custom(format!("{}: {}", status, text)))
//     }
// }

// pub async fn post_json_with_auth(url : impl Into<String>, body : impl Into<String>, auth : impl Into<String>) -> Result<String> {
//     let url = url.into();
//     let body = body.into();
//     let auth = auth.into();
//     let resp = reqwest::Client::new().post(&url).body(body).header("Content-Type", "application/json").header("Authorization", auth).send().await?;
//     let status = resp.status();
//     let text = resp.text().await?;
//     if status.is_success() {
//         Ok(text)
//     } else {
//         Err(Error::Custom(format!("{}: {}", status, text)))
//     }
// }
