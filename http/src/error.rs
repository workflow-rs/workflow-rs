use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Custom: {0}")]
    Custom(String),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Not implemented")]
    NotImplemented,
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Custom(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Custom(s.to_string())
    }
}

impl Error {
    pub fn custom<T: ToString>(s: T) -> Self {
        Error::Custom(s.to_string())
    }
}
