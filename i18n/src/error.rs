use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("i18n: {0}")]
    Custom(String),

    #[error("i18n: missing translation for language code '{0}'")]
    MissingTranslation(String),

    #[error("i18n: missing language info for language code '{0}'")]
    MissingLanguage(String),

    #[error("i18n: enabling invalid language code '{0}'")]
    EnablingUnknownLanguageCode(String),

    #[error("i18n: received invalid language code '{0}'")]
    UnknownLanguageCode(String),

    #[error("i18n: unable to get storage path")]
    StoragePath,

    #[error("i18n: serde json failure: {0}")]
    JSON(#[from] serde_json::Error),

    #[error("i18n: io failure: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    pub fn custom<T: ToString>(s: T) -> Self {
        Error::Custom(s.to_string())
    }
}
