use crate::error::Error;
use crate::result::Result;
use ritehash::FxHasher64;
use std::collections::HashMap;
use std::fs::read_dir;
use std::hash::BuildHasherDefault;
use std::path::{Path, PathBuf};
use std::sync::Arc;
pub type FxBuildHasher = BuildHasherDefault<FxHasher64>;
pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

static mut JSON_DATA: Option<String> = None;
static mut DICTIONARY: Option<Dictionary> = None;
static mut MISSING_ENTRIES_FILENAME: Option<PathBuf> = None;

/// Initializes the i18n subsystem. This function must be called at the beginning of the program execution.
pub fn try_init(
    current_code: impl Into<String>,
    default_code: impl Into<String>,
    json_data: Option<&'static str>,
    missing_entries_path: Option<impl Into<PathBuf>>,
) -> Result<()> {
    let dictionary = Dictionary::try_new(json_data, current_code, default_code)?;
    let default_code = dictionary.default_code();
    let missing_entries_filename =
        missing_entries_path.map(|s| s.into().join(format!("{default_code}.json")));

    unsafe {
        DICTIONARY = Some(dictionary);
        MISSING_ENTRIES_FILENAME = missing_entries_filename;
    }

    Ok(())
}

/// Obtain the default path to the i18n storage directory.
pub fn storage_path() -> Result<PathBuf> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            None
        } else {
            let mut path = std::env::current_exe()?;
            path.pop();
            if path.ends_with("debug") || path.ends_with("release") {
                path.pop();
                if path.ends_with("target") {
                    path.pop();
                }
                path.push("i18n");
                Ok(path)
            } else {
                Ok(std::env::current_dir()?)
            }
        }
    }
}

/// Create the default i18n configuration file `i18n.conf` in the supplied folder path.
pub fn create(i18n_folder: impl Into<PathBuf>) -> Result<()> {
    let i18n_folder = i18n_folder.into();
    let target_filename = i18n_folder.join("i18n.data");
    std::fs::write(
        target_filename,
        serde_json::to_string_pretty(&Data::default())?,
    )?;
    Ok(())
}

/// Merge all files ending with `_XX.json` (where `XX` is a language code) into a single file `i18n.json`
pub fn merge(
    source_storage_path: Option<impl Into<PathBuf>>,
    destination_storage_path: Option<impl Into<PathBuf>>,
) -> Result<()> {
    let suffixes = dictionary()
        .language_codes()
        .iter()
        .map(|code| (code.to_string(), format!("_{code}.json")))
        .collect::<Vec<_>>();
    let source_storage_path = source_storage_path
        .map(|s| s.into())
        .unwrap_or(storage_path()?);
    let destination_storage_path = destination_storage_path
        .map(|s| s.into())
        .unwrap_or(source_storage_path.clone());

    let files = read_dir(source_storage_path.clone())?
        .map(|result| result.map(|entry| entry.path()))
        .filter_map(|result| {
            result
                .map(|path| {
                    suffixes.iter().find_map(|(code, suffix)| {
                        if path.is_file()
                            && path
                                .as_os_str()
                                .to_str()
                                .map(|s| s.ends_with(suffix))
                                .unwrap_or(false)
                        {
                            Some((code, path.clone()))
                        } else {
                            None
                        }
                    })
                })
                .transpose()
        })
        .map(|result| result.map_err(Error::from))
        .collect::<Result<Vec<_>>>()?;

    let merged_translations = files
        .into_iter()
        .map(|(code, path)| {
            std::fs::read_to_string(path)
                .map_err(Error::from)
                .and_then(|json_data| {
                    serde_json::from_str::<FxHashMap<String, String>>(json_data.as_str())
                        .map_err(Error::from)
                }) //;//.collect
                .map(|translation| (code, translation))
        })
        .collect::<Result<FxHashMap<_, _>>>()?;

    let dictionary = dictionary();
    let i18n_data_filename = destination_storage_path.join("i18n.json");

    static mut DATA: Option<String> = None;

    let (enabled, aliases, languages) = if i18n_data_filename.exists() {
        let Data {
            enabled,
            aliases,
            languages,
            ..
        } = unsafe {
            DATA = Some(std::fs::read_to_string(&i18n_data_filename)?);
            serde_json::from_str::<Data>(DATA.as_ref().unwrap())?
        };
        (enabled, aliases, languages)
    } else {
        (
            dictionary.enabled.clone(),
            dictionary.aliases.clone(),
            dictionary.languages.clone(),
        )
    };
    let data = Data {
        enabled,
        aliases,
        languages,
        translations: merged_translations
            .iter()
            .map(|(code, translation)| {
                (
                    code.as_str(),
                    Arc::new(
                        translation
                            .iter()
                            .map(|(k, v)| (k.as_str(), v.as_str()))
                            .collect(),
                    ),
                )
            })
            .collect(),
    };

    std::fs::write(i18n_data_filename, serde_json::to_string_pretty(&data)?)?;

    unsafe {
        DATA = None;
    }

    Ok(())
}

#[inline(always)]
pub fn dictionary() -> &'static mut Dictionary {
    unsafe {
        DICTIONARY
            .as_mut()
            .expect("i18n dictionary is not initialized")
    }
}

fn missing_entries_filename() -> Option<&'static PathBuf> {
    unsafe { MISSING_ENTRIES_FILENAME.as_ref() }
}

/// Store existing and missing default language entries to a file.
pub fn store_default_dictionary(path: &Path) -> Result<()> {
    let dict = dictionary();
    let source = dict.default_translations();
    let merged = source
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .chain(dict.missing.iter().map(|(k, v)| (k.clone(), v.clone())))
        .collect::<FxHashMap<String, String>>();
    std::fs::write(path, serde_json::to_string_pretty(&merged)?)?;

    Ok(())
}

pub fn store(i18n_folder: impl Into<PathBuf>) -> Result<()> {
    let i18n_file = i18n_folder.into().join("i18n.json");
    std::fs::write(i18n_file, to_string()?)?;
    Ok(())
}

pub fn to_string() -> Result<String> {
    let data = Data::from(&*dictionary());
    Ok(serde_json::to_string_pretty(&data)?)
}

pub fn load(i18n_folder: impl Into<PathBuf>) -> Result<()> {
    let i18n_file = i18n_folder.into().join("i18n.json");
    from_string(std::fs::read_to_string(i18n_file)?)?;

    Ok(())
}

pub fn from_string(json_data: impl Into<String>) -> Result<()> {
    let current_code = dictionary().current_code().to_string();
    let default_code = dictionary().default_code().to_string();

    unsafe {
        JSON_DATA = Some(json_data.into());
        DICTIONARY = Some(Dictionary::try_new(
            Some(JSON_DATA.as_ref().unwrap().as_str()),
            current_code,
            default_code,
        )?);
    }

    Ok(())
}

/// Translate a string to the currently user-selected language.
pub fn i18n(text: &str) -> &str {
    let translation = dictionary().current_translations().get(text);

    match translation {
        Some(translated) => translated,
        None => {
            let dict = dictionary();
            if !dict.missing.contains_key(text) {
                dict.missing.insert(text.to_string(), text.to_string());

                if let Some(path) = missing_entries_filename() {
                    if let Err(err) = store_default_dictionary(path) {
                        println!("i18n io error: {}", err);
                    }
                }
            }

            text
        }
    }
}

/// Dictionary structure containing all translations and related data.
pub struct Dictionary {
    /// list of languages {"en": "English", "ja": "日本語", ..."}
    languages: FxHashMap<&'static str, &'static str>,
    /// list of aliases ("en-GB": "en"), ("en-US": "en"), ("zh-CN": "zh_HANS"), ("zh-TW": "zh_HANT") etc.
    aliases: FxHashMap<&'static str, &'static str>,
    /// Map of translations {"ja": { "Hello" : "こんにちは" }}
    translations: FxHashMap<&'static str, Arc<FxHashMap<&'static str, &'static str>>>,
    /// Missing translation entries (default language)
    missing: FxHashMap<String, String>,
    /// Enabled language codes ["en", "ja"]
    enabled: Vec<&'static str>,
    /// Current language code
    current_code: String,
    /// Current language title
    current_title: String,
    /// Current language translations {"Hello" : "こんにちは", ...}
    current_translations: Arc<FxHashMap<&'static str, &'static str>>,
    /// Default language code (the language used in the source code)
    default_code: String,
    /// Default language translations {"Hello" : "Hello", ...}
    default_translations: Arc<FxHashMap<&'static str, &'static str>>,
}

impl Dictionary {
    /// Create a new dictionary from JSON data. JSON data must be `&'static str`, i.e. loaded by the application via the `include_str!()` macro.
    fn try_new(
        json_data: Option<&'static str>,
        current_code: impl Into<String>,
        default_code: impl Into<String>,
    ) -> Result<Self> {
        let Data {
            enabled,
            languages,
            aliases,
            translations,
        } = if let Some(json_data) = json_data {
            serde_json::from_str::<Data>(json_data)?
        } else {
            Data::default()
        };

        let current_code: String = current_code.into();
        let current_code = aliases
            .get(current_code.as_str())
            .copied()
            .unwrap_or(current_code.as_str())
            .to_string();
        let current_title = languages
            .get(current_code.as_str())
            .ok_or(Error::UnknownLanguageCode(current_code.to_string()))?
            .to_string();
        let current_translations = translations
            .get(current_code.as_str())
            .ok_or(Error::UnknownLanguageCode(current_code.to_string()))?
            .clone();

        let default_code: String = default_code.into();
        let default_code = aliases
            .get(default_code.as_str())
            .copied()
            .unwrap_or(default_code.as_str())
            .to_string();
        let default_translations = translations
            .get(default_code.as_str())
            .ok_or(Error::UnknownLanguageCode(default_code.to_string()))?
            .clone();

        for code in enabled.iter() {
            if !languages.contains_key(code) {
                return Err(Error::EnablingUnknownLanguageCode(code.to_string()));
            }
        }

        Ok(Dictionary {
            languages,
            aliases,
            translations,
            missing: FxHashMap::default(),
            enabled,
            current_code,
            current_title,
            current_translations,
            default_code,
            default_translations,
        })
    }

    /// Resolve a language code or a language alias to a language code.
    fn resolve_aliases(&self, maybe_alias: impl Into<String>) -> Result<String> {
        let maybe_alias = maybe_alias.into();
        match self.aliases.get(maybe_alias.as_str()) {
            Some(code) => Ok(code.to_string()),
            None => {
                if self.languages.contains_key(maybe_alias.as_str()) {
                    Ok(maybe_alias)
                } else {
                    Err(Error::UnknownLanguageCode(maybe_alias))
                }
            }
        }
    }

    #[inline(always)]
    pub fn current_code(&self) -> &str {
        &self.current_code
    }

    #[inline(always)]
    pub fn current_title(&self) -> &str {
        &self.current_title
    }

    #[inline(always)]
    pub fn default_code(&self) -> &str {
        &self.default_code
    }

    #[inline(always)]
    pub fn current_translations(&self) -> &Arc<FxHashMap<&'static str, &'static str>> {
        &self.current_translations
    }

    #[inline(always)]
    pub fn default_translations(&self) -> &Arc<FxHashMap<&'static str, &'static str>> {
        &self.default_translations
    }

    pub fn language_title(&self, language_code: impl Into<String>) -> Result<&str> {
        let language_code: String = language_code.into();
        if let Some(lang) = self.languages.get(language_code.as_str()) {
            Ok(*lang)
        } else {
            Err(Error::UnknownLanguageCode(language_code))
        }
    }

    pub fn activate_language_code(&mut self, language_code: impl Into<String>) -> Result<()> {
        let language_code: String = language_code.into();
        let current_code = self.resolve_aliases(language_code.as_str())?;
        let current_title = self.language_title(current_code.as_str())?.to_string();
        let current_translations = dictionary()
            .translations
            .get(language_code.as_str())
            .ok_or(Error::UnknownLanguageCode(language_code))?
            .clone();

        self.current_code = current_code;
        self.current_title = current_title;
        self.current_translations = current_translations;

        Ok(())
    }

    /// Obtain a list of available language codes.
    pub fn language_codes(&self) -> Vec<String> {
        self.languages.keys().map(|s| s.to_string()).collect()
    }

    /// Obtain a list of enabled languages as `(code, name)` pairs.
    pub fn enabled_languages(&self) -> Vec<(&'static str, &'static str)> {
        self.enabled
            .clone()
            .into_iter()
            .map(|s| (s, *self.languages.get(s).unwrap()))
            .collect()
    }
}

/// i18n.json data file structure.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = "'de: 'data"))]
pub struct Data<'data> {
    enabled: Vec<&'data str>,
    aliases: FxHashMap<&'data str, &'data str>,
    languages: FxHashMap<&'data str, &'data str>,
    translations: FxHashMap<&'data str, Arc<FxHashMap<&'data str, &'data str>>>,
}

impl<'data> Default for Data<'data> {
    fn default() -> Self {
        let languages: FxHashMap<_, _> = [
            ("af", "Afrikaans"),
            ("ar", "Arabic"),
            ("bg", "Bulgarian"),
            ("bn", "Bengali"),
            ("en", "English"),
            ("es", "Español"),
            ("el", "Greek"),
            ("et", "Esti"),
            ("fr", "Français"),
            ("de", "Deutsch"),
            ("da", "Danish"),
            ("cs", "Czech"),
            ("fa", "Farsi"),
            ("fi", "Finnish"),
            ("fil", "Filipino"),
            ("he", "Hebrew"),
            ("hi", "Hindi"),
            ("hr", "Croatian"),
            ("hu", "Hungarian"),
            ("it", "Italiano"),
            ("is", "Icelandic"),
            ("ja", "日本語"),
            ("ko", "Korean"),
            ("lt", "Lithuanian"),
            ("nb", "Norwegian"),
            ("nl", "Dutch"),
            ("no", "Norwegian"),
            ("pa", "Panjabi"),
            ("pl", "Polish"),
            ("pt", "Português"),
            ("ro", "Romanian"),
            ("ru", "Русский"),
            ("sk", "Slovak"),
            ("sr", "Serbian"),
            ("sl", "Slovenian"),
            ("sv", "Swedish"),
            ("ta", "Tamil"),
            ("te", "Telugu"),
            ("th", "Thai"),
            ("tr", "Turkish"),
            ("uk", "Ukrainian"),
            ("ur", "Urdu"),
            ("vi", "Vietnamese"),
            ("mn", "Mongolian"),
            ("zh_HANS", "中文"),
            ("zh_HANT", "繁體中文"),
        ]
        .into_iter()
        .collect();

        let aliases: FxHashMap<_, _> = [
            ("en-GB", "en"),
            ("en-US", "en"),
            ("zh-CN", "zh_HANS"),
            ("zh-TW", "zh_HANT"),
        ]
        .into_iter()
        .collect();

        let mut translations = FxHashMap::default();
        languages.iter().for_each(|(code, _)| {
            translations.insert(*code, Arc::new(FxHashMap::default()));
        });

        Data {
            enabled: vec!["en"],
            aliases,
            languages,
            translations,
        }
    }
}

impl<'data> From<&Dictionary> for Data<'data> {
    fn from(dict: &Dictionary) -> Self {
        let Dictionary {
            languages,
            aliases,
            translations,
            enabled,
            ..
        } = dict;

        Data {
            enabled: enabled.clone(),
            aliases: aliases.clone(),
            languages: languages.clone(),
            translations: translations.clone(),
        }
    }
}
