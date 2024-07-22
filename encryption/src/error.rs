use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    // #[error(transparent)]
    // TomlDe(#[from] toml::de::Error),

    // #[error(transparent)]
    // TomlSer(#[from] toml::ser::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    // #[error(transparent)]
    // FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Unable to decrypt")]
    Chacha20poly1305(chacha20poly1305::Error),

    #[error("(Argon2) {0}")]
    Argon2(argon2::Error),

    #[error("(Argon2::password_hash) {0}")]
    Argon2ph(argon2::password_hash::Error),

    #[error("Decryption failed (invalid data length)")]
    DecryptionDataLength,
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Custom(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Custom(s.to_string())
    }
}

impl Error {
    pub fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Self::Argon2(err)
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::Argon2ph(err)
    }
}

impl From<chacha20poly1305::Error> for Error {
    fn from(err: chacha20poly1305::Error) -> Self {
        Self::Chacha20poly1305(err)
    }
}
