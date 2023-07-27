//!
//! File system abstraction layer. Currently supporting storage on the filesystem
//! and the browser domain-associated local storage ([Web Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API)).
//!
//! Storage APIs abstracted:
//! - Std File I/O (fs::xxx)
//! - NodeJS File I/O (fs::read_file_sync)
//! - Local Storage
//!
//! By default, all I/O functions will use the name of the file as a key
//! for localstorage. If you want to manually specify the localstorage key,
//! you should use `*_with_localstorage()` suffixed functions.
//!

use crate::error::Error;
use crate::result::Result;
use cfg_if::cfg_if;
// use js_sys::Function;
#[allow(unused_imports)]
use js_sys::{Object, Reflect};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::{Path, PathBuf};
use wasm_bindgen::prelude::*;
use workflow_core::dirs;
use workflow_core::runtime;
// use workflow_wasm::object::ObjectTrait;
// use js_sys::Array;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn require(s: &str) -> JsValue;
}

#[wasm_bindgen(inline_js = r#"
if (!globalThis.require) {
    globalThis.require = () => { return {}; };
}
const fs = globalThis.require('fs'); 
const fs_promises = globalThis.require('fs/promises'); 
export { fs, fs_promises };
"#)]
extern "C" {

    // #[wasm_bindgen(extends = Object)]
    // #[derive(Debug, Clone, PartialEq, Eq)]
    // pub type ReadDir;

    #[wasm_bindgen(js_name = readdir, js_namespace = fs)]
    pub fn readdir_sync(path: &str, callback: js_sys::Function);

    #[wasm_bindgen(catch, js_name = readdir, js_namespace = fs_promises)]
    async fn fs_readdir(path: &str) -> Result<JsValue>;

    #[wasm_bindgen(catch, js_name = readdir, js_namespace = fs_promises)]
    async fn fs_readdir_with_options(path: &str, options: Object) -> Result<JsValue>;

    #[wasm_bindgen(catch, js_name = existsSync, js_namespace = fs)]
    fn fs_exists_sync(path: &str) -> Result<bool>;

    #[wasm_bindgen(catch, js_name = writeFileSync, js_namespace = fs)]
    fn fs_write_file_sync(path: &str, data: &str, options: Object) -> Result<()>;

    #[wasm_bindgen(catch, js_name = readFileSync, js_namespace = fs)]
    fn fs_read_file_sync(path: &str, options: Object) -> Result<JsValue>;

    #[wasm_bindgen(catch, js_name = mkdirSync, js_namespace = fs)]
    fn fs_mkdir_sync(path: &str, options: Object) -> Result<()>;

    #[wasm_bindgen(catch, js_name = unlinkSync, js_namespace = fs)]
    fn fs_unlink_sync(path: &str) -> Result<()>;

    #[wasm_bindgen(catch, js_name = statSync, js_namespace = fs)]
    fn fs_stat_sync(path: &str) -> Result<JsValue>;
}

pub fn local_storage() -> web_sys::Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

#[derive(Default)]
pub struct Options {
    pub local_storage_key: Option<String>,
}

impl Options {
    pub fn with_local_storage_key(key: &str) -> Self {
        Options {
            local_storage_key: Some(key.to_string()),
        }
    }

    pub fn local_storage_key(&self, filename: &Path) -> String {
        self.local_storage_key
            .clone()
            .unwrap_or(filename.file_name().unwrap().to_str().unwrap().to_string())
    }
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {

        pub async fn exists_with_options<P : AsRef<Path>>(filename: P, options : Options) -> Result<bool> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                Ok(fs_exists_sync(filename.as_ref())?)
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                Ok(local_storage().get_item(&key_name)?.is_some())
            }
        }

        pub async fn read_to_string_with_options<P : AsRef<Path>>(filename: P, options : Options) -> Result<String> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                Reflect::set(&options, &"encoding".into(), &"utf-8".into())?;
                // options.set("encoding", "utf-8");
                let js_value = fs_read_file_sync(&filename, options)?;
                let text = js_value.as_string().ok_or(Error::DataIsNotAString(filename))?;
                Ok(text)
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                if let Some(text) = local_storage().get_item(&key_name)? {
                    Ok(text)
                } else {
                    Err(Error::NotFound(filename.as_ref().to_string_lossy().to_string()))
                }
            }
        }

        pub async fn write_string_with_options<P : AsRef<Path>>(filename: P, options: Options, text : &str) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                fs_write_file_sync(&filename, text, options)?;
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                local_storage().set_item(&key_name, text)?;
            }
            Ok(())
        }

        pub async fn remove_with_options<P : AsRef<Path>>(filename: P, options: Options) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                // let options = Object::new();
                fs_unlink_sync(&filename)?;
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                local_storage().remove_item(&key_name)?;
            }
            Ok(())
        }

        pub async fn create_dir_all<P : AsRef<Path>>(filename: P) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let options = Object::new();
                Reflect::set(&options, &JsValue::from("recursive"), &JsValue::from_bool(true))?;
                let filename = filename.as_ref().to_platform_string();
                fs_mkdir_sync(&filename, options)?;
            }

            Ok(())
        }


        async fn fetch_metadata(path: &str, entries : &mut [DirEntry]) -> Result<()> {
            for entry in entries.iter_mut() {
                let path = format!("{}/{}",path, entry.file_name());
                let metadata = fs_stat_sync(&path).unwrap();
                entry.metadata = metadata.try_into().ok();
            }

            Ok(())
        }

        async fn readdir_impl(path: &Path, metadata : bool) -> std::result::Result<Vec<DirEntry>,String> {
            let path_string = path.to_string_lossy().to_string();
            let files = fs_readdir(&path_string).await.map_err(|e|e.to_string())?;
            let list = files.dyn_into::<js_sys::Array>().expect("readdir: expecting resulting entries to be an array");
            let mut entries = list.to_vec().into_iter().map(|s| s.into()).collect::<Vec<DirEntry>>();

            if metadata {
                fetch_metadata(&path_string, &mut entries).await.map_err(|e|e.to_string())?;
            }

            Ok(entries)
        }

        pub async fn readdir<P>(path: P, metadata : bool) -> Result<Vec<DirEntry>>
        where P : AsRef<Path> + Send + 'static
        {
            // this is a hack to bypass JsFuture being !Send
            // until I had a chance to setup a proper infrastructure
            // to relay JS promises within Send contexts.
            // we want to use async version of readdir to ensure
            // our executor is not blocked.

            use workflow_core::sendable::Sendable;
            use workflow_core::task::dispatch;
            use workflow_core::channel::oneshot;

            if runtime::is_node() || runtime::is_nw() {

                let (sender, receiver) = oneshot();
                dispatch(async move {
                    let path = path.as_ref();
                    let result = readdir_impl(path, metadata).await;
                    sender.send(Sendable(result)).await.unwrap();
                });

                Ok(receiver.recv().await.unwrap().unwrap()?)
            } else {
                panic!("readdir not supported on this platform")
            }
        }

        // -----------------------------------------

    } else {  // cfg_if - native platforms

        // -----------------------------------------

        pub async fn exists_with_options<P : AsRef<Path>>(filename: P, _options: Options) -> Result<bool> {
            Ok(filename.as_ref().exists())
        }

        pub async fn read_to_string_with_options<P : AsRef<Path>>(filename: P, _options: Options) -> Result<String> {
            Ok(std::fs::read_to_string(filename)?)
        }

        pub async fn write_string_with_options<P : AsRef<Path>>(filename: P, _options: Options, text : &str) -> Result<()> {
            Ok(std::fs::write(filename, text)?)
        }

        pub async fn remove_with_options<P : AsRef<Path>>(filename: P, _options: Options) -> Result<()> {
            std::fs::remove_file(filename)?;
            Ok(())
        }

        pub async fn create_dir_all<P : AsRef<Path>>(dir: P) -> Result<()> {
            std::fs::create_dir_all(dir)?;
            Ok(())
        }


        pub async fn readdir<P : AsRef<Path>>(path: P, metadata : bool) -> Result<Vec<DirEntry>> {
            let entries = std::fs::read_dir(path.as_ref())?;

            if metadata {
                let mut list = Vec::new();
                for de in entries {
                    let de = de?;
                    let metadata = std::fs::metadata(de.path())?;
                    let dir_entry = DirEntry::from((de,metadata));
                    list.push(dir_entry);
                }
                Ok(list)
            } else {
                Ok(entries.map(|r|r.map(|e|e.into())).collect::<std::result::Result<Vec<_>,_>>()?)
            }


        }

    }

}

#[derive(Clone, Debug)]
pub struct Metadata {
    created: u64,
    modified: u64,
    accessed: u64,
    len: u64,
}

impl Metadata {
    pub fn created(&self) -> u64 {
        self.created
    }

    pub fn modified(&self) -> u64 {
        self.modified
    }

    pub fn accessed(&self) -> u64 {
        self.accessed
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl From<std::fs::Metadata> for Metadata {
    fn from(metadata: std::fs::Metadata) -> Self {
        Metadata {
            created: metadata
                .created()
                .unwrap()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified: metadata
                .modified()
                .unwrap()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            accessed: metadata
                .accessed()
                .unwrap()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            len: metadata.len(),
        }
    }
}

impl TryFrom<JsValue> for Metadata {
    type Error = Error;
    fn try_from(metadata: JsValue) -> Result<Self> {
        if metadata.is_undefined() {
            return Err(Error::Metadata);
        }
        // let object = Object::from(metadata);
        workflow_log::log_info!("{:?}", metadata);
        let created = (Reflect::get(&metadata, &"birthtimeMs".into())
            .unwrap()
            .as_f64()
            .unwrap()
            / 1000.0) as u64;
        let modified = (Reflect::get(&metadata, &"mtimeMs".into())
            .unwrap()
            .as_f64()
            .unwrap()
            / 1000.0) as u64;
        let accessed = (Reflect::get(&metadata, &"atimeMs".into())
            .unwrap()
            .as_f64()
            .unwrap()
            / 1000.0) as u64;
        let size = Reflect::get(&metadata, &"size".into())
            .unwrap()
            .as_f64()
            .unwrap() as u64;

        Ok(Metadata {
            created,
            modified,
            accessed,
            len: size,
        })
    }
}

#[derive(Clone, Debug)]
pub struct DirEntry {
    file_name: String,
    metadata: Option<Metadata>,
}

impl DirEntry {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

impl From<std::fs::DirEntry> for DirEntry {
    fn from(de: std::fs::DirEntry) -> Self {
        DirEntry {
            file_name: de.file_name().to_string_lossy().to_string(),
            metadata: None,
        }
    }
}

impl From<(std::fs::DirEntry, std::fs::Metadata)> for DirEntry {
    fn from((de, metadata): (std::fs::DirEntry, std::fs::Metadata)) -> Self {
        DirEntry {
            file_name: de.file_name().to_string_lossy().to_string(),
            metadata: Some(metadata.into()),
        }
    }
}

impl From<JsValue> for DirEntry {
    fn from(de: JsValue) -> Self {
        DirEntry {
            file_name: de.as_string().unwrap(),
            metadata: None,
        }
    }
}

pub async fn exists<P: AsRef<Path>>(filename: P) -> Result<bool> {
    exists_with_options(filename, Options::default()).await
}

pub async fn read_to_string(filename: &Path) -> Result<String> {
    read_to_string_with_options(filename, Options::default()).await
}

pub async fn write_string(filename: &Path, text: &str) -> Result<()> {
    write_string_with_options(filename, Options::default(), text).await
}

pub async fn remove(filename: &Path) -> Result<()> {
    remove_with_options(filename, Options::default()).await
}

pub async fn read_json_with_options<T>(filename: &Path, options: Options) -> Result<T>
where
    T: DeserializeOwned,
{
    let text = read_to_string_with_options(filename, options).await?;
    Ok(serde_json::from_str(&text)?)
}

pub async fn write_json_with_options<T>(filename: &Path, options: Options, value: &T) -> Result<()>
where
    T: Serialize,
{
    let json = serde_json::to_string(value)?;
    write_string_with_options(filename, options, &json).await?;
    Ok(())
}

pub async fn read_json<T>(filename: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    read_json_with_options(filename, Options::default()).await
}

pub async fn write_json<T>(filename: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    write_json_with_options(filename, Options::default(), value).await
}

pub fn resolve_path(path: &str) -> Result<PathBuf> {
    if let Some(_stripped) = path.strip_prefix("~/") {
        if runtime::is_web() {
            Ok(PathBuf::from(path))
        } else if runtime::is_node() || runtime::is_nw() {
            Ok(dirs::home_dir()
                .ok_or_else(|| Error::HomeDir(path.to_string()))?
                .join(_stripped))
        } else {
            cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    Ok(PathBuf::from(path))
                } else {
                    Ok(home::home_dir().ok_or_else(||Error::HomeDir(path.to_string()))?.join(_stripped))
                }
            }
        }
    } else {
        Ok(PathBuf::from(path))
    }
}

/// Normalizes path, dereferencing relative references `.` and `..`
/// and converting path separators to current platform separators.
/// (detects platform nativly or via NodeJS if operating in WASM32
/// environment)
pub trait NormalizePath {
    fn normalize(&self) -> Result<PathBuf>;
}

impl NormalizePath for Path {
    fn normalize(&self) -> Result<PathBuf> {
        normalize(self)
    }
}

impl NormalizePath for PathBuf {
    fn normalize(&self) -> Result<PathBuf> {
        normalize(self)
    }
}

/// Convert path separators to unix or to current platform.
/// Detects platform natively or using NodeJS if operating
/// under WASM32 environment. Since in WASM32 paths default
/// to forward slashes, when running WASM32 in Windows paths
/// needs to be converted back and forth for various path-related
/// functions to work.
pub trait ToPlatform {
    fn to_platform(&self) -> PathBuf;
    fn to_platform_string(&self) -> String;
    fn to_unix(&self) -> PathBuf;
}

impl ToPlatform for Path {
    fn to_platform(&self) -> PathBuf {
        if runtime::is_windows() {
            convert_path_separators(self, "/", "\\")
        } else {
            self.to_path_buf()
        }
    }

    fn to_platform_string(&self) -> String {
        self.to_platform().to_string_lossy().to_string()
    }

    fn to_unix(&self) -> PathBuf {
        if runtime::is_windows() {
            convert_path_separators(self, "\\", "/")
        } else {
            self.to_path_buf()
        }
    }
}

/// Normalizes path, dereferencing relative references `.` and `..`
/// and converting path separators to current platform separators.
/// (detects platform nativly or via NodeJS if operating in WASM32
/// environment). Uses [`ToPlatform`] to perform path conversion.
pub fn normalize<P>(path: P) -> Result<PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().to_unix();
    let mut result = PathBuf::new();

    for component in path.components() {
        if let Some(c) = component.as_os_str().to_str() {
            if c == "." {
                continue;
            } else if c == ".." {
                result.pop();
            } else {
                result.push(c);
            }
        } else {
            return Err(Error::InvalidPath(path.to_string_lossy().to_string()));
        }
    }

    Ok(result.to_platform())
}

fn convert_path_separators<P>(path: P, from: &str, to: &str) -> PathBuf
where
    P: AsRef<Path>,
{
    let path = path.as_ref().to_string_lossy();
    let path = path.replace(from, to);
    PathBuf::from(path)
}
