pub mod error;
pub mod result;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use cfg_if::cfg_if;
use crate::result::Result;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        use async_std::path::PathBuf;
        use async_std::fs;
    } else {
        use base64::{encode, decode};
    }
}

///
/// # Unistore
/// 
/// A simple file loader that allows user to
/// specify different paths on various 
/// operating systems with fallbacks.
///
pub struct Unistore {
    // linux (fallsback to unix, generic)
    pub linux : Option<String>,
    // macos (fallsback to unix, generic)
    pub macos : Option<String>,
    // unix (fallsback to generic)
    pub unix : Option<String>,
    // windows (fallsback to generic)
    pub windows : Option<String>,
    // fallback for all OSes
    pub generic : Option<String>,
    // browser locastorage (fallsback to a hash of generic in hex)
    pub browser : Option<String>,
}

impl Unistore {

    pub fn new() -> Unistore {
        Unistore {
            linux: None,
            macos: None,
            unix: None,
            windows: None,
            generic: None,
            browser: None,
        }
    }

    pub fn with_linux(&mut self, linux: &str) -> &mut Unistore {
        self.linux = Some(linux.to_string());
        self
    }

    pub fn with_macos(&mut self, macos: &str) -> &mut Unistore {
        self.macos = Some(macos.to_string());
        self
    }

    pub fn with_unix(&mut self, unix: &str) -> &mut Unistore {
        self.unix = Some(unix.to_string());
        self
    }

    pub fn with_windows(&mut self, windows: &str) -> &mut Unistore {
        self.windows = Some(windows.to_string());
        self
    }

    pub fn with_generic(&mut self, generic: &str) -> &mut Unistore {
        self.generic = Some(generic.to_string());
        self
    }

    pub fn with_browser(&mut self, browser: &str) -> &mut Unistore {
        self.browser = Some(browser.to_string());
        self
    }

    pub fn filename(&self) -> String {
        cfg_if! {
            if #[cfg(target_os = "macos")] {
                find(&[self.macos.as_ref(),self.unix.as_ref(),self.generic.as_ref()])
            } else if #[cfg(target_os = "linux")] {
                find(&[self.linux.as_ref(),self.unix.as_ref(),self.generic.as_ref()])
            } else if #[cfg(target_family = "unix")] {
                find(&[self.unix.as_ref(),self.generic.as_ref()])
            } else if #[cfg(target_family = "windows")] {
                find(&[self.windows.as_ref(),self.generic.as_ref()])
            } else if #[cfg(target_arch = "wasm32")] {
                if let Some(browser) = self.browser.as_ref() {
                    browser.clone()
                } else if let Some(generic) = self.generic.as_ref() {
                    // hash of generic
                    hash(generic)
                } else {
                    panic!("no path found for the current operating environment");
                }
            }
        }
    }

    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            pub async fn exists(&self) -> Result<bool> {
                let filename = self.filename();
                Ok(local_storage().get_item(&filename)?.is_some())
            }
        
            pub async fn read(&self) -> Result<Vec<u8>> {
                let filename = self.filename();
                let v = local_storage().get_item(&filename)?.unwrap();
                Ok(decode(v)?)
            }
            
            pub async fn write(&self, data: &[u8]) -> Result<()> {
                let filename = self.filename();
                let v = encode(data);
                local_storage().set_item(&filename, &v)?;
                Ok(())
            }

        } else {
            pub async fn exists(&self) -> Result<bool> {
                let filename = parse(self.filename());
                Ok(filename.exists().await)
            }
        
            pub async fn read(&self) -> Result<Vec<u8>> {
                let filename = parse(self.filename());
                Ok(fs::read(&filename).await?)
            }
            
            pub async fn write(&self, data: &[u8]) -> Result<()> {
                let filename = parse(self.filename());
                Ok(fs::write(&filename, data).await?)
            }
        }
    }

}

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        pub fn parse(path : String) -> PathBuf {
        
            if path.starts_with("~") {
                let home_dir: PathBuf = home::home_dir().unwrap().into();
                home_dir.join(path[1..].to_string())
            } else {
                PathBuf::from(path)
            }
        }
    } else {
        pub fn local_storage() -> web_sys::Storage {
            web_sys::window().unwrap().local_storage().unwrap().unwrap()
        }
    }
}


pub fn find(paths : &[Option<&String>]) -> String {
    for path in paths.iter() {
        if let Some(path) = *path {
            return path.clone();
        }
    }
    panic!("no path found for the current operating environment");
}

fn hash<T>(t: T) -> String
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    let v = hasher.finish();
    format!("{:x}", v)
}

