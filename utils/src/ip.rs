
use crate::imports::*;

pub fn public_ip() -> Result<String> {
    Ok(reqwest::blocking::get("https://api.ipify.org")?.text()?)
}
