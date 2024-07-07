use crate::error::Error;
use crate::result::Result;

pub async fn get(url: impl Into<String>) -> Result<String> {
    Request::new(url).get().await
}

pub async fn get_bytes(url: impl Into<String>) -> Result<Vec<u8>> {
    Request::new(url).get_bytes().await
}

pub async fn get_json<T: serde::de::DeserializeOwned + 'static>(
    url: impl Into<String>,
) -> Result<T> {
    Request::new(url).get_json().await
}

pub struct Request {
    pub url: String,
    pub user_agent: Option<String>,
}

impl Request {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            user_agent: None,
        }
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub async fn get(self) -> Result<String> {
        let mut req = reqwest::Client::new().get(&self.url);
        if let Some(user_agent) = self.user_agent {
            req = req.header("User-Agent", user_agent);
        }
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await?;
        if status.is_success() {
            Ok(text)
        } else {
            Err(Error::Custom(format!("{}: {}", status, text)))
        }
    }

    pub async fn get_bytes(self) -> Result<Vec<u8>> {
        let mut req = reqwest::Client::new().get(&self.url);
        if let Some(user_agent) = self.user_agent {
            req = req.header("User-Agent", user_agent);
        }
        let resp = req.send().await?;
        let status = resp.status();
        let bytes = resp.bytes().await?;
        if status.is_success() {
            Ok(bytes.to_vec())
        } else {
            Err(Error::Custom(format!("{}: {:?}", status, bytes)))
        }
    }

    pub async fn get_json<T: serde::de::DeserializeOwned + 'static>(self) -> Result<T> {
        let mut req = reqwest::Client::new().get(&self.url);
        if let Some(user_agent) = self.user_agent {
            req = req.header("User-Agent", user_agent);
        }
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await?;
        if status.is_success() {
            Ok(serde_json::from_str(&text)?)
        } else {
            Err(Error::Custom(format!("{}: {}", status, text)))
        }
    }
}
