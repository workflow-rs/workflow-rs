/// A specialized [`Result`](std::result::Result) for service operations,
/// using this crate's [`Error`](crate::error::Error) type.
pub type Result<T> = std::result::Result<T, crate::error::Error>;
