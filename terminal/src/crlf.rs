/// Trait for converting text with `\n` line endings into the `\r\n`
/// line endings expected by raw terminal output.
pub trait CrLf {
    /// Returns a copy of the string with every `\n` replaced by `\r\n`.
    fn crlf(&self) -> String;
}

impl CrLf for str {
    fn crlf(&self) -> String {
        self.replace('\n', "\r\n")
    }
}

impl CrLf for String {
    fn crlf(&self) -> String {
        self.replace('\n', "\r\n")
    }
}
