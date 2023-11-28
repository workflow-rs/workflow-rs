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

    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type Fs;

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

#[inline(always)]
pub async fn readdir(path: &str) -> std::result::Result<JsValue, JsValue> {
    FSP.fs_readdir(path).await
}

#[inline(always)]
pub async fn readdir_with_options(
    path: &str,
    options: Object,
) -> std::result::Result<JsValue, JsValue> {
    FSP.fs_readdir_with_options(path, options).await
}

#[inline(always)]
pub fn readdir_sync(path: &str, callback: js_sys::Function) {
    FS.fs_readdir_sync(path, callback)
}

#[inline(always)]
pub fn exists_sync(path: &str) -> std::result::Result<bool, JsValue> {
    FS.fs_exists_sync(path)
}

#[inline(always)]
pub fn write_file_sync(
    path: &str,
    data: JsValue,
    options: Object,
) -> std::result::Result<(), JsValue> {
    FS.fs_write_file_sync(path, data, options)
}

#[inline(always)]
pub fn read_file_sync(path: &str, options: Object) -> std::result::Result<JsValue, JsValue> {
    FS.fs_read_file_sync(path, options)
}

#[inline(always)]
pub fn mkdir_sync(path: &str, options: Object) -> std::result::Result<(), JsValue> {
    FS.fs_mkdir_sync(path, options)
}

#[inline(always)]
pub fn unlink_sync(path: &str) -> std::result::Result<(), JsValue> {
    FS.fs_unlink_sync(path)
}

#[inline(always)]
pub fn rename_sync(from: &str, to: &str) -> std::result::Result<(), JsValue> {
    FS.fs_rename_sync(from, to)
}

#[inline(always)]
pub fn stat_sync(path: &str) -> std::result::Result<JsValue, JsValue> {
    FS.fs_stat_sync(path)
}
