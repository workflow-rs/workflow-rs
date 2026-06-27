//! Module for the file descriptor limit management.

///
#[cfg(not(target_arch = "wasm32"))]
pub fn try_set_fd_limit(limit: u64) -> std::io::Result<u64> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
                rlimit::setmaxstdio(limit as u32).map(|v| v as u64)
        } else if #[cfg(unix)] {
            rlimit::increase_nofile_limit(limit)
        }
    }
}

/// Returns the current open file descriptor limit for the process, or an error
/// on platforms where the limit cannot be queried.
pub fn limit() -> std::io::Result<i32> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            Ok(rlimit::getmaxstdio() as i32)
        }
        else if #[cfg(unix)] {
            Ok(rlimit::getrlimit(rlimit::Resource::NOFILE).unwrap().0 as i32)
        }
        else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported platform: file descriptor limit is not available"))
        }
    }
}
