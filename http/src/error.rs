use thiserror::Error;

/// Errors produced by the HTTP client operations in this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// A custom, free-form error message, including non-success HTTP responses.
    #[error("Custom: {0}")]
    Custom(String),

    /// An error originating from the underlying `reqwest` HTTP client.
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// An error serializing or deserializing JSON.
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),

    /// The requested operation is not implemented on this platform.
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
    /// Creates a [`Error::Custom`] error from any value convertible to a string.
    pub fn custom<T: ToString>(s: T) -> Self {
        Error::Custom(s.to_string())
    }
}
