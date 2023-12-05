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
        todo!();
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        todo!();
    }
}
