use wasm_bindgen::prelude::*;
use node_child_process::{
    ChildProcess,
    KillSignal,
    SpawnOptions,
    SpawnArgs,
    spawn_with_args_and_options
};
use crate::error::Error;
use crate::result::Result;
use futures::{select, FutureExt};
use node_sys::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use workflow_core::channel::{Channel, Receiver};
use workflow_core::task::*;
use workflow_log::*;
use workflow_wasm::callback::*;

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

pub struct ExecutionResult {
    pub exit_code: u32,
    pub stdout: String,
    pub stderr: String,
}

pub struct Options {
    /// process arguments (first element is a process name)
    argv: Vec<String>,
    /// current working directory
    cwd: Option<PathBuf>,
    /// restart on exit
    restart: bool,
    /// delay
    restart_delay: Duration,
    use_force: bool,
    use_force_delay: Duration,
    // env : HashMap<String, String>,
}

impl Options {
    pub fn new(
        argv: &[&str],
        cwd: Option<PathBuf>,
        restart: bool,
        restart_delay: Option<Duration>,
        use_force: bool,
        use_force_delay: Option<Duration>,
    ) -> Options {
        let argv = argv.iter().map(|s| s.to_string()).collect::<Vec<_>>();

        Options {
            argv,
            cwd,
            restart,
            restart_delay: restart_delay.unwrap_or_default(),
            use_force,
            use_force_delay: use_force_delay.unwrap_or(Duration::from_millis(10_000)),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            argv: Vec::new(),
            cwd: None,
            restart: true,
            restart_delay: Duration::default(),
            use_force: false,
            use_force_delay: Duration::from_millis(10_000),
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
    stdout: Channel<String>,
    stderr: Channel<String>,
    exit: Channel<u32>,
    proc: Arc<Mutex<Option<Arc<ChildProcess>>>>,
    callbacks: CallbackMap,
}

impl Inner {
    pub fn new(options: &Options) -> Inner {
        Inner {
            argv: Mutex::new(options.argv.clone()),
            cwd: Mutex::new(options.cwd.clone()),
            running: AtomicBool::new(false),
            restart: AtomicBool::new(options.restart),
            restart_delay: Mutex::new(options.restart_delay),
            use_force: AtomicBool::new(options.use_force),
            use_force_delay: Mutex::new(options.use_force_delay),
            stdout: Channel::unbounded(),
            stderr: Channel::unbounded(),
            exit: Channel::oneshot(),
            proc: Arc::new(Mutex::new(None)),
            callbacks: CallbackMap::new(),
        }
    }

    fn program(&self) -> String {
        self.argv.lock().unwrap().get(0).unwrap().clone()
    }

    fn args(&self) -> Vec<String> {
        self.argv.lock().unwrap()[1..].to_vec()
    }

    fn cwd(&self) -> Option<PathBuf> {
        self.cwd.lock().unwrap().clone()
    }

    pub async fn run(&self, stop: Receiver<()>) -> Result<()> {
        loop {
            if self.running.load(Ordering::SeqCst) {
                return Err(Error::AlreadyRunning);
            }

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

                Arc::new(spawn_with_args_and_options(
                    &program, &args, &options,
                ))
            };

            let exit = self.exit.sender.clone();
            let close = callback!(move |code: u32| {
                exit.try_send(code)
                    .expect("unable to send close notification");
            });
            proc.on("close", close.as_ref());
            self.callbacks.retain(close.clone())?;

            let stdout_tx = self.stdout.sender.clone();
            let stdout_cb = callback!(move |data: buffer::Buffer| {
                stdout_tx
                    .try_send(String::from(data.to_string(None, None, None)))
                    .unwrap();
            });
            proc.stdout().on("data", stdout_cb.as_ref());
            self.callbacks.retain(stdout_cb)?;

            let stderr_tx = self.stderr.sender.clone();
            let stderr_cb = callback!(move |data: buffer::Buffer| {
                stderr_tx
                    .try_send(String::from(data.to_string(None, None, None)))
                    .unwrap();
            });
            proc.stderr().on("data", stderr_cb.as_ref());
            self.callbacks.retain(stderr_cb)?;

            *self.proc.lock().unwrap() = Some(proc.clone());
            self.running.store(true, Ordering::SeqCst);

            let kill = select! {
                _ = self.exit.receiver.recv().fuse() => {
                    if !self.restart.load(Ordering::SeqCst) {
                        break;
                    } else {
                        let restart_delay = *self.restart_delay.lock().unwrap();
                        select! {
                            _ = sleep(restart_delay).fuse() => {
                                false
                            },
                            _ = stop.recv().fuse() => {
                                true
                            }
                        }
                    }
                },
                // manual shutdown
                _ = stop.recv().fuse() => {
                    true
                }
            };

            if kill && self.running.load(Ordering::SeqCst) {
                self.restart.store(false, Ordering::SeqCst);
                proc.kill_with_signal(KillSignal::SIGTERM);
                if !self.use_force.load(Ordering::SeqCst) {
                    self.exit.receiver.recv().await?;
                } else {
                    let use_force_delay = sleep(*self.use_force_delay.lock().unwrap());
                    select! {
                        _ = self.exit.receiver.recv().fuse() => {
                            break;
                        },
                        _ = use_force_delay.fuse() => {
                            proc.kill_with_signal(KillSignal::SIGKILL);
                            self.exit.receiver.recv().await?;
                            break;
                        }
                    }
                }
            }
        }
        log_info!("loop done...");

        self.callbacks.clear();
        *self.proc.lock().unwrap() = None;
        self.running.store(false, Ordering::SeqCst);

        Ok(())
    }
}

#[derive(Clone)]
pub struct Process {
    inner: Arc<Inner>,
    task: Arc<Task<Arc<Inner>, ()>>,
}

impl Process {
    pub fn new(options: &Options) -> Process {
        let inner = Arc::new(Inner::new(options));

        let task = task!(|inner: Arc<Inner>, stop| async move {
            inner.run(stop).await.ok();
        });
        log_info!("creating process");
        Process {
            inner,
            task: Arc::new(task),
        }
    }

    pub fn stdout(&self) -> Receiver<String> {
        self.inner.stdout.receiver.clone()
    }

    pub fn stderr(&self) -> Receiver<String> {
        self.inner.stderr.receiver.clone()
    }

    pub fn run(&self) -> Result<()> {
        log_info!("run...");
        self.task.run(self.inner.clone())?;
        log_info!("run done...");
        Ok(())
    }

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

    pub fn stop(&self) -> Result<()> {
        log_info!("running.load");
        if !self.inner.running.load(Ordering::SeqCst) {
            return Err(Error::NotRunning);
        }

        log_info!("running.restart store");
        self.inner.restart.store(false, Ordering::SeqCst);
        log_info!("last");

        log_info!("kill");
        self.task.stop()?;
        log_info!("stop is done");

        Ok(())
    }

    pub async fn join(&self) -> Result<()> {
        self.task.join().await?;
        Ok(())
    }

    pub async fn stop_and_join(&self) -> Result<()> {
        log_info!("calling stop();");
        self.stop()?;
        log_info!("calling join();");
        self.join().await?;
        Ok(())
    }

    pub async fn exec_with_args(
        &self,
        args: &[&str],
        cwd: Option<PathBuf>,
    ) -> Result<ExecutionResult> {
        let proc = self.inner.program();
        let args: SpawnArgs = args.into();
        let options = SpawnOptions::new();
        if let Some(cwd) = cwd {
            options.cwd(cwd.as_os_str().to_str().unwrap_or_else(|| {
                panic!("Process::exec_with_args(): invalid path: {}", cwd.display())
            }));
        }

        let cp = spawn_with_args_and_options(&proc, &args, &options);
        let exit = self.inner.exit.sender.clone();
        let close = callback!(move |code: u32| {
            exit.try_send(code)
                .expect("unable to send close notification");
        });
        cp.on("close", close.as_ref());

        let stdout_tx = self.inner.stdout.sender.clone();
        let stdout_cb = callback!(move |data: buffer::Buffer| {
            stdout_tx
                .try_send(String::from(data.to_string(None, None, None)))
                .expect("unable to send stdout data");
        });
        cp.stdout().on("data", stdout_cb.as_ref());

        let stderr_tx = self.inner.stderr.sender.clone();
        let stderr_cb = callback!(move |data: buffer::Buffer| {
            stderr_tx
                .try_send(String::from(data.to_string(None, None, None)))
                .expect("unable to send stderr data");
        });
        cp.stderr().on("data", stderr_cb.as_ref());

        let exit_code = self.inner.exit.recv().await?;

        let mut stdout = String::new();
        for _ in 0..self.inner.stdout.len() {
            stdout.push_str(&self.inner.stdout.try_recv()?);
        }

        let mut stderr = String::new();
        for _ in 0..self.inner.stderr.len() {
            stderr.push_str(&self.inner.stderr.try_recv()?);
        }

        Ok(ExecutionResult {
            exit_code,
            stdout,
            stderr,
        })
    }

    pub async fn get_version(&self) -> Result<Version> {
        let text = self.exec_with_args(["--version"].as_slice(), None).await?.stdout;
        let v = text
            .split('.')
            .flat_map(|v| v.parse::<u64>())
            .collect::<Vec<_>>();

        if v.len() != 3 {
            return Ok(Version::none());
        }

        Ok(Version::new(v[0], v[1], v[2]))
    }
}



#[wasm_bindgen]
pub async fn test() {
    log_info!("running rust test() fn");
    workflow_wasm::panic::init_console_panic_hook();
    //log_info!("process.pid:{:?}", process.pid());
    //let id = process.get_uid();
    //log_info!("process.get_gid(): id:{}, {:?}", id, process.get_gid());
    //process.kill(id.try_into().unwrap());

    let proc = Process::new(&Options::new(
        // &["ls", "-m", "-s"],
        // &["/Users/aspect/dev/kaspa-dev/kaspad"],
        &["/Users/aspect/dev/kaspa-dev/kaspad/kaspad"],
        None,
        true,
        Some(Duration::from_millis(3000)),
        true,
        Some(Duration::from_millis(100)),
    ));
    // futures::task
    let task = task!(|stdout: Receiver<String>, stop: Receiver<()>| async move {
        loop {
            select! {
                v = stdout.recv().fuse() => {
                    if let Ok(v) = v {
                        log_info!("| {}",v);
                    }
                },
                _ = stop.recv().fuse() => {
                    // if let Ok(v) = v {
                        log_info!("stop...");
                        break;
                    // }
                }

            }
            log_info!("in loop");
        }
        // proc
    });
    task.run(proc.stdout()).expect("task.run()");

    proc.run().expect("proc.run()");

    sleep(Duration::from_millis(5_000)).await;

    log_info!("### === STOPPING PROCESS ...");
    proc.stop_and_join()
        .await
        .expect("proc.stop_and_join() failure");
    task.stop_and_join()
        .await
        .expect("task.stop_and_join() failure");
    // let args: SpawnArgs = ["-m", "-s"].as_slice().into(); // = SpawnArgs::from(&["-m", "-s"]);
    //                                                                      // let args = SpawnArgs::from(["-m", "-s"].as_slice());
    // let options = SpawnOptions::new();
    // options.cwd("../");

    // //let cp = spawn("ls");
    // //let cp = spawn_with_args("ls", &args);
    // let cp = spawn_with_args_and_options("ls", &args, &options);

    // //log_info!("spawn('ls'): {:#?}", cp);

    // // let close_callback = Closure::<dyn Fn(buffer::Buffer)>::new(move |data:buffer::Buffer|{
    // //     log_info!("close: {}", data.to_string(None, None, None));
    // // });
    // // let data_callback = Closure::<dyn Fn(buffer::Buffer)>::new(move |data:buffer::Buffer|{
    // //     log_info!("data: {}", data.to_string(None, None, None));
    // // });

    // let (sender, receiver) = oneshot();

    // // cp.on("close", close_callback.as_ref().unchecked_ref());
    // let close = callback!(move |data: buffer::Buffer| {
    //     log_info!("close: {}", data.to_string(None, None, None));
    //     sender
    //         .try_send(())
    //         .expect("unable to send close notification");
    // });
    // cp.on("close", close.as_ref());
    // // cp.stdout().on("data", data_callback.as_ref().unchecked_ref());
    // let data = callback!(move |data: buffer::Buffer| {
    //     log_info!("data: {}", data.to_string(None, None, None));
    // });
    // cp.stdout().on("data", data.as_ref());

    // receiver
    //     .recv()
    //     .await
    //     .expect("error receiving close notification");
    // // close_callback.forget();
    // // data_callback.forget();

    // //let p = require("process");
    // //log_info!("process: {:?}", p);
}
