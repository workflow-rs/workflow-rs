//!
//! [`task`](self) module provides helper functions for use with async closures that *operate uniformly*
//! in native ([`tokio`](https://crates.io/crates/tokio)-backed) and WASM ([`async_std`]-backed) environments
//! (i.e. a web browser).
//!
//! Following functions are are available:
//! - [`spawn()`] - non-blocking spawn of the supplied async closure
//! - [`sleep()`] - suspends the task for a given Duration
//! - [`yield_now()`] - yields rust executor
//! - [`yield_executor()`] - yields to top-level executor (browser async loop)
//!
//! A [`Task`] struct is also available and allows spawning an async closures while
//! providing it with an argument, a return value and a channel that signals termination.
//! Once started, the task can be externally terminated and/or waited until completion.
//!
//! <div class="example-wrap compile_fail"><pre class="compile_fail" style="white-space:normal;font:inherit;">
//! Blocking spawn is not available as browser-WASM can
//! not block task execution due to a single-threaded async environment.
//! </pre></div>
//!

#[allow(unused_imports)]
use cfg_if::cfg_if;
use futures::Future;
pub use workflow_core_macros::{set_task, task};

cfg_if! {
    if #[cfg(not(any(target_arch = "wasm32", target_os = "solana")))] {

        pub mod native {
            //! native implementation
            pub use super::*;

            pub use tokio::task::yield_now;
            pub use tokio::task::yield_now as yield_executor;
            pub use tokio::time::sleep;

            pub fn spawn<F, T>(future: F)
            where
            F: Future<Output = T> + Send + 'static,
            T: Send + 'static,
            {
                tokio::task::spawn(async {
                // async_std::task::spawn(async {
                    future.await
                });
            }
        }

        pub use native::*;
    }
}

pub mod wasm {
    //! WASM implementation
    pub use super::*;

    pub fn spawn<F, T>(_future: F)
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                // wasm32 spawn shim
                // spawn and spawn_local are currently not available on wasm32 architectures
                // ironically, block_on is but it spawns a task instead of blocking it
                // unfortunately access to [`async_std::task::Builder::local()`] is
                // private.
                async_std::task::block_on(async move { _future.await });
                // async_std::task::Builder::new().local(_future).unwrap();
            } else {
                panic!("workflow_core::task::wasm::spawn() is not allowed on non-wasm target");
            }
        }
    }

    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use std::sync::{Arc, Mutex};
            use wasm_bindgen::prelude::*;
            use instant::Duration;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen (catch, js_name = setTimeout)]
                pub fn set_timeout(closure: &Closure<dyn FnMut()>, timeout: u32) -> std::result::Result<u32, JsValue>;
                #[wasm_bindgen (catch, js_name = clearTimeout)]
                pub fn clear_timeout(interval: u32) -> std::result::Result<(), JsValue>;
                #[wasm_bindgen(js_name = requestAnimationFrame)]
                fn request_animation_frame(callback:js_sys::Function);
            }

            type SleepClosure = Closure<dyn FnMut()>;
            /// Suspends current task for the given [`Duration`]
            pub async fn sleep(duration : Duration) {
                let (sender, receiver) = crate::channel::oneshot::<()>();
                let interval = {
                    let mutex_init : Arc<Mutex<Option<SleepClosure>>> = Arc::new(Mutex::new(None));
                    let mutex_clear = mutex_init.clone();
                    let closure = Closure::new(move ||{
                        sender.try_send(()).unwrap();
                        *mutex_clear.clone().lock().unwrap() = None;
                    });
                    let interval = set_timeout(&closure, duration.as_millis() as u32).unwrap();
                    *mutex_init.lock().unwrap() = Some(closure);
                    interval
                };
                receiver.recv().await.unwrap();
                clear_timeout(interval).unwrap();
            }

            pub use async_std::task::yield_now;
            pub async fn yield_executor() {
                if !unsafe { REQUEST_ANIMATION_FRAME_INITIALIZED } {
                    init_yield();
                    unsafe { REQUEST_ANIMATION_FRAME_INITIALIZED = true };
                } else {
                    let promise = js_sys::Promise::new(&mut |res, _|{
                        request_animation_frame(res);
                    });
                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                }
            }

            static mut REQUEST_ANIMATION_FRAME_INITIALIZED: bool = false;

            fn init_yield(){
                let _ = js_sys::Function::new_no_args("
                    if (!this.requestAnimationFrame){
                        if (this.setImmediate)
                            this.requestAnimationFrame = (callback)=>setImmediate(callback)
                        else
                            this.requestAnimationFrame = (callback)=>setTimeout(callback, 0)
                    }
                ")
                .call0(&JsValue::undefined());
            }

        } else {
            pub use async_std::task::sleep;
            pub use async_std::task::yield_now;
        }
    }
}
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

use crate::channel::{oneshot, Receiver, RecvError, SendError, Sender, TryRecvError, TrySendError};
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Errors produced by the [`Task`] implementation
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("The task is not running")]
    NotRunning,
    #[error("The task is already running")]
    AlreadyRunning,
    #[error("Task channel send error {0}")]
    SendError(String),
    #[error("Task channel receive error: {0:?}")]
    RecvError(#[from] RecvError),
    #[error("Task channel try send error: {0}")]
    TrySendError(String),
    #[error("Task channel try receive {0:?}")]
    TryRecvError(#[from] TryRecvError),
}

impl<T> From<SendError<T>> for TaskError {
    fn from(err: SendError<T>) -> Self {
        TaskError::SendError(err.to_string())
    }
}

impl<T> From<TrySendError<T>> for TaskError {
    fn from(err: TrySendError<T>) -> Self {
        TaskError::SendError(err.to_string())
    }
}

/// Result type used by the [`Task`] implementation
pub type TaskResult<T> = std::result::Result<T, TaskError>;

pub type TaskFn<A, T> = Arc<Box<dyn Send + Sync + Fn(A, Receiver<()>) -> FnReturn<T> + 'static>>;
pub type FnReturn<T> = Pin<Box<(dyn Send + Sync + 'static + Future<Output = T>)>>;

struct TaskInner<A, T>
where
    A: Send,
    T: 'static,
{
    termination: (Sender<()>, Receiver<()>),
    completion: (Sender<T>, Receiver<T>),
    running: Arc<AtomicBool>,
    task_fn: Arc<Mutex<Option<TaskFn<A, T>>>>,
    args: PhantomData<A>,
}

impl<A, T> TaskInner<A, T>
where
    A: Send + Sync + 'static,
    T: Send + 'static,
{
    fn new_with_boxed_task_fn<FN>(task_fn: Box<FN>) -> Self
    //TaskInner<A, T>
    where
        FN: Send + Sync + Fn(A, Receiver<()>) -> FnReturn<T> + 'static,
    {
        let termination = oneshot();
        let completion = oneshot();

        TaskInner {
            termination,
            completion,
            running: Arc::new(AtomicBool::new(false)),
            task_fn: Arc::new(Mutex::new(Some(Arc::new(task_fn)))),
            args: PhantomData,
        }
    }

    pub fn blank() -> Self {
        let termination = oneshot();
        let completion = oneshot();
        TaskInner {
            termination,
            completion,
            running: Arc::new(AtomicBool::new(false)),
            task_fn: Arc::new(Mutex::new(None)),
            args: PhantomData,
        }
    }

    fn task_fn(&self) -> TaskFn<A, T> {
        self.task_fn
            .lock()
            .unwrap()
            .as_ref()
            .expect("Task::task_fn is not initialized")
            .clone()
    }

    /// Replace task fn with an alternate function.
    /// The passed function must be boxed.
    fn set_boxed_task_fn(
        &self,
        task_fn: Box<dyn Send + Sync + Fn(A, Receiver<()>) -> FnReturn<T> + 'static>,
    ) {
        let task_fn = Arc::new(task_fn);
        *self.task_fn.lock().unwrap() = Some(task_fn);
    }

    pub fn run<'l>(self: &'l Arc<Self>, args: A) -> TaskResult<&'l Arc<Self>> {
        if !self.completion.1.is_empty() {
            panic!("Task::run(): task completion channel is not empty");
        }

        if !self.termination.1.is_empty() {
            panic!("Task::run(): task termination channel is not empty");
        }

        let this = self.clone();
        let cb = self.task_fn();
        workflow_core::task::spawn(async move {
            this.running.store(true, Ordering::SeqCst);

            let result = cb(args, this.termination.1.clone()).await;
            this.running.store(false, Ordering::SeqCst);
            this.completion
                .0
                .send(result)
                .await
                .expect("Error signaling task completion");
        });

        Ok(self)
    }

    pub fn stop(&self) -> TaskResult<()> {
        if self.running.load(Ordering::SeqCst) {
            self.termination.0.try_send(())?;
        }
        Ok(())
    }

    /// Blocks until the task exits. Resolves immediately
    /// if the task is not running.
    pub async fn join(&self) -> TaskResult<T> {
        if self.running.load(Ordering::SeqCst) {
            Ok(self.completion.1.recv().await?)
        } else {
            Err(TaskError::NotRunning)
        }
    }

    /// Signals termination and blocks until the
    /// task exits.
    pub async fn stop_and_join(&self) -> TaskResult<T> {
        if self.running.load(Ordering::SeqCst) {
            self.termination.0.send(()).await?;
            Ok(self.completion.1.recv().await?)
        } else {
            Err(TaskError::NotRunning)
        }
    }
}

/// [`Task`] struct allows you to spawn an async fn that can run
/// in a loop as a task (similar to a thread), checking for a
/// termination signal (so that execution can be aborted),
/// upon completion returning a value to the creator.
///
/// You can pass a [`channel`](crate::channel) as an argument to the async
/// function if you wish to communicate with the task.
///
/// NOTE: You should always call `task.join().await` to await
/// for the task completion if re-using the task.
///
/// ```rust
/// let task = task!(
///     |args : (), stop : Receiver<()>| async move {
///         let mut index = args;
///         loop {
///             if stop.try_recv().is_ok() {
///                 break;
///             }
///             // ... do something ...
///             index += 1;
///         }
///         return index;
///     }
/// );
///
/// // spawn the task instance ...
/// // passing 256 as the `args` argument
/// task.run(256)?;
/// ...
///
/// // signal termination ...
/// task.stop()?;
///
/// // await for the task completion ...
/// // the `result` is the returned `index` value
/// let result = task.join().await?;
///
/// // rinse and repeat if needed
/// task.run(256)?;
///
/// ```
///
#[derive(Clone)]
pub struct Task<A, T>
where
    A: Send,
    T: 'static,
{
    inner: Arc<TaskInner<A, T>>,
}

impl<A, T> Task<A, T>
where
    A: Send + Sync + 'static,
    T: Send + 'static,
{
    ///
    /// Create a new [`Task`] instance by supplying it with
    /// an async closure that has 2 arguments:
    /// ```rust
    /// task!(|args, signal| async move {
    ///     // ...
    ///     return v;
    /// })
    /// ```
    pub fn new<FN>(task_fn: FN) -> Task<A, T>
    where
        FN: Send + Sync + Fn(A, Receiver<()>) -> FnReturn<T> + 'static,
    {
        Self::new_with_boxed_task_fn(Box::new(task_fn))
    }

    fn new_with_boxed_task_fn<FN>(task_fn: Box<FN>) -> Task<A, T>
    where
        FN: Send + Sync + Fn(A, Receiver<()>) -> FnReturn<T> + 'static,
    {
        Task {
            inner: Arc::new(TaskInner::new_with_boxed_task_fn(task_fn)),
        }
    }

    /// Create an instance of the task without any task function.
    /// The task function can be passed later via [`Task::set_task_fn()`].
    pub fn blank() -> Self {
        Task {
            inner: Arc::new(TaskInner::blank()),
        }
    }

    /// Replace task fn with an alternate function.
    /// The task must be restarted for the replacement
    /// to take effect.  The function passed does not
    /// need to be boxed.
    pub fn set_task_fn<FN>(&self, task_fn: FN)
    where
        FN: Send + Sync + Fn(A, Receiver<()>) -> FnReturn<T> + 'static,
    {
        self.inner.set_boxed_task_fn(Box::new(task_fn))
    }

    /// Run the task supplying the provided argument to the
    /// closure supplied at creation.
    pub fn run(&self, args: A) -> TaskResult<&Self> {
        self.inner.run(args)?;
        Ok(self)
    }

    /// Signal termination on the channel supplied
    /// to the task closure; The task has to check
    /// for the signal periodically or await on
    /// the future of the signal.
    pub fn stop(&self) -> TaskResult<()> {
        self.inner.stop()
    }

    /// Blocks until the task exits. Resolves immediately
    /// if the task is not running.
    pub async fn join(&self) -> TaskResult<T> {
        self.inner.join().await
    }

    /// Signals termination and blocks until the
    /// task exits.
    pub async fn stop_and_join(&self) -> TaskResult<T> {
        self.inner.stop_and_join().await
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {

    use super::*;
    use std::time::Duration;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    pub async fn test_task() {
        let task = Task::new(|args, stop| -> FnReturn<String> {
            Box::pin(async move {
                println!("starting task... {}", args);
                for i in 0..10 {
                    if stop.try_recv().is_ok() {
                        println!("stopping task...");
                        break;
                    }
                    println!("t: {}", i);
                    workflow_core::task::sleep(Duration::from_millis(500)).await;
                }
                println!("exiting task...");
                return format!("finished {args}");
            })
        });

        task.run("- first -").ok();

        for i in 0..5 {
            println!("m: {}", i);
            workflow_core::task::sleep(Duration::from_millis(500)).await;
        }

        let ret1 = task.join().await.expect("[ret1] task wait failed");
        println!("ret1: {:?}", ret1);

        task.stop().ok();

        task.run("- second -").ok();

        for i in 0..5 {
            println!("m: {}", i);
            workflow_core::task::sleep(Duration::from_millis(500)).await;
        }

        task.stop().ok();
        let ret2 = task.join().await.expect("[ret2] task wait failed");
        println!("ret2: {:?}", ret2);

        println!("done");
    }
}
