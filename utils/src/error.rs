use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
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
    pub fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
