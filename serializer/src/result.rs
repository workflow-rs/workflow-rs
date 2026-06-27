/// Alias for the standard library I/O error type.
pub type IoError = std::io::Error;
/// Alias for the standard library I/O error kind enumeration.
pub type IoErrorKind = std::io::ErrorKind;
/// Alias for a `Result` whose error is a standard library I/O error.
pub type IoResult<T> = std::io::Result<T>;
