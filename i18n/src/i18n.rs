use ritehash::FxHasher64;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::sync::{Arc, Mutex};
pub type FxBuildHasher = BuildHasherDefault<FxHasher64>;
pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("i18n received invalid language code {0}")]
    LanguageCode(String),
}
type Result<T> = std::result::Result<T, Error>;

static mut DICT: Option<Arc<Dict>> = None;
static mut LANG: Option<String> = None;

pub fn init_i18n<S: Into<String>>(lang_code: S) -> Result<()> {
    let dict = Arc::new(Dict::default());
    let lang_code: String = lang_code.into();
    let code = dict.transpose(lang_code.as_str())?;
    unsafe {
        DICT = Some(dict.clone());
    }
    unsafe {
        LANG = Some(code.into());
    }
    Ok(())
}

pub fn lang() -> &'static str {
    unsafe {
        LANG.as_ref()
            .expect("i18n language code is not initialized")
    }
}

pub fn dict() -> Arc<Dict> {
    unsafe {
        DICT.as_ref()
            .expect("i18n dictionary is not initialized")
            .clone()
    }
}

pub fn load() -> Result<()> {
    Ok(())
}

pub fn store() -> Result<()> {
    Ok(())
}

pub fn i18n(text: &str) -> String {
    let dict = dict();
    let mut nodes = dict.nodes.lock().expect("i18n nodes lock failure");
    let node = nodes.get(text);

    match node {
        Some(node) => {
            let translation = node.get(lang());
            match translation {
                Some(translation) => translation.clone(),
                None => String::from(text),
            }
        }
        None => {
            let mut node = FxHashMap::default();
            node.insert("en".to_string(), text.to_string());
            nodes.insert(text.to_string(), node);

            String::from(text)
        }
    }
}

pub struct Dict {
    languages: FxHashMap<&'static str, &'static str>,
    aliases: FxHashMap<&'static str, &'static str>,
    nodes: Mutex<FxHashMap<String, FxHashMap<String, String>>>,
}

impl Dict {
    fn transpose<'code>(&self, code: &'code str) -> Result<&'code str> {
        match self.aliases.get(code) {
            Some(code) => Ok(code),
            None => {
                if self.languages.contains_key(&code) {
                    Ok(code)
                } else {
                    Err(Error::LanguageCode(code.into()))
                }
            }
        }
    }
    pub fn language<'code>(&self, code: &'code str) -> Result<Option<&'code str>> {
        if let Some(lang) = self.languages.get(code) {
            Ok(Some(*lang))
        } else {
            Ok(None)
        }
    }
}

impl Default for Dict {
    fn default() -> Self {
        let nodes = Mutex::new(FxHashMap::default());
        let mut languages = FxHashMap::default();
        let mut aliases = FxHashMap::default();

        [
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
        .iter()
        .for_each(|(code, name)| {
            languages.insert(*code, *name);
        });

        [
            ("en-GB", "en"),
            ("en-US", "en"),
            ("zh-CN", "zh_HANS"),
            ("zh-TW", "zh_HANT"),
        ]
        .iter()
        .for_each(|(code, alias)| {
            aliases.insert(*alias, *code);
        });

        Dict {
            languages,
            aliases,
            nodes,
        }
    }
}
