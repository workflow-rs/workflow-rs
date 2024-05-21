//!
//! File system abstraction layer. Currently supporting storage on the filesystem
//! and the browser domain-associated local storage ([Web Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API)).
//!
//! Storage APIs abstracted:
//! - Rust std file I/O (fs::xxx)
//! - NodeJS file I/O (fs::read_file_sync)
//! - Browser local storage
//!
//! By default, all I/O functions will use the name of the file as a key
//! for localstorage. If you want to manually specify the localstorage key.
//!

use crate::error::Error;
use crate::result::Result;
use cfg_if::cfg_if;
use js_sys::Reflect;
use js_sys::Uint8Array;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::{Path, PathBuf};
use wasm_bindgen::prelude::*;
use workflow_core::dirs;
use workflow_core::runtime;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Uint8Array)]
    #[derive(Clone, Debug)]
    pub type Buffer;

    #[wasm_bindgen(static_method_of = Buffer, js_name = from)]
    pub fn from_uint8_array(array: &Uint8Array) -> Buffer;

}

pub fn local_storage() -> web_sys::Storage {
    web_sys::window().unwrap().local_storage().ok().flatten().expect("localStorage is not available")
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
        use workflow_core::hex::*;
        use workflow_wasm::jserror::*;
        use workflow_node as node;
        use js_sys::Object;
        use workflow_chrome::storage::LocalStorage as ChromeStorage;


        pub async fn exists_with_options<P : AsRef<Path>>(filename: P, options : Options) -> Result<bool> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                Ok(node::fs::exists_sync(filename.as_ref())?)
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                if runtime::is_chrome_extension(){
                    Ok(ChromeStorage::get_item(&key_name).await?.is_some())
                }else{
                    Ok(local_storage().get_item(&key_name)?.is_some())
                }
            }
        }

        pub fn exists_with_options_sync<P : AsRef<Path>>(filename: P, options : Options) -> Result<bool> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                Ok(node::fs::exists_sync(filename.as_ref())?)
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                if runtime::is_chrome_extension(){
                    Err(Error::Custom("localStorage api is unavailable, you can use exists_with_options() for chrome.storage.local api.".to_string()))
                }else{
                    Ok(local_storage().get_item(&key_name)?.is_some())
                }
            }
        }

        pub async fn read_to_string_with_options<P : AsRef<Path>>(filename: P, options : Options) -> Result<String> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                Reflect::set(&options, &"encoding".into(), &"utf-8".into())?;
                let js_value = node::fs::read_file_sync(&filename, options)?;
                let text = js_value.as_string().ok_or(Error::DataIsNotAString(filename))?;
                Ok(text)
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                if runtime::is_chrome_extension(){
                    if let Some(text) = ChromeStorage::get_item(&key_name).await?{
                        Ok(text)
                    }else {
                        Err(Error::NotFound(filename.as_ref().to_string_lossy().to_string()))
                    }
                }else if let Some(text) = local_storage().get_item(&key_name)? {
                    Ok(text)
                } else {
                    Err(Error::NotFound(filename.as_ref().to_string_lossy().to_string()))
                }
            }
        }

        pub fn read_to_string_with_options_sync<P : AsRef<Path>>(filename: P, options : Options) -> Result<String> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                Reflect::set(&options, &"encoding".into(), &"utf-8".into())?;
                let js_value = node::fs::read_file_sync(&filename, options)?;
                let text = js_value.as_string().ok_or(Error::DataIsNotAString(filename))?;
                Ok(text)
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                if runtime::is_chrome_extension(){
                    Err(Error::Custom("localStorage api is unavailable, you can use exists_with_options() for chrome.storage.local api.".to_string()))
                }else if let Some(text) = local_storage().get_item(&key_name)? {
                    Ok(text)
                } else {
                    Err(Error::NotFound(filename.as_ref().to_string_lossy().to_string()))
                }
            }
        }

        pub async fn read_binary_with_options<P : AsRef<Path>>(filename: P, options : Options) -> Result<Vec<u8>> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                let buffer = node::fs::read_file_sync(&filename, options)?;
                let data = buffer.dyn_into::<Uint8Array>()?;
                Ok(data.to_vec())
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                let data = if runtime::is_chrome_extension(){
                    ChromeStorage::get_item(&key_name).await?
                }else{
                    local_storage().get_item(&key_name)?
                };

                if let Some(text) = data{
                    let data = Vec::<u8>::from_hex(&text)?;
                    Ok(data)
                } else {
                    Err(Error::NotFound(filename.as_ref().to_string_lossy().to_string()))
                }
            }
        }

        pub fn read_binary_with_options_sync<P : AsRef<Path>>(filename: P, options : Options) -> Result<Vec<u8>> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                let buffer = node::fs::read_file_sync(&filename, options)?;
                let data = buffer.dyn_into::<Uint8Array>()?;
                Ok(data.to_vec())
            } else if runtime::is_chrome_extension(){
                    Err(Error::Custom("localStorage api is unavailable, you can use read_binary_with_options() for chrome.storage.local api.".to_string()))
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                if let Some(text) = local_storage().get_item(&key_name)? {
                    let data = Vec::<u8>::from_hex(&text)?;
                    Ok(data)
                } else {
                    Err(Error::NotFound(filename.as_ref().to_string_lossy().to_string()))
                }
            }
        }

        pub async fn write_string_with_options<P : AsRef<Path>>(filename: P, options: Options, text : &str) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                Reflect::set(&options, &"encoding".into(), &"utf-8".into())?;
                let data = JsValue::from(text);
                node::fs::write_file_sync(&filename, data, options)?;
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                if runtime::is_chrome_extension(){
                    ChromeStorage::set_item(&key_name, text).await?;
                }else{
                    local_storage().set_item(&key_name, text)?;
                }
            }

            Ok(())
        }

        pub fn write_string_with_options_sync<P : AsRef<Path>>(filename: P, options: Options, text : &str) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                Reflect::set(&options, &"encoding".into(), &"utf-8".into())?;
                let data = JsValue::from(text);
                node::fs::write_file_sync(&filename, data, options)?;
            } else if runtime::is_chrome_extension(){
                return Err(Error::Custom("localStorage api is unavailable, you can use write_string_with_options() for chrome.storage.local api.".to_string()));
            }else{
                let key_name = options.local_storage_key(filename.as_ref());
                local_storage().set_item(&key_name, text)?;
            }
            Ok(())
        }

        pub async fn write_binary_with_options<P : AsRef<Path>>(filename: P, options: Options, data : &[u8]) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                let uint8_array = Uint8Array::from(data);
                let buffer = Buffer::from_uint8_array(&uint8_array);
                node::fs::write_file_sync(&filename, buffer.into(), options)?;
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                if runtime::is_chrome_extension(){
                    ChromeStorage::set_item(&key_name, data.to_hex().as_str()).await?;
                }else{
                    local_storage().set_item(&key_name, data.to_hex().as_str())?;
                }
            }
            Ok(())
        }

        pub fn write_binary_with_options_sync<P : AsRef<Path>>(filename: P, options: Options, data : &[u8]) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                let options = Object::new();
                let uint8_array = Uint8Array::from(data);
                let buffer = Buffer::from_uint8_array(&uint8_array);
                node::fs::write_file_sync(&filename, buffer.into(), options)?;
            } else if runtime::is_chrome_extension(){
                return Err(Error::Custom("localStorage api is unavailable, you can use write_binary_with_options() for chrome.storage.local api.".to_string()));
            }else{
                let key_name = options.local_storage_key(filename.as_ref());
                local_storage().set_item(&key_name, data.to_hex().as_str())?;
            }

            Ok(())
        }

        pub async fn remove_with_options<P : AsRef<Path>>(filename: P, options: Options) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                node::fs::unlink_sync(&filename)?;
            } else {
                let key_name = options.local_storage_key(filename.as_ref());
                if runtime::is_chrome_extension(){
                    ChromeStorage::remove_item(&key_name).await?;
                }else{
                    local_storage().remove_item(&key_name)?;
                }
            }
            Ok(())
        }

        pub fn remove_with_options_sync<P : AsRef<Path>>(filename: P, options: Options) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let filename = filename.as_ref().to_platform_string();
                node::fs::unlink_sync(&filename)?;
            } else if runtime::is_chrome_extension(){
                return Err(Error::Custom("localStorage api is unavailable, you can use remove_with_options() for chrome.storage.local api.".to_string()));
            }else{
                let key_name = options.local_storage_key(filename.as_ref());
                local_storage().remove_item(&key_name)?;
            }
            Ok(())
        }

        pub async fn rename<P : AsRef<Path>>(from: P, to: P) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let from = from.as_ref().to_platform_string();
                let to = to.as_ref().to_platform_string();
                node::fs::rename_sync(&from,&to)?;
                Ok(())
            } else {
                Err(Error::NotSupported)
            }
        }

        pub fn rename_sync<P : AsRef<Path>>(from: P, to: P) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let from = from.as_ref().to_platform_string();
                let to = to.as_ref().to_platform_string();
                node::fs::rename_sync(&from,&to)?;
                Ok(())
            } else {
                Err(Error::NotSupported)
            }
        }

        pub async fn create_dir_all<P : AsRef<Path>>(filename: P) -> Result<()> {
            create_dir_all_sync(filename)
        }

        pub fn create_dir_all_sync<P : AsRef<Path>>(filename: P) -> Result<()> {
            if runtime::is_node() || runtime::is_nw() {
                let options = Object::new();
                Reflect::set(&options, &JsValue::from("recursive"), &JsValue::from_bool(true))?;
                let filename = filename.as_ref().to_platform_string();
                node::fs::mkdir_sync(&filename, options)?;
            }

            Ok(())
        }


        async fn fetch_metadata(path: &str, entries : &mut [DirEntry]) -> std::result::Result<(),JsErrorData> {
            for entry in entries.iter_mut() {
                let path = format!("{}/{}",path, entry.file_name());
                let metadata = node::fs::stat_sync(&path).unwrap();
                entry.metadata = metadata.try_into().ok();
            }

            Ok(())
        }

        async fn readdir_impl(path: &Path, metadata : bool) -> std::result::Result<Vec<DirEntry>,JsErrorData> {
            let path_string = path.to_string_lossy().to_string();
            let files = node::fs::readdir(&path_string).await?;
            let list = files.dyn_into::<js_sys::Array>().expect("readdir: expecting resulting entries to be an array");
            let mut entries = list.to_vec().into_iter().map(|s| s.into()).collect::<Vec<DirEntry>>();

            if metadata {
                fetch_metadata(&path_string, &mut entries).await?; //.map_err(|e|e.to_string())?;
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
            } else if runtime::is_chrome_extension(){
                let entries = ChromeStorage::keys().await?
                    .into_iter()
                    .map(DirEntry::from)
                    .collect::<Vec<_>>();
                Ok(entries)
            } else{
                let local_storage = local_storage();

                let mut entries = vec![];
                let length = local_storage.length().unwrap();
                for i in 0..length {
                    let key = local_storage.key(i)?;
                    if let Some(key) = key {
                        entries.push(DirEntry::from(key));
                    }
                }
                Ok(entries)
            }
        }

        // -----------------------------------------

    } else {  // cfg_if - native platforms

        // -----------------------------------------

        pub async fn exists_with_options<P : AsRef<Path>>(filename: P, _options: Options) -> Result<bool> {
            Ok(filename.as_ref().exists())
        }

        pub fn exists_with_options_sync<P : AsRef<Path>>(filename: P, _options: Options) -> Result<bool> {
            Ok(filename.as_ref().exists())
        }

        pub async fn read_to_string_with_options<P : AsRef<Path>>(filename: P, _options: Options) -> Result<String> {
            Ok(std::fs::read_to_string(filename)?)
        }

        pub fn read_to_string_with_options_sync<P : AsRef<Path>>(filename: P, _options: Options) -> Result<String> {
            Ok(std::fs::read_to_string(filename)?)
        }

        pub async fn read_binary_with_options<P : AsRef<Path>>(filename: P, _options: Options) -> Result<Vec<u8>> {
            Ok(std::fs::read(filename)?)
        }

        pub fn read_binary_with_options_sync<P : AsRef<Path>>(filename: P, _options: Options) -> Result<Vec<u8>> {
            Ok(std::fs::read(filename)?)
        }

        pub async fn write_string_with_options<P : AsRef<Path>>(filename: P, _options: Options, text : &str) -> Result<()> {
            Ok(std::fs::write(filename, text)?)
        }

        pub fn write_string_with_options_sync<P : AsRef<Path>>(filename: P, _options: Options, text : &str) -> Result<()> {
            Ok(std::fs::write(filename, text)?)
        }

        pub async fn write_binary_with_options<P : AsRef<Path>>(filename: P, _options: Options, data : &[u8]) -> Result<()> {
            Ok(std::fs::write(filename, data)?)
        }

        pub fn write_binary_with_options_sync<P : AsRef<Path>>(filename: P, _options: Options, data : &[u8]) -> Result<()> {
            Ok(std::fs::write(filename, data)?)
        }

        pub async fn remove_with_options<P : AsRef<Path>>(filename: P, _options: Options) -> Result<()> {
            std::fs::remove_file(filename)?;
            Ok(())
        }

        pub fn remove_with_options_sync<P : AsRef<Path>>(filename: P, _options: Options) -> Result<()> {
            std::fs::remove_file(filename)?;
            Ok(())
        }

        pub async fn rename<P : AsRef<Path>>(from: P, to: P) -> Result<()> {
            std::fs::rename(from,to)?;
            Ok(())
        }

        pub fn rename_sync<P : AsRef<Path>>(from: P, to: P) -> Result<()> {
            std::fs::rename(from,to)?;
            Ok(())
        }

        pub async fn create_dir_all<P : AsRef<Path>>(dir: P) -> Result<()> {
            std::fs::create_dir_all(dir)?;
            Ok(())
        }

        pub fn create_dir_all_sync<P : AsRef<Path>>(dir: P) -> Result<()> {
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

impl From<String> for DirEntry {
    fn from(s: String) -> Self {
        DirEntry {
            file_name: s,
            metadata: None,
        }
    }
}

/// Check if a file exists
pub async fn exists<P: AsRef<Path>>(filename: P) -> Result<bool> {
    exists_with_options(filename, Options::default()).await
}

/// Check if a file exists
pub fn exists_sync<P: AsRef<Path>>(filename: P) -> Result<bool> {
    exists_with_options_sync(filename, Options::default())
}

/// Read file contents to a string. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be used.
pub async fn read_to_string(filename: &Path) -> Result<String> {
    read_to_string_with_options(filename, Options::default()).await
}

/// Read file contents to a string. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be used.
pub fn read_to_string_sync(filename: &Path) -> Result<String> {
    read_to_string_with_options_sync(filename, Options::default())
}

/// Read binary file contents to a `Vec<u8>`. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be used and the data is assumed to be hex-encoded.
pub async fn read(filename: &Path) -> Result<Vec<u8>> {
    read_binary_with_options(filename, Options::default()).await
}

/// Read binary file contents to a `Vec<u8>`. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be used and the data is assumed to be hex-encoded.
pub fn read_sync(filename: &Path) -> Result<Vec<u8>> {
    read_binary_with_options_sync(filename, Options::default())
}

/// Write a string to a text file. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be used.
pub async fn write_string(filename: &Path, text: &str) -> Result<()> {
    write_string_with_options(filename, Options::default(), text).await
}

/// Write a string to a text file. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be used.
pub fn write_string_sync(filename: &Path, text: &str) -> Result<()> {
    write_string_with_options_sync(filename, Options::default(), text)
}

/// Write a `Vec<u8>` to a binary file. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be used and the data will be hex-encoded.
pub async fn write(filename: &Path, data: &[u8]) -> Result<()> {
    write_binary_with_options(filename, Options::default(), data).await
}

/// Write a `Vec<u8>` to a binary file. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be used and the data will be hex-encoded.
pub async fn write_sync(filename: &Path, data: &[u8]) -> Result<()> {
    write_binary_with_options_sync(filename, Options::default(), data)
}

/// Remove the file at the given path. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be removed.
pub async fn remove(filename: &Path) -> Result<()> {
    remove_with_options(filename, Options::default()).await
}

/// Remove the file at the given path. If using within the web browser
/// environment, a local storage key with the name of the file
/// will be removed.
pub fn remove_sync(filename: &Path) -> Result<()> {
    remove_with_options_sync(filename, Options::default())
}

/// Read text file and deserialized it using `serde-json`.
pub async fn read_json_with_options<T>(filename: &Path, options: Options) -> Result<T>
where
    T: DeserializeOwned,
{
    let text = read_to_string_with_options(filename, options).await?;
    Ok(serde_json::from_str(&text)?)
}

/// Read text file and deserialized it using `serde-json`.
pub fn read_json_with_options_sync<T>(filename: &Path, options: Options) -> Result<T>
where
    T: DeserializeOwned,
{
    let text = read_to_string_with_options_sync(filename, options)?;
    Ok(serde_json::from_str(&text)?)
}

/// Write a serializable value to a text file using `serde-json`.
pub async fn write_json_with_options<T>(filename: &Path, options: Options, value: &T) -> Result<()>
where
    T: Serialize,
{
    let json = serde_json::to_string(value)?;
    write_string_with_options(filename, options, &json).await?;
    Ok(())
}

/// Write a serializable value to a text file using `serde-json`.
pub fn write_json_with_options_sync<T>(filename: &Path, options: Options, value: &T) -> Result<()>
where
    T: Serialize,
{
    let json = serde_json::to_string(value)?;
    write_string_with_options_sync(filename, options, &json)?;
    Ok(())
}

/// Read text file and deserialized it using `serde-json`.
pub async fn read_json<T>(filename: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    read_json_with_options(filename, Options::default()).await
}

/// Read text file and deserialized it using `serde-json`.
pub fn read_json_sync<T>(filename: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    read_json_with_options_sync(filename, Options::default())
}

/// Write a serializable value to a text file using `serde-json`.
pub async fn write_json<T>(filename: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    write_json_with_options(filename, Options::default(), value).await
}

/// Write a serializable value to a text file using `serde-json`.
pub fn write_json_sync<T>(filename: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    write_json_with_options_sync(filename, Options::default(), value)
}

/// Parses the supplied path resolving `~/` to the home directory.
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
/// (detects platform natively or via NodeJS if operating in WASM32
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
/// (detects platform natively or via NodeJS if operating in WASM32
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
