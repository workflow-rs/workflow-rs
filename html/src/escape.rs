use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

pub fn escape_attr<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("[<>&\"]").unwrap();
    }
    let input = input.into();
    let first = REGEX.find(&input);
    if let Some(m) = first {
        let first = m.start();
        let len = input.len();
        let mut output: Vec<u8> = Vec::with_capacity(len + len / 2);
        output.extend_from_slice(input[0..first].as_bytes());
        let rest = input[first..].bytes();
        for c in rest {
            match c {
                b'<' => output.extend_from_slice(b"&lt;"),
                b'>' => output.extend_from_slice(b"&gt;"),
                b'&' => output.extend_from_slice(b"&amp;"),
                b'"' => output.extend_from_slice(b"&quot;"),
                _ => output.push(c),
            }
        }
        Cow::Owned(unsafe { String::from_utf8_unchecked(output) })
    } else {
        input
    }
}
pub fn escape_html<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("[<>&]").unwrap();
    }
    let input = input.into();
    let first = REGEX.find(&input);
    if let Some(m) = first {
        let first = m.start();
        let len = input.len();
        let mut output: Vec<u8> = Vec::with_capacity(len + len / 2);
        output.extend_from_slice(input[0..first].as_bytes());
        let rest = input[first..].bytes();
        for c in rest {
            match c {
                b'<' => output.extend_from_slice(b"&lt;"),
                b'>' => output.extend_from_slice(b"&gt;"),
                b'&' => output.extend_from_slice(b"&amp;"),
                _ => output.push(c),
            }
        }
        Cow::Owned(unsafe { String::from_utf8_unchecked(output) })
    } else {
        input
    }
}
