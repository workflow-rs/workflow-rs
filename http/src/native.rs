use crate::error::Error;
use crate::result::Result;

/// Performs a GET request to `url` and returns the response body as a string.
pub async fn get(url: impl Into<String>) -> Result<String> {
    Request::new(url).get().await
}

/// Performs a GET request to `url` and returns the response body as raw bytes.
pub async fn get_bytes(url: impl Into<String>) -> Result<Vec<u8>> {
    Request::new(url).get_bytes().await
}

/// Performs a GET request to `url` and deserializes the response body from
/// JSON into `T`.
pub async fn get_json<T: serde::de::DeserializeOwned + 'static>(
    url: impl Into<String>,
) -> Result<T> {
    Request::new(url).get_json().await
}

/// Builder for an HTTP GET request, holding the target URL and optional
/// headers before the request is sent.
pub struct Request {
    /// The target URL to request.
    pub url: String,
    /// Optional value for the `User-Agent` request header.
    pub user_agent: Option<String>,
}

impl Request {
    /// Creates a new request targeting `url` with no custom headers set.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            user_agent: None,
        }
    }

    /// Sets the `User-Agent` header to send with the request and returns the
    /// updated request for chaining.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Sends the configured GET request and returns the response body as a
    /// string. Returns an error on a non-success HTTP status.
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

    /// Sends the configured GET request and returns the response body as raw
    /// bytes. Returns an error on a non-success HTTP status.
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

    /// Sends the configured GET request and deserializes the response body
    /// from JSON into `T`. Returns an error on a non-success HTTP status or
    /// if deserialization fails.
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
