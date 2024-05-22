use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failure after {0} retries: {1}")]
    RetryFailure(usize, String),
}
