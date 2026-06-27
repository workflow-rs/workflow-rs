use thiserror::Error;

/// Errors produced by i18n loading, translation, and storage operations.
#[derive(Error, Debug)]
pub enum Error {
    /// A custom, free-form error message.
    #[error("i18n: {0}")]
    Custom(String),

    /// No translation table exists for the given language code.
    #[error("i18n: missing translation for language code '{0}'")]
    MissingTranslation(String),

    /// No language metadata (title) exists for the given language code.
    #[error("i18n: missing language info for language code '{0}'")]
    MissingLanguage(String),

    /// An attempt was made to enable a language code that is not known.
    #[error("i18n: enabling invalid language code '{0}'")]
    EnablingUnknownLanguageCode(String),

    /// A supplied language code does not correspond to any known language.
    #[error("i18n: received invalid language code '{0}'")]
    UnknownLanguageCode(String),

    /// The storage path for the i18n data file could not be determined.
    #[error("i18n: unable to get storage path")]
    StoragePath,

    /// JSON serialization or deserialization failed.
    #[error("i18n: serde json failure: {0}")]
    JSON(#[from] serde_json::Error),

    /// An underlying I/O operation failed.
    #[error("i18n: io failure: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    /// Construct a [`Error::Custom`] from any value convertible to a string.
    pub fn custom<T: ToString>(s: T) -> Self {
        Error::Custom(s.to_string())
    }
}
