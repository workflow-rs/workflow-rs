//!
//! `Interval` stream backed by the Tokio `Interval` stream.
//!

#![allow(dead_code)]

use crate::channel::Channel;
use futures::{task::AtomicWaker, Stream};
use instant::Duration;
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll},
};

struct Inner {
    ready: AtomicBool,
    period: Mutex<Duration>,
    waker: AtomicWaker,
    shutdown_ctl: Channel,
    period_ctl: Channel<Duration>,
}

/// 
/// `Interval` stream used by the `interval()` function to provide a
/// a time interval stream. The stream is backed by tokio interval 
/// stream on native platforms and by by the JavaScript `setInterval()`
/// and `clearInterval()` APIs in WASM32 environment. 
/// 
/// This Interval stream has an advantage of having `Send` and `Sync` markers.
/// 
/// Please note that the `Interval` fires upon creation to mimic
/// the tokio-backed Interval stream available on the native target.
/// 
#[derive(Clone)]
pub struct Interval {
    inner: Arc<Inner>,
}

impl Interval {
    /// Create a new `Interval` stream that will resolve each given duration.
    pub fn new(duration: Duration) -> Self {
        let inner = Arc::new(Inner {
            ready: AtomicBool::new(false),
            waker: AtomicWaker::new(),
            period: Mutex::new(duration),
            shutdown_ctl: Channel::oneshot(),
            period_ctl: Channel::unbounded(),
        });

        let inner_ = inner.clone();

        crate::task::spawn(async move {
            let mut current_period = *inner_.period.lock().unwrap();

            'outer: loop {
                let mut interval = tokio::time::interval(current_period);

                'inner: loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            inner_.ready.store(true, Ordering::SeqCst);
                            inner_.waker.wake();
                        },
                        new_period = inner_.period_ctl.recv() => {
                            if let Ok(new_period) = new_period {
                                current_period = new_period;
                            } else {
                                // if the duration channel is closed, we stop the interval
                                break 'outer;
                            }
                            break 'inner;
                        },
                        _ = inner_.shutdown_ctl.recv() => {
                            break 'outer;
                        },
                    }
                }
            }
        });

        Interval { inner }
    }

    #[inline]
    fn change_period(&self, period: Duration) {
        self.inner
            .period_ctl
            .try_send(period)
            .expect("Interval::change_period() unable to send period signal");
    }

    #[inline]
    fn shutdown(&self) {
        self.inner
            .shutdown_ctl
            .try_send(())
            .expect("Interval::shutdown() unable to send shutdown signal");
    }

    /// Cancel the current timeout.
    pub fn cancel(&self) {
        self.shutdown();
    }
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.ready.load(Ordering::SeqCst) {
            true => {
                self.inner.ready.store(false, Ordering::SeqCst);
                Poll::Ready(Some(()))
            }
            false => {
                self.inner.waker.register(cx.waker());
                if self.inner.ready.load(Ordering::SeqCst) {
                    self.inner.ready.store(false, Ordering::SeqCst);
                    Poll::Ready(Some(()))
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// `async interval()` function backed by the JavaScript `createInterval()`
pub fn interval(duration: Duration) -> Interval {
    Interval::new(duration)
}
