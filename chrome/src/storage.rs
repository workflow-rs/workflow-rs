use crate::error::Error;
use cfg_if::cfg_if;
use chrome_sys::storage;
use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;
use workflow_core::task::call_async_send;

pub struct LocalStorage;

impl LocalStorage {
    pub async fn set_item(key: &str, value: &str) -> Result<JsValue, JsValue> {
        let key = key.to_string();
        let value = value.to_string();
        call_async_send!(async move {
            let data = Object::new();
            js_sys::Reflect::set(&data, &key.into(), &value.into())?;
            storage::set(data.into()).await
        })
    }

    pub async fn get_item(key: &str) -> Result<Option<String>, JsValue> {
        let _key = key.to_string();
        let obj = call_async_send!(storage::get(_key).await)?;
        Ok(js_sys::Reflect::get(&obj, &key.into())?.as_string())
    }

    pub async fn get_items(keys: Vec<&str>) -> Result<StorageData, JsValue> {
        let keys = keys.iter().map(|k| k.to_string()).collect::<Vec<_>>();
        Ok(call_async_send!(async move {
            let query = Array::new();
            for key in keys {
                query.push(&key.into());
            }
            storage::get_items(query).await
        })?
        .try_into()?)
    }

    pub async fn get_all() -> Result<StorageData, JsValue> {
        Ok(call_async_send!(storage::get_all().await)?.try_into()?)
    }

    pub async fn keys() -> Result<Vec<String>, JsValue> {
        Ok(Self::get_all().await?.keys())
    }

    pub async fn remove_item(key: &str) -> Result<(), JsValue> {
        let key = key.to_string();
        call_async_send!(storage::remove(key).await)
    }

    pub async fn rename_item(from_key: &str, to_key: &str) -> Result<(), Error> {
        let from_key = from_key.to_string();
        let to_key = to_key.to_string();

        if Self::get_item(&to_key).await?.is_some() {
            return Err(Error::KeyExists(to_key));
        }
        if let Some(existing) = Self::get_item(&from_key).await? {
            Self::set_item(&to_key, &existing).await?;
            Self::remove_item(&from_key).await?;
            Ok(())
        } else {
            Err(Error::MissingKey(from_key))
        }
    }

    pub async fn remove_items(keys: Vec<&str>) -> Result<(), JsValue> {
        let keys = keys.iter().map(|k| k.to_string()).collect::<Vec<_>>();
        call_async_send!(async move {
            let query = Array::new();
            for key in keys {
                query.push(&key.into());
            }
            storage::remove_items(query).await
        })
    }

    pub async fn clear() -> Result<(), JsValue> {
        call_async_send!(storage::clear().await)
    }

    #[cfg(debug_assertions)]
    pub async fn unit_tests() -> Result<(), String> {
        use workflow_core::sendable::Sendable;

        let old_data = Sendable(Self::get_all().await.unwrap());
        let error = Sendable(test_impl().await.err());
        let old_data_clone = old_data.clone();
        call_async_send!(storage::set(old_data.unwrap().inner.into()).await).unwrap();

        let new_data = Self::get_all().await.unwrap();
        for key in new_data.keys() {
            let new_value = new_data.get(&key).unwrap();
            let old_value = old_data_clone.get(&key).unwrap();
            if new_value != old_value {
                return Err(format!(
                    "[WARNING] Data restore failed: {key} => {old_value:?} != {new_value:?}"
                ));
            }
        }

        if let Some(err) = error.unwrap() {
            return Err(err.as_string().unwrap_or(format!("{err:?}")));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StorageData {
    pub inner: Object,
}

impl TryFrom<JsValue> for StorageData {
    type Error = JsError;
    fn try_from(inner: JsValue) -> Result<Self, Self::Error> {
        if !inner.is_object() {
            return Err(JsError::new(&format!(
                "Invalid JsValue: cant convert JsValue ({inner:?}) to StorageData."
            )));
        }
        let inner = Object::from(inner);
        Ok(Self { inner })
    }
}

impl StorageData {
    pub fn keys(&self) -> Vec<String> {
        let mut keys = vec![];
        for key in Object::keys(&self.inner) {
            keys.push(key.as_string().unwrap());
        }

        keys
    }

    pub fn has(&self, key: &str) -> bool {
        self.inner.has_own_property(&key.into())
    }

    pub fn get_value(&self, key: &str) -> Result<Option<JsValue>, JsValue> {
        let value = js_sys::Reflect::get(&self.inner, &key.into())?;
        if value.eq(&JsValue::UNDEFINED) {
            Ok(None)
        } else {
            Ok(Some(value))
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, JsValue> {
        let value = js_sys::Reflect::get(&self.inner, &key.into())?;
        if value.eq(&JsValue::UNDEFINED) {
            Ok(None)
        } else {
            Ok(value.as_string())
        }
    }
}

#[cfg(debug_assertions)]
macro_rules! assert_test {
    ($name:literal, $cond1:expr, $cond2:expr) => {{
        if $cond1 != $cond2 {
            return Result::<(), JsValue>::Err(
                format!(
                    "{} => {}, {:?} != {:?}",
                    $name,
                    stringify!($cond1),
                    $cond1,
                    $cond2
                )
                .into(),
            );
        }
    }};
}

#[cfg(debug_assertions)]
async fn test_impl() -> Result<(), JsValue> {
    LocalStorage::clear().await?;
    {
        let empty_data = LocalStorage::get_all().await?;
        assert_test!("Key length should be 0", empty_data.keys().len(), 0);
    }
    {
        LocalStorage::set_item("key1", "value-1").await?;
        let data = LocalStorage::get_all().await?;
        assert_test!("Key length should be 1", data.keys().len(), 1);
        assert_test!("'key1' key should be there", data.has("key1"), true);
        assert_test!(
            "value for 'key1' key should be 'value-1'",
            data.get("key1")?.unwrap(),
            "value-1"
        );
    }
    {
        let item = LocalStorage::get_item("key1").await?.unwrap();
        assert_test!("value for 'key1' key should be 'value-1'", item, "value-1");
    }
    {
        LocalStorage::set_item("key2", "value-2").await?;
        let data = LocalStorage::get_all().await?;
        assert_test!("Key length should be 2", data.keys().len(), 2);
        assert_test!("'key2' key should be there", data.has("key2"), true);
        assert_test!(
            "value for 'key2' key should be 'value-2'",
            data.get("key2")?.unwrap(),
            "value-2"
        );
    }
    {
        let item = LocalStorage::get_item("key2").await?.unwrap();
        assert_test!("value for 'key2' key should be 'value-2'", item, "value-2");
    }
    {
        LocalStorage::set_item("key3", "value-3").await?;
        let data = LocalStorage::get_all().await?;
        assert_test!("Key length should be 3", data.keys().len(), 3);
        assert_test!("'key3' key should be there", data.has("key3"), true);
        assert_test!(
            "value for 'key3' key should be 'value-3'",
            data.get("key3")?.unwrap(),
            "value-3"
        );
    }
    {
        let data = LocalStorage::get_items(vec!["key2", "key3"]).await?;
        assert_test!("Key length should be 2", data.keys().len(), 2);
        assert_test!("'key2' key should be there", data.has("key2"), true);
        assert_test!("'key3' key should be there", data.has("key3"), true);
        assert_test!(
            "value for 'key2' key should be 'value-2'",
            data.get("key2")?.unwrap(),
            "value-2"
        );
        assert_test!(
            "value for 'key3' key should be 'value-3'",
            data.get("key3")?.unwrap(),
            "value-3"
        );
    }
    {
        LocalStorage::remove_item("key2").await?;
        let data = LocalStorage::get_all().await?;
        assert_test!(
            "After remove_item, Key length should be 2",
            data.keys().len(),
            2
        );
        assert_test!("'key2' key should not be there", data.has("key2"), false);
        assert_test!(
            "value for 'key2' key should be None",
            data.get("key2")?,
            Option::<String>::None
        );
    }
    {
        LocalStorage::clear().await?;
        let data = LocalStorage::get_all().await?;
        assert_test!("After clear, Key length should be 0", data.keys().len(), 0);
        assert_test!("'key1' key should not be there", data.has("key1"), false);
        assert_test!(
            "value for 'key2' key should be None",
            data.get("key2")?,
            Option::<String>::None
        );
    }

    Ok(())
}

pub async fn __chrome_storage_unit_test() {
    cfg_if! {
        if #[cfg(all(target_arch = "wasm32", debug_assertions))] {
            if !workflow_core::runtime::is_chrome_extension() {
                workflow_log::log_info!("ChromeStorage::test() FAILED: these are unit tests for chrome extension storage api.");
                return
            }
            use LocalStorage as ChromeStorage;
            match ChromeStorage::unit_tests().await{
                Ok(_)=>workflow_log::log_info!("ChromeStorage::test() PASSED"),
                Err(err)=>workflow_log::log_error!("ChromeStorage::test() FAILED: {err:?}")
            };
        }
    }
}
