//! Cursor helper structs (from [https://crates.io/crates/termion](https://crates.io/crates/termion))

use numtoa::NumToA;
use std::fmt;

/// Move cursor left.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Left(pub u16);

impl From<Left> for String {
    fn from(this: Left) -> String {
        let mut buf = [0u8; 20];
        ["\x1B[", this.0.numtoa_str(10, &mut buf), "D"].concat()
    }
}

impl fmt::Display for Left {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}D", self.0)
    }
}

/// Move cursor right.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Right(pub u16);

impl From<Right> for String {
    fn from(this: Right) -> String {
        let mut buf = [0u8; 20];
        ["\x1B[", this.0.numtoa_str(10, &mut buf), "C"].concat()
    }
}

impl fmt::Display for Right {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}C", self.0)
    }
}
