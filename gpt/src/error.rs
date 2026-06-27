use thiserror::Error;

/// Errors that can occur while talking to the chat completions API.
#[derive(Error, Debug)]
pub enum Error {
    /// An underlying HTTP request or response failure from `reqwest`.
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// All retry attempts were exhausted; holds the number of retries
    /// performed and the last error message encountered.
    #[error("Failure after {0} retries: {1}")]
    RetryFailure(usize, String),
}
