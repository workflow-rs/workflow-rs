pub trait CrLf {
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
