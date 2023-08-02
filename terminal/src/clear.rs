//!
//! Clear line helper (ANSI escape codes to clear the terminal line)
//!

use std::fmt;

pub struct ClearLine;

impl fmt::Display for ClearLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[2K\r")
    }
}

impl AsRef<[u8]> for ClearLine {
    fn as_ref(&self) -> &'static [u8] {
        "\x1B[2K\r".as_bytes()
    }
}

impl AsRef<str> for ClearLine {
    fn as_ref(&self) -> &'static str {
        "\x1B[2K\r"
    }
}

pub struct ClearScreen;

impl fmt::Display for ClearScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[2J")
    }
}

impl AsRef<[u8]> for ClearScreen {
    fn as_ref(&self) -> &'static [u8] {
        "\x1B[2J".as_bytes()
    }
}

impl AsRef<str> for ClearScreen {
    fn as_ref(&self) -> &'static str {
        "\x1B[2J"
    }
}
