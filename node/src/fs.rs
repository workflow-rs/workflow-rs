use crate::require;
use js_sys::Object;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref FS: Fs = require("fs").unchecked_into();
    static ref FSP: FsPromises = require("fs/promises").unchecked_into();
}

#[wasm_bindgen]
extern "C" {

    /// Binding to the Node.js `fs/promises` module, exposing the promise-based
    /// (asynchronous) file system API.
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type FsPromises;

    #[wasm_bindgen(catch, js_name = readdir, method)]
    async fn fs_readdir(this: &FsPromises, path: &str) -> std::result::Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = readdir, method)]
    async fn fs_readdir_with_options(
        this: &FsPromises,
        path: &str,
        options: Object,
    ) -> std::result::Result<JsValue, JsValue>;

    /// Binding to the Node.js `fs` module, exposing the synchronous
    /// (`*Sync`) file system API.
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type Fs;

    /// Reads the contents of a directory, passing the resulting entries to the
    /// supplied JavaScript callback (`fs.readdirSync`).
    #[wasm_bindgen(js_name = readdirSync, method)]
    pub fn fs_readdir_sync(this: &Fs, path: &str, callback: js_sys::Function);

    #[wasm_bindgen(catch, js_name = existsSync, method)]
    fn fs_exists_sync(this: &Fs, path: &str) -> std::result::Result<bool, JsValue>;

    #[wasm_bindgen(catch, js_name = writeFileSync, method)]
    fn fs_write_file_sync(
        this: &Fs,
        path: &str,
        data: JsValue,
        options: Object,
    ) -> std::result::Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = readFileSync, method)]
    fn fs_read_file_sync(
        this: &Fs,
        path: &str,
        options: Object,
    ) -> std::result::Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = mkdirSync, method)]
    fn fs_mkdir_sync(this: &Fs, path: &str, options: Object) -> std::result::Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = renameSync, method)]
    fn fs_rename_sync(this: &Fs, from: &str, to: &str) -> std::result::Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = unlinkSync, method)]
    fn fs_unlink_sync(this: &Fs, path: &str) -> std::result::Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = statSync, method)]
    fn fs_stat_sync(this: &Fs, path: &str) -> std::result::Result<JsValue, JsValue>;
}

unsafe impl Send for Fs {}
unsafe impl Sync for Fs {}
unsafe impl Send for FsPromises {}
unsafe impl Sync for FsPromises {}

/// Asynchronously reads the contents of the directory at `path`, returning an
/// array of the names of the entries it contains.
#[inline(always)]
pub async fn readdir(path: &str) -> std::result::Result<JsValue, JsValue> {
    FSP.fs_readdir(path).await
}

/// Asynchronously reads the contents of the directory at `path`, applying the
/// given options (e.g. `withFileTypes`, `recursive`, `encoding`).
#[inline(always)]
pub async fn readdir_with_options(
    path: &str,
    options: Object,
) -> std::result::Result<JsValue, JsValue> {
    FSP.fs_readdir_with_options(path, options).await
}

/// Reads the contents of the directory at `path`, delivering the resulting
/// entries to the supplied JavaScript `callback`.
#[inline(always)]
pub fn readdir_sync(path: &str, callback: js_sys::Function) {
    FS.fs_readdir_sync(path, callback)
}

/// Synchronously returns `true` if a file system entry exists at `path`.
#[inline(always)]
pub fn exists_sync(path: &str) -> std::result::Result<bool, JsValue> {
    FS.fs_exists_sync(path)
}

/// Synchronously writes `data` to the file at `path`, creating or replacing it
/// according to the supplied options (e.g. `encoding`, `mode`, `flag`).
#[inline(always)]
pub fn write_file_sync(
    path: &str,
    data: JsValue,
    options: Object,
) -> std::result::Result<(), JsValue> {
    FS.fs_write_file_sync(path, data, options)
}

/// Synchronously reads and returns the contents of the file at `path`. The
/// returned value is a `Buffer`, or a string when an `encoding` is given in
/// `options`.
#[inline(always)]
pub fn read_file_sync(path: &str, options: Object) -> std::result::Result<JsValue, JsValue> {
    FS.fs_read_file_sync(path, options)
}

/// Synchronously creates the directory at `path`, honouring the supplied
/// options (e.g. `recursive`, `mode`).
#[inline(always)]
pub fn mkdir_sync(path: &str, options: Object) -> std::result::Result<(), JsValue> {
    FS.fs_mkdir_sync(path, options)
}

/// Synchronously removes the file (or symbolic link) at `path`.
#[inline(always)]
pub fn unlink_sync(path: &str) -> std::result::Result<(), JsValue> {
    FS.fs_unlink_sync(path)
}

/// Synchronously renames (moves) the file or directory from `from` to `to`.
#[inline(always)]
pub fn rename_sync(from: &str, to: &str) -> std::result::Result<(), JsValue> {
    FS.fs_rename_sync(from, to)
}

/// Synchronously retrieves the [`fs.Stats`](https://nodejs.org/api/fs.html#class-fsstats)
/// object describing the file system entry at `path`.
#[inline(always)]
pub fn stat_sync(path: &str) -> std::result::Result<JsValue, JsValue> {
    FS.fs_stat_sync(path)
}
