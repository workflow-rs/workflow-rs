use crate::error::Error;
use crate::result::Result;
use arc_swap::*;
use ritehash::FxHasher64;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
pub type FxBuildHasher = BuildHasherDefault<FxHasher64>;
pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

static mut JSON_DATA: Option<String> = None;
static mut JSON_DATA_GUARD: Option<Mutex<()>> = None;
static DICTIONARY: ArcSwapOption<Dictionary> = ArcSwapOption::const_empty();

pub type StoreFn = dyn Send + Sync + Fn(&str) -> Result<()> + 'static;
pub type DictionaryArgs<'a> = Vec<(&'static str, &'a str)>;

pub struct Builder {
    current_code: String,
    default_code: String,
    static_json_data: Option<&'static str>,
    string_json_data: Option<String>,
    store_fn: Option<Arc<StoreFn>>,
}

impl Builder {
    pub fn new(current_code: &str, default_code: &str) -> Self {
        Builder {
            current_code: current_code.to_string(),
            default_code: default_code.to_string(),
            static_json_data: None,
            string_json_data: None,
            store_fn: None,
        }
    }

    pub fn with_static_json_data(mut self, json_data: &'static str) -> Self {
        self.static_json_data = Some(json_data);
        self
    }

    pub fn with_string_json_data(mut self, json_data: Option<String>) -> Self {
        self.string_json_data = json_data;
        self
    }

    pub fn with_store(
        mut self,
        store_fn: impl Fn(&str) -> Result<()> + Send + Sync + 'static,
    ) -> Self {
        self.store_fn = Some(Arc::new(store_fn));
        self
    }

    pub fn try_init(self) -> Result<()> {
        let json_data = if let Some(json_data) = self.string_json_data {
            unsafe {
                JSON_DATA = Some(json_data);
                JSON_DATA.as_deref()
            }
        } else {
            self.static_json_data
        };

        let dictionary = Arc::new(Dictionary::try_new(
            self.current_code,
            self.default_code,
            json_data,
            self.store_fn,
        )?);

        DICTIONARY.swap(Some(dictionary.clone()));

        Ok(())
    }
}

/// Create the default i18n data file at the supplied path.
pub fn create(i18n_file: impl Into<PathBuf>) -> Result<()> {
    let i18n_file = i18n_file.into();
    std::fs::write(i18n_file, serde_json::to_string_pretty(&Data::default())?)?;
    Ok(())
}

/// Merge all files ending with `_XX.json` (where `XX` is a language code) into a single file `i18n.json`
#[cfg(not(target_arch = "wasm32"))]
pub fn import_translation_files<P: AsRef<Path>>(source_folder_path: P, reload: bool) -> Result<()> {
    let source_folder_path = source_folder_path.as_ref();

    let dictionary = dictionary();

    let suffixes = dictionary
        .language_codes()
        .iter()
        .map(|code| (code.to_string(), format!("_{code}.json")))
        .collect::<Vec<_>>();

    let files = std::fs::read_dir(source_folder_path)?
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
                })
                .map(|translation| (code, translation))
        })
        .collect::<Result<FxHashMap<_, _>>>()?;

    let translations = dictionary
        .translations
        .clone()
        .into_iter()
        .chain(merged_translations.iter().map(|(code, translation)| {
            (
                code.as_str(),
                Arc::new(
                    translation
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect(),
                ),
            )
        }))
        .collect();

    let data = Data {
        enabled: dictionary.enabled.clone(),
        aliases: dictionary.aliases.clone(),
        languages: dictionary.languages.clone(),
        translations,
    };

    let json = serde_json::to_value(&data)?;
    let json_data = crate::json::to_json(&json);
    // let json_data = serde_json::to_string_pretty(&data)?;

    dictionary.store_fn().clone().unwrap()(json_data.as_str())?;

    if reload {
        from_string(json_data)?;
    }

    Ok(())
}

pub fn export_default_language(
    store_fn: impl Fn(&str) -> Result<()> + Send + Sync + 'static,
) -> Result<()> {
    let dictionary = dictionary();
    let source = dictionary.default_translations();
    let merged = source
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .chain(
            dictionary
                .missing
                .lock()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        )
        .collect::<FxHashMap<String, String>>();

    let json = serde_json::to_value(merged)?;
    let json_data = crate::json::to_json(&json);
    store_fn(json_data.as_str())
    // store_fn(serde_json::to_string_pretty(&merged)?.as_str())
}

#[inline(always)]
pub fn dictionary() -> Arc<Dictionary> {
    DICTIONARY.load().as_ref().unwrap().clone()
}

pub fn guard() -> MutexGuard<'static, ()> {
    unsafe { JSON_DATA_GUARD.as_ref().unwrap().lock().unwrap() }
}

pub fn load(json_data_file: impl AsRef<Path>) -> Result<()> {
    from_string(std::fs::read_to_string(json_data_file)?)?;
    Ok(())
}

pub fn from_string(json_data: impl Into<String>) -> Result<()> {
    let _guard = guard();

    let (current_code, default_code, store_fn) = {
        let dictionary = dictionary();
        (
            dictionary.current_code().to_string(),
            dictionary.default_code().to_string(),
            dictionary.store_fn().clone(),
        )
    };

    unsafe {
        JSON_DATA = Some(json_data.into());
        DICTIONARY.swap(Some(Arc::new(Dictionary::try_new(
            current_code,
            default_code,
            Some(JSON_DATA.as_ref().unwrap().as_str()),
            store_fn,
        )?)));
    }

    Ok(())
}

/// Translate a string to the currently user-selected language.
pub fn i18n(text: &str) -> &str {
    #[cfg(feature = "thread-safe")]
    let _guard = guard();

    let dictionary = dictionary();

    match dictionary.translate(text) {
        Some(translated) => translated,
        None => {
            let needs_store = {
                let mut missing = dictionary.missing.lock().unwrap();
                if !missing.contains_key(text) {
                    missing.insert(text.to_string(), text.to_string());
                    true
                } else {
                    false
                }
            };

            if needs_store {
                if let Some(store_fn) = dictionary.store_fn() {
                    match dictionary.to_json() {
                        Ok(json_data) => {
                            if let Err(err) = store_fn(json_data.as_str()) {
                                println!("i18n error: {}", err);
                            }
                        }
                        Err(err) => {
                            println!("i18n error: {}", err);
                        }
                    }
                }
            }

            text
        }
    }
}

// a tiny accelerator to save a heap alloc instead of using a String
#[inline(always)]
fn make_arg<K>(buffer: &mut [u8], key: K) -> &str
where
    K: AsRef<str>,
{
    let key = key.as_ref();
    let mut written = 0;

    buffer[written] = b'{';
    written += 1;

    buffer[written..written + key.len()].copy_from_slice(key.as_bytes());
    written += key.len();

    buffer[written] = b'}';
    written += 1;

    std::str::from_utf8(&buffer[..written]).unwrap()
}

/// Translate a string to the currently user-selected language
/// and replace given placeholders with given values.
/// Parameter 'replacements' is a vector consisting of key value pairs,
/// where the key is the placeholder within 'text'.
pub fn i18n_args<'a, K, V>(text: &str, replacements: impl IntoIterator<Item = &'a (K, V)>) -> String
where
    K: AsRef<str> + 'a,
    V: AsRef<str> + 'a,
{
    let mut buffer = [0u8; 64];
    let mut translated = String::from(i18n(text));

    for (key, value) in replacements {
        let placeholder = make_arg(&mut buffer, key);
        translated = str::replace(&translated, placeholder, value.as_ref());
    }

    translated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_i18n_args() {
        Builder::new("en", "en").try_init().unwrap();

        let name = "John";
        let text = "Hello, {name}!";
        let translated = i18n_args(text, &[("name", name)]);
        // println!("{translated}");
        assert_eq!(translated, "Hello, John!");
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
    missing: Mutex<FxHashMap<String, String>>,
    /// Enabled language codes ["en", "ja"]
    enabled: Vec<&'static str>,
    /// Current language code
    current_code: ArcSwap<String>,
    /// Current language title
    current_title: ArcSwap<String>,
    /// Current language translations {"Hello" : "こんにちは", ...}
    current_translations: ArcSwap<FxHashMap<&'static str, &'static str>>,
    /// Default language code (the language used in the source code)
    default_code: String,
    /// Default language translations {"Hello" : "Hello", ...}
    default_translations: Arc<FxHashMap<&'static str, &'static str>>,
    // / Full data file path
    // json_data_file_path: Option<PathBuf>,
    /// Storage callback function
    store_fn: Option<Arc<StoreFn>>,
}

impl Dictionary {
    /// Create a new dictionary from JSON data. JSON data must be `&'static str`, i.e. loaded by the application via the `include_str!()` macro.
    fn try_new(
        current_code: impl Into<String>,
        default_code: impl Into<String>,
        json_data: Option<&'static str>,
        // json_data_file_path: Option<&Path>,
        store_fn: Option<Arc<StoreFn>>,
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
            missing: Mutex::new(FxHashMap::default()),
            enabled,
            current_code: ArcSwap::new(Arc::new(current_code)),
            current_title: ArcSwap::new(Arc::new(current_title)),
            current_translations: ArcSwap::new(current_translations),
            default_code,
            default_translations,
            store_fn,
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

    pub fn store_fn(&self) -> &Option<Arc<StoreFn>> {
        &self.store_fn
    }

    #[inline(always)]
    pub fn current_code(&self) -> Arc<String> {
        self.current_code.load().clone()
    }

    #[inline(always)]
    pub fn current_title(&self) -> Arc<String> {
        self.current_title.load().clone()
    }

    #[inline(always)]
    pub fn default_code(&self) -> &str {
        &self.default_code
    }

    #[inline(always)]
    pub fn current_translations(&self) -> Arc<FxHashMap<&'static str, &'static str>> {
        self.current_translations.load().clone()
    }

    #[inline(always)]
    pub fn translate(&self, text: &str) -> Option<&'static str> {
        let current_translations = self.current_translations.load().clone();
        current_translations.get(text).copied()
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

    pub fn activate_language_code(&self, language_code: impl Into<String>) -> Result<()> {
        let language_code: String = language_code.into();
        let current_code = self.resolve_aliases(language_code.as_str())?;
        let current_title = self.language_title(current_code.as_str())?.to_string();
        let current_translations = dictionary()
            .translations
            .get(language_code.as_str())
            .ok_or(Error::UnknownLanguageCode(language_code))?
            .clone();

        self.current_code.store(Arc::new(current_code));
        self.current_title.store(Arc::new(current_title));
        self.current_translations.store(current_translations);

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

    pub fn to_json(&self) -> Result<String> {
        let data = Storable::from(self);
        let json = serde_json::to_value(data)?;
        let json_data = crate::json::to_json(&json);
        Ok(json_data)
        // Ok(serde_json::to_string_pretty(&data)?)
    }
}

pub struct Languages(FxHashMap<&'static str, &'static str>);

impl Default for Languages {
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
            ("zh", "中文"),
        ]
        .into_iter()
        .collect();

        Languages(languages)
    }
}

impl Languages {
    pub fn new() -> Self {
        Languages(FxHashMap::default())
    }

    pub fn add(&mut self, code: &'static str, name: &'static str) {
        self.0.insert(code, name);
    }

    pub fn into_inner(self) -> FxHashMap<&'static str, &'static str> {
        self.0
    }

    pub fn codes(&self) -> Vec<&'static str> {
        self.0.keys().copied().collect()
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

impl Default for Data<'_> {
    fn default() -> Self {
        let languages = Languages::default().into_inner();

        let aliases: FxHashMap<_, _> = [
            ("en-GB", "en"),
            ("en-US", "en"),
            ("zh-CN", "zh"),
            ("zh-TW", "zh"),
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

#[derive(Debug, Clone, serde::Serialize)]
pub struct Storable {
    enabled: Vec<&'static str>,
    aliases: FxHashMap<&'static str, &'static str>,
    languages: FxHashMap<&'static str, &'static str>,
    translations: FxHashMap<&'static str, FxHashMap<String, String>>,
}

impl From<&Dictionary> for Storable {
    fn from(dict: &Dictionary) -> Self {
        let Dictionary {
            languages,
            aliases,
            translations,
            missing,
            default_code,
            enabled,
            ..
        } = dict;

        let mut translations: FxHashMap<&'static str, FxHashMap<String, String>> = translations
            .iter()
            .map(|(code, language_translation)| {
                let language_translation: FxHashMap<String, String> = language_translation
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();
                (*code, language_translation)
            })
            .collect();

        let default_translations = translations.get_mut(default_code.as_str()).unwrap();
        default_translations.extend(
            missing
                .lock()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );

        Storable {
            enabled: enabled.clone(),
            aliases: aliases.clone(),
            languages: languages.clone(),
            translations: translations.clone(),
        }
    }
}
