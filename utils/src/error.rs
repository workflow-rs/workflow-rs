use thiserror::Error;

/// Errors produced by the `workflow-utils` crate.
#[derive(Error, Debug)]
pub enum Error {
    /// A free-form error carrying a custom message.
    #[error("{0}")]
    Custom(String),

    /// An underlying I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// An error from the `reqwest` HTTP client.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// An invalid HTTP header name was supplied.
    #[error(transparent)]
    InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),

    /// An invalid HTTP header value was supplied.
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// Failed to parse an integer (e.g. while parsing a version component).
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    /// An error from the `workflow-http` crate.
    #[error(transparent)]
    Http(#[from] workflow_http::error::Error),
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
    /// Constructs a [`Error::Custom`] from any displayable value.
    pub fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

// impl From<reqwest::Error> for Error {
//     fn from(err: reqwest::Error) -> Self {
//         Self::Reqwest(err)
//     }
// }
