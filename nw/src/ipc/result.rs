/// Result type used throughout the IPC module, with [`Error`](super::error::Error) as the error type.
pub type Result<T> = std::result::Result<T, super::error::Error>;
/// Result type returned by IPC method and notification handlers, using
/// [`ResponseError`](super::error::ResponseError) for failures conveyed back to the caller.
pub type ResponseResult<T> = std::result::Result<T, super::error::ResponseError>;
