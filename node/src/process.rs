//!
//! Module encapsulating [`Process`] API for running child process daemons under Node.js and NWJS
//!
use crate::child_process::{
    spawn_with_args_and_options, ChildProcess, KillSignal, SpawnArgs, SpawnOptions,
};
use crate::error::Error;
use crate::result::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use futures::{select, FutureExt};
use node_sys::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use workflow_core::channel::{oneshot, Channel, Receiver, Sender};
use workflow_core::task::*;
use workflow_core::time::Instant;
use workflow_log::*;
use workflow_task::*;
use workflow_wasm::callback::*;
use workflow_wasm::jserror::*;

/// Version struct for standard version extraction from executables via `--version` output
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub none: bool,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Version {
        Version {
            major,
            minor,
            patch,
            none: false,
        }
    }

    pub fn none() -> Version {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
            none: true,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.none {
            write!(f, "n/a")
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}

/// Child process execution result
pub struct ExecutionResult {
    pub termination: Termination,
    pub stdout: String,
    pub stderr: String,
}

impl ExecutionResult {
    pub fn is_error(&self) -> bool {
        matches!(self.termination, Termination::Error(_))
    }
}

pub enum Termination {
    Exit(u32),
    Error(String),
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub enum Event {
    Start,
    Exit(u32),
    Error(String),
    Stdout(String),
    Stderr(String),
}

/// Options for [`Process`] daemon runner
pub struct Options {
    /// Process arguments (the first element is the process binary file name / executable)
    argv: Vec<String>,
    /// Current working directory
    cwd: Option<PathBuf>,
    /// Automatic restart on exit
    restart: bool,
    /// Delay between automatic restarts
    restart_delay: Duration,
    /// This flag triggers forceful process termination after a given period of time.
    /// At the termination, the process is issued a `SIGTERM` signal. If the process fails
    /// to exit after a given period of time and `use_force` is enabled, the process
    /// will be issued a `SIGKILL` signal, triggering it's immediate termination.
    use_force: bool,
    /// Delay period after which to issue a `SIGKILL` signal.
    use_force_delay: Duration,
    /// Events relay [`Event`] enum that carries events emitted by the child process
    /// this includes stdout and stderr output, [`Event::Exit`] in case of a graceful
    /// termination and [`Event::Error`] in case of an error.
    events: Channel<Event>,
    muted_buffer_capacity: Option<usize>,
    mute: bool,
}

#[allow(clippy::too_many_arguments)]
impl Options {
    pub fn new(
        argv: &[&str],
        cwd: Option<PathBuf>,
        restart: bool,
        restart_delay: Option<Duration>,
        use_force: bool,
        use_force_delay: Option<Duration>,
        events: Channel<Event>,
        muted_buffer_capacity: Option<usize>,
        mute: bool,
    ) -> Options {
        let argv = argv.iter().map(|s| s.to_string()).collect::<Vec<_>>();

        Options {
            argv,
            cwd,
            restart,
            restart_delay: restart_delay.unwrap_or_default(),
            use_force,
            use_force_delay: use_force_delay.unwrap_or(Duration::from_millis(10_000)),
            events,
            muted_buffer_capacity,
            mute,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            argv: Vec::new(),
            cwd: None,
            restart: true,
            restart_delay: Duration::from_millis(3_000),
            use_force: false,
            use_force_delay: Duration::from_millis(10_000),
            events: Channel::unbounded(),
            muted_buffer_capacity: None,
            mute: false,
        }
    }
}

struct Inner {
    argv: Mutex<Vec<String>>,
    cwd: Mutex<Option<PathBuf>>,
    running: AtomicBool,
    restart: AtomicBool,
    restart_delay: Mutex<Duration>,
    use_force: AtomicBool,
    use_force_delay: Mutex<Duration>,
    events: Channel<Event>,
    proc: Arc<Mutex<Option<Arc<ChildProcess>>>>,
    callbacks: CallbackMap,
    start_time: Arc<Mutex<Option<Instant>>>,
    mute: Arc<AtomicBool>,
    muted_buffer_capacity: Option<usize>,
    muted_buffer_stdout: Arc<Mutex<VecDeque<String>>>,
    muted_buffer_stderr: Arc<Mutex<VecDeque<String>>>,
}

unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

impl Inner {
    pub fn new(options: Options) -> Inner {
        Inner {
            argv: Mutex::new(options.argv),
            cwd: Mutex::new(options.cwd),
            running: AtomicBool::new(false),
            restart: AtomicBool::new(options.restart),
            restart_delay: Mutex::new(options.restart_delay),
            use_force: AtomicBool::new(options.use_force),
            use_force_delay: Mutex::new(options.use_force_delay),
            events: options.events,
            proc: Arc::new(Mutex::new(None)),
            callbacks: CallbackMap::new(),
            start_time: Arc::new(Mutex::new(None)),
            mute: Arc::new(AtomicBool::new(options.mute)),
            muted_buffer_capacity: options.muted_buffer_capacity,
            muted_buffer_stdout: Arc::new(Mutex::new(VecDeque::default())),
            muted_buffer_stderr: Arc::new(Mutex::new(VecDeque::default())),
        }
    }

    fn program(&self) -> String {
        self.argv.lock().unwrap().first().unwrap().clone()
    }

    fn args(&self) -> Vec<String> {
        self.argv.lock().unwrap()[1..].to_vec()
    }

    fn cwd(&self) -> Option<PathBuf> {
        self.cwd.lock().unwrap().clone()
    }

    pub fn uptime(&self) -> Option<Duration> {
        if self.running.load(Ordering::SeqCst) {
            self.start_time.lock().unwrap().map(|ts| ts.elapsed())
        } else {
            None
        }
    }

    fn buffer_muted(&self, data: buffer::Buffer, muted_buffer: &Arc<Mutex<VecDeque<String>>>) {
        let muted_buffer_capacity = self.muted_buffer_capacity.unwrap_or_default();
        if muted_buffer_capacity > 0 {
            let mut muted_buffer = muted_buffer.lock().unwrap();
            let buffer = String::from(data.to_string(None, None, None));
            let lines = buffer.split('\n').collect::<Vec<_>>();
            for line in lines {
                let line = line.trim();
                if !line.is_empty() {
                    muted_buffer.push_back(trim(line.to_string()));
                }
            }
            while muted_buffer.len() > muted_buffer_capacity {
                muted_buffer.pop_front();
            }
        }
    }

    fn drain_muted(
        &self,
        acc: &Arc<Mutex<VecDeque<String>>>,
        sender: &Sender<Event>,
        stdout: bool,
    ) -> Result<()> {
        let mut acc = acc.lock().unwrap();
        if stdout {
            acc.drain(..).for_each(|line| {
                sender.try_send(Event::Stdout(line)).unwrap();
            });
        } else {
            acc.drain(..).for_each(|line| {
                sender.try_send(Event::Stderr(line)).unwrap();
            });
        }
        Ok(())
    }

    pub fn toggle_mute(&self) -> Result<bool> {
        if self.mute.load(Ordering::SeqCst) {
            self.mute.store(false, Ordering::SeqCst);
            self.drain_muted(&self.muted_buffer_stdout, &self.events.sender, true)?;
            self.drain_muted(&self.muted_buffer_stderr, &self.events.sender, false)?;
            Ok(false)
        } else {
            self.mute.store(true, Ordering::SeqCst);
            Ok(true)
        }
    }

    pub fn mute(&self, mute: bool) -> Result<()> {
        if mute != self.mute.load(Ordering::SeqCst) {
            self.mute.store(mute, Ordering::SeqCst);
            if !mute {
                self.drain_muted(&self.muted_buffer_stdout, &self.events.sender, true)?;
                self.drain_muted(&self.muted_buffer_stderr, &self.events.sender, false)?;
            }
        }

        Ok(())
    }

    pub async fn run(self: &Arc<Self>, stop: Receiver<()>) -> Result<()> {
        if self.running.load(Ordering::SeqCst) {
            return Err(Error::AlreadyRunning);
        }

        'outer: loop {
            let termination = Channel::<Termination>::oneshot();

            self.start_time.lock().unwrap().replace(Instant::now());

            let proc = {
                let program = self.program();
                let args = &self.args();

                let args: SpawnArgs = args.as_slice().into();
                let options = SpawnOptions::new();
                if let Some(cwd) = &self.cwd() {
                    options.cwd(cwd.as_os_str().to_str().unwrap_or_else(|| {
                        panic!("Process::exec_with_args(): invalid path: {}", cwd.display())
                    }));
                }

                Arc::new(spawn_with_args_and_options(&program, &args, &options))
            };

            let this = self.clone();
            let exit_sender = termination.sender.clone();
            let exit = callback!(move |code: JsValue| {
                let code = code.as_f64().unwrap_or_default() as u32;
                this.events.sender.try_send(Event::Exit(code)).ok();
                exit_sender
                    .try_send(Termination::Exit(code))
                    .expect("unable to send close notification");
            });
            proc.on("exit", exit.as_ref());
            self.callbacks.retain(exit.clone())?;

            let this = self.clone();
            let error_sender = termination.sender.clone();
            let error = callback!(move |err: JsValue| {
                let msg = JsErrorData::from(err);
                this.events
                    .sender
                    .try_send(Event::Error(msg.to_string()))
                    .ok();
                error_sender
                    .try_send(Termination::Error(msg.to_string()))
                    .expect("unable to send close notification");
            });
            proc.on("error", error.as_ref());
            self.callbacks.retain(error.clone())?;

            let this = self.clone();
            let stdout_cb = callback!(move |data: buffer::Buffer| {
                if this.mute.load(Ordering::SeqCst) {
                    this.buffer_muted(data, &this.muted_buffer_stdout);
                } else {
                    this.events
                        .sender
                        .try_send(Event::Stdout(String::from(
                            data.to_string(None, None, None),
                        )))
                        .unwrap();
                }
            });
            proc.stdout().on("data", stdout_cb.as_ref());
            self.callbacks.retain(stdout_cb)?;

            let this = self.clone();
            let stderr_cb = callback!(move |data: buffer::Buffer| {
                if this.mute.load(Ordering::SeqCst) {
                    this.buffer_muted(data, &this.muted_buffer_stderr);
                } else {
                    this.events
                        .sender
                        .try_send(Event::Stderr(String::from(
                            data.to_string(None, None, None),
                        )))
                        .unwrap();
                }
            });
            proc.stderr().on("data", stderr_cb.as_ref());
            self.callbacks.retain(stderr_cb)?;

            *self.proc.lock().unwrap() = Some(proc.clone());
            self.running.store(true, Ordering::SeqCst);

            self.events.sender.try_send(Event::Start).unwrap();

            let kill = select! {
                // process exited
                e = termination.receiver.recv().fuse() => {

                    // if exited with error, abort...
                    if matches!(e,Ok(Termination::Error(_))) {
                        break;
                    }

                    // if restart is not required, break
                    if !self.restart.load(Ordering::SeqCst) {
                        break;
                    } else {
                        // sleep and then restart
                        let restart_delay = *self.restart_delay.lock().unwrap();
                        select! {
                            // slept well, aim to restart
                            _ = sleep(restart_delay).fuse() => {
                                false
                            },
                            // stop received while sleeping, break
                            _ = stop.recv().fuse() => {
                                break;
                            }
                        }
                    }
                },
                // manual shutdown while the process is running
                _ = stop.recv().fuse() => {
                    true
                }
            };

            if kill {
                // start process termination
                self.restart.store(false, Ordering::SeqCst);
                proc.kill_with_signal(KillSignal::SIGTERM);
                // if not using force, wait for process termination on SIGTERM
                if !self.use_force.load(Ordering::SeqCst) {
                    termination.receiver.recv().await?;
                    break;
                } else {
                    // if using force, sleep and kill with SIGKILL
                    let use_force_delay = sleep(*self.use_force_delay.lock().unwrap());
                    select! {
                        // process exited normally, break
                        _ = termination.receiver.recv().fuse() => {
                            break 'outer;
                        },
                        // post SIGKILL and wait for exit
                        _ = use_force_delay.fuse() => {
                            proc.kill_with_signal(KillSignal::SIGKILL);
                            termination.receiver.recv().await?;
                            break 'outer;
                        },
                    }
                }
            }
        }

        self.callbacks.clear();
        *self.proc.lock().unwrap() = None;
        self.running.store(false, Ordering::SeqCst);

        Ok(())
    }
}

/// The [`Process`] class facilitating execution of a Child Process in Node.js or NWJS
/// environments. This wrapper runs the child process as a daemon, restarting it if
/// it fails.  The process provides `stdout` and `stderr` output as channel [`Receiver`]
/// channels, allowing for a passive capture of the process console output.
#[derive(Clone)]
pub struct Process {
    inner: Arc<Inner>,
    task: Arc<Task<Arc<Inner>, ()>>,
}

unsafe impl Send for Process {}
unsafe impl Sync for Process {}

impl Process {
    /// Create new process instance
    pub fn new(options: Options) -> Process {
        let inner = Arc::new(Inner::new(options));

        let task = task!(|inner: Arc<Inner>, stop| async move {
            inner.run(stop).await.ok();
        });

        Process {
            inner,
            task: Arc::new(task),
        }
    }

    pub fn new_once(path: &str) -> Process {
        let options = Options::new(
            &[path],
            None,
            false,
            None,
            false,
            // None,
            // None,
            None,
            Channel::unbounded(),
            None,
            false,
        );

        Self::new(options)
    }

    pub async fn version(path: &str) -> Result<Version> {
        version(path).await
    }

    pub fn is_running(&self) -> bool {
        self.inner.running.load(Ordering::SeqCst)
    }

    pub fn mute(&self, mute: bool) -> Result<()> {
        self.inner.mute(mute)
    }

    pub fn toggle_mute(&self) -> Result<bool> {
        self.inner.toggle_mute()
    }

    pub fn uptime(&self) -> Option<Duration> {
        self.inner.uptime()
    }

    /// Obtain a clone of the channel [`Receiver`] that captures
    /// [`Event`] of the underlying process.
    pub fn events(&self) -> Receiver<Event> {
        self.inner.events.receiver.clone()
    }

    pub fn replace_argv(&self, argv: Vec<String>) {
        *self.inner.argv.lock().unwrap() = argv;
    }

    /// Run the process in the background.  Spawns an async task that
    /// monitors the process, capturing its output and restarting
    /// the process if it exits prematurely.
    pub fn run(&self) -> Result<()> {
        self.task.run(self.inner.clone())?;
        Ok(())
    }

    /// Issue a `SIGKILL` signal, terminating the process immediately.
    pub fn kill(&self) -> Result<()> {
        if !self.inner.running.load(Ordering::SeqCst) {
            Err(Error::NotRunning)
        } else if let Some(proc) = self.inner.proc.lock().unwrap().as_ref() {
            self.inner.restart.store(false, Ordering::SeqCst);
            proc.kill_with_signal(KillSignal::SIGKILL);
            Ok(())
        } else {
            Err(Error::ProcIsAbsent)
        }
    }

    /// Issue a `SIGTERM` signal causing the process to exit. The process
    /// will be restarted by the monitoring task.
    pub fn restart(&self) -> Result<()> {
        if !self.inner.running.load(Ordering::SeqCst) {
            Err(Error::NotRunning)
        } else if let Some(proc) = self.inner.proc.lock().unwrap().as_ref() {
            proc.kill_with_signal(KillSignal::SIGTERM);
            Ok(())
        } else {
            Err(Error::ProcIsAbsent)
        }
    }

    /// Stop the process by disabling auto-restart and issuing
    /// a `SIGTERM` signal. Returns `Ok(())` if the process
    /// is not running.
    pub fn stop(&self) -> Result<()> {
        if self.inner.running.load(Ordering::SeqCst) {
            self.inner.restart.store(false, Ordering::SeqCst);
            self.task.stop()?;
        }

        Ok(())
    }

    /// Join the process like you would a thread - this async
    /// function blocks until the process exits.
    pub async fn join(&self) -> Result<()> {
        if self.task.is_running() {
            self.task.join().await?;
        }
        Ok(())
    }

    /// Stop the process and block until it exits.
    pub async fn stop_and_join(&self) -> Result<()> {
        self.stop()?;
        self.join().await?;
        Ok(())
    }
}

/// Execute the process single time with custom command-line arguments.
/// Useful to obtain a version via `--version` or perform single-task
/// executions - not as a daemon.
pub async fn exec(
    // &self,
    argv: &[&str],
    cwd: Option<PathBuf>,
) -> Result<ExecutionResult> {
    let proc = *argv.first().unwrap();

    let args: SpawnArgs = argv[1..].into();
    let options = SpawnOptions::new();
    if let Some(cwd) = cwd {
        options.cwd(cwd.as_os_str().to_str().unwrap_or_else(|| {
            panic!("Process::exec_with_args(): invalid path: {}", cwd.display())
        }));
    }

    let termination = Channel::<Termination>::oneshot();
    let (stdout_tx, stdout_rx) = oneshot();
    let (stderr_tx, stderr_rx) = oneshot();

    let cp = spawn_with_args_and_options(proc, &args, &options);

    let exit = termination.sender.clone();
    let exit = callback!(move |code: u32| {
        exit.try_send(Termination::Exit(code))
            .expect("unable to send close notification");
    });
    cp.on("exit", exit.as_ref());

    let error = termination.sender.clone();
    let error = callback!(move |err: JsValue| {
        error
            .try_send(Termination::Error(format!("{:?}", err)))
            .expect("unable to send close notification");
    });
    cp.on("error", error.as_ref());

    let stdout_cb = callback!(move |data: buffer::Buffer| {
        stdout_tx
            .try_send(String::from(data.to_string(None, None, None)))
            .expect("unable to send stdout data");
    });
    cp.stdout().on("data", stdout_cb.as_ref());

    let stderr_cb = callback!(move |data: buffer::Buffer| {
        stderr_tx
            .try_send(String::from(data.to_string(None, None, None)))
            .expect("unable to send stderr data");
    });
    cp.stderr().on("data", stderr_cb.as_ref());

    let termination = termination.recv().await?;

    let mut stdout = String::new();
    for _ in 0..stdout_rx.len() {
        stdout.push_str(&stdout_rx.try_recv()?);
    }

    let mut stderr = String::new();
    for _ in 0..stderr_rx.len() {
        stderr.push_str(&stdout_rx.try_recv()?);
    }

    Ok(ExecutionResult {
        termination,
        stdout,
        stderr,
    })
}

/// Obtain the process version information by running it with `--version` argument.
pub async fn version(proc: &str) -> Result<Version> {
    let text = exec([proc, "--version"].as_slice(), None).await?.stdout;
    let vstr = if let Some(vstr) = text.split_whitespace().last() {
        vstr
    } else {
        return Ok(Version::none());
    };

    let v = vstr
        .split('.')
        .flat_map(|v| v.parse::<u64>())
        .collect::<Vec<_>>();

    if v.len() != 3 {
        return Ok(Version::none());
    }

    Ok(Version::new(v[0], v[1], v[2]))
}

pub fn trim(mut s: String) -> String {
    // let mut s = String::from(self);
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

// #[wasm_bindgen]
pub async fn test_child_process() {
    log_info!("running rust test() fn");
    workflow_wasm::panic::init_console_panic_hook();

    let proc = Process::new(Options::new(
        &["/Users/aspect/dev/kaspa-dev/kaspad/kaspad"],
        None,
        true,
        Some(Duration::from_millis(3000)),
        true,
        Some(Duration::from_millis(100)),
        Channel::unbounded(),
        None,
        false,
    ));
    // futures::task
    let task = task!(|events: Receiver<Event>, stop: Receiver<()>| async move {
        loop {
            select! {
                v = events.recv().fuse() => {
                    if let Ok(v) = v {
                        log_info!("| {:?}",v);
                    }
                },
                _ = stop.recv().fuse() => {
                    log_info!("stop...");
                    break;
                }
            }
            log_info!("in loop");
        }
    });
    task.run(proc.events()).expect("task.run()");

    proc.run().expect("proc.run()");

    sleep(Duration::from_millis(5_000)).await;

    proc.stop_and_join()
        .await
        .expect("proc.stop_and_join() failure");
    task.stop_and_join()
        .await
        .expect("task.stop_and_join() failure");
}
