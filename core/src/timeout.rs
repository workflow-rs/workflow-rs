//!
//! Experimental - do not use
//! 

use crate::time::Duration;
use futures::future::FusedFuture;
use std::future::Future;
use std::marker::Unpin;

pub struct Timeout;

pub async fn timeout<T>(
    duration: Duration,
    task: impl Future<Output = T> + FusedFuture + Unpin,
) -> std::result::Result<T, Timeout> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let sleep = crate::task::sleep(duration);
            futures::pin_mut!(sleep);
            futures::pin_mut!(task);
            futures::select! {
                _ = sleep => {
                    Err(Timeout)
                },
                t = task => {
                    Ok(t)
                }
            }

        } else {
            let sleep = tokio::time::sleep(duration);
            tokio::pin!(sleep);
            tokio::select! {
                _ = sleep => {
                    Err(Timeout)
                },
                t = task => {
                    Ok(t)
                }
            }

        }
    }
}
