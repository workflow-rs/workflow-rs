use crate::result::Result;
use cfg_if::cfg_if;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        use std::path::PathBuf;
        use tokio::fs;
    } else {
        // use base64::{Engine as _, engine::general_purpose};
    }
}

///
/// # Store
///
/// A simple file loader that allows user to
/// specify different paths on various
/// operating systems with fallbacks.
///
pub struct Store {
    /// Path used on Linux (falls back to `unix`, then `generic`).
    pub linux: Option<String>,
    /// Path used on macOS (falls back to `unix`, then `generic`).
    pub macos: Option<String>,
    /// Path used on Unix targets (falls back to `generic`).
    pub unix: Option<String>,
    /// Path used on Windows (falls back to `generic`).
    pub windows: Option<String>,
    /// Fallback path for all operating systems.
    pub generic: Option<String>,
    /// Browser local-storage key (falls back to a hex hash of `generic`).
    pub browser: Option<String>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    /// Creates a new [`Store`] with all paths unset.
    pub fn new() -> Store {
        Store {
            linux: None,
            macos: None,
            unix: None,
            windows: None,
            generic: None,
            browser: None,
        }
    }

    /// Sets the path used on Linux targets.
    pub fn with_linux(&mut self, linux: &str) -> &mut Store {
        self.linux = Some(linux.to_string());
        self
    }

    /// Sets the path used on macOS targets.
    pub fn with_macos(&mut self, macos: &str) -> &mut Store {
        self.macos = Some(macos.to_string());
        self
    }

    /// Sets the Unix path, used as a fallback for Linux and macOS targets.
    pub fn with_unix(&mut self, unix: &str) -> &mut Store {
        self.unix = Some(unix.to_string());
        self
    }

    /// Sets the path used on Windows targets.
    pub fn with_windows(&mut self, windows: &str) -> &mut Store {
        self.windows = Some(windows.to_string());
        self
    }

    /// Sets the generic fallback path used when no OS-specific path matches.
    pub fn with_generic(&mut self, generic: &str) -> &mut Store {
        self.generic = Some(generic.to_string());
        self
    }

    /// Sets the browser local-storage key used under the `wasm32` target.
    pub fn with_browser(&mut self, browser: &str) -> &mut Store {
        self.browser = Some(browser.to_string());
        self
    }

    /// Resolves the storage path for the current operating environment,
    /// applying the OS-specific fallback chain. Panics if no suitable
    /// path has been configured.
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

            pub async fn read_to_string(&self) -> Result<String> {
                let filename = self.filename();
                let v = local_storage().get_item(&filename)?.unwrap();
                // Ok(general_purpose::STANDARD.decode(v)?)
                Ok(v)
            }

            pub async fn write_string(&self, data: &str) -> Result<()> {
                let filename = self.filename();
                // let v = general_purpose::STANDARD.encode(data);
                local_storage().set_item(&filename, data)?;
                Ok(())
            }

        } else {
            /// Returns `true` if a file exists at the resolved file path.
            pub async fn exists(&self) -> Result<bool> {
                let filename = parse(self.filename());
                Ok(fs::metadata(&filename).await.is_ok())
            }

            /// Reads the entire contents of the resolved file path into a string.
            pub async fn read_to_string(&self) -> Result<String> {
                let filename = parse(self.filename());
                Ok(fs::read_to_string(&filename).await?)
            }

            /// Writes the given string to the resolved file path,
            /// overwriting any existing contents.
            pub async fn write_string(&self, data: &str) -> Result<()> {
                let filename = parse(self.filename());
                Ok(fs::write(&filename, data).await?)
            }
        }
    }
}

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        /// Converts a path string into a [`PathBuf`], expanding a leading
        /// `~` to the user's home directory.
        pub fn parse(path : String) -> PathBuf {

            if let Some(stripped) = path.strip_prefix('~') {
                let home_dir: PathBuf = home::home_dir().unwrap();
                home_dir.join(stripped)
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

/// Returns the first `Some` path from the given list of candidates,
/// panicking if every candidate is `None`.
pub fn find(paths: &[Option<&String>]) -> String {
    for path in paths.iter() {
        if let Some(path) = *path {
            return path.clone();
        }
    }
    panic!("no path found for the current operating environment");
}

/// Computes a stable hexadecimal hash string for any [`Hash`]-able value,
/// used to derive a browser local-storage key when no explicit key is given.
pub fn hash<T>(t: T) -> String
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    let v = hasher.finish();
    format!("{v:x}")
}
