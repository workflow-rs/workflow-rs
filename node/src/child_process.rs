use crate::node_sys::*;
use crate::require;
use js_sys::{Array, Object};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use workflow_log::log_info;

lazy_static! {
    static ref CP: Cp = require("child_process").unchecked_into();
}

#[wasm_bindgen]
extern "C" {

    /// Binding to the Node.js `child_process` module, used to spawn child
    /// processes.
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type Cp;

    /// Spawns a child process running `cmd` with no arguments
    /// (`child_process.spawn`).
    #[wasm_bindgen(js_name = spawn, method)]
    pub fn cp_spawn(this: &Cp, cmd: &str) -> ChildProcess;

    /// Spawns a child process running `cmd` with the given `args`
    /// (`child_process.spawn`).
    #[wasm_bindgen(js_name = spawn, method)]
    pub fn cp_spawn_with_args(this: &Cp, cmd: &str, args: &SpawnArgs) -> ChildProcess;

    /// Spawns a child process running `cmd` with the given `args` and spawn
    /// `options` (`child_process.spawn`).
    #[wasm_bindgen(js_name = spawn, method)]
    pub fn cp_spawn_with_args_and_options(
        this: &Cp,
        cmd: &str,
        args: &SpawnArgs,
        options: &SpawnOptions,
    ) -> ChildProcess;

    /// JavaScript array of command-line arguments passed to a spawned process.
    #[wasm_bindgen(extends = Array, js_namespace = child_process)]
    #[derive(Debug, Clone, PartialEq)]
    pub type SpawnArgs;

    /// JavaScript options object (cwd, env, stdio, …) for spawning a process.
    #[wasm_bindgen(extends = Object, js_namespace = child_process)]
    #[derive(Debug, Clone, PartialEq)]
    pub type SpawnOptions;

    /// Handle to a spawned child process (Node.js `ChildProcess`), an
    /// [`EventEmitter`] exposing its streams and lifecycle.
    #[wasm_bindgen(extends = EventEmitter, js_namespace = child_process)]
    #[derive(Clone, Debug)]
    pub type ChildProcess;

    /// The process's exit code (valid once it has exited).
    #[wasm_bindgen(method, getter)]
    pub fn exit_code(this: &ChildProcess) -> u64;

    /// The operating-system process identifier.
    #[wasm_bindgen(method, getter)]
    pub fn pid(this: &ChildProcess) -> u64;

    /// The process's standard output stream.
    #[wasm_bindgen(method, getter)]
    pub fn stdout(this: &ChildProcess) -> ReadableStream;

    /// The process's standard error stream.
    #[wasm_bindgen(method, getter)]
    pub fn stderr(this: &ChildProcess) -> ReadableStream;

    /// The process's standard input stream.
    #[wasm_bindgen(method, getter)]
    pub fn stdin(this: &ChildProcess) -> WritableStream;

    /// Sends the default termination signal to the process; returns `true` if
    /// the signal was delivered successfully.
    #[wasm_bindgen(method)]
    pub fn kill(this: &ChildProcess) -> bool;

    #[wasm_bindgen(method, js_name=kill)]
    fn kill_with_signal_impl(this: &ChildProcess, signal: JsValue) -> bool;
}

unsafe impl Send for Cp {}
unsafe impl Sync for Cp {}

unsafe impl Send for ChildProcess {}
unsafe impl Sync for ChildProcess {}

unsafe impl Send for SpawnOptions {}
unsafe impl Sync for SpawnOptions {}

unsafe impl Send for SpawnArgs {}
unsafe impl Sync for SpawnArgs {}

/// Spawns a new child process running `cmd` with no arguments, returning a
/// handle to the resulting [`ChildProcess`].
#[inline(always)]
pub fn spawn(cmd: &str) -> ChildProcess {
    CP.cp_spawn(cmd)
}

/// Spawns a new child process running `cmd` with the given `args`, returning a
/// handle to the resulting [`ChildProcess`].
#[inline(always)]
pub fn spawn_with_args(cmd: &str, args: &SpawnArgs) -> ChildProcess {
    CP.cp_spawn_with_args(cmd, args)
}

/// Spawns a new child process running `cmd` with the given `args` and
/// `options`, returning a handle to the resulting [`ChildProcess`].
#[inline(always)]
pub fn spawn_with_args_and_options(
    cmd: &str,
    args: &SpawnArgs,
    options: &SpawnOptions,
) -> ChildProcess {
    CP.cp_spawn_with_args_and_options(cmd, args, options)
}

/// Signal to send to a child process when terminating it via
/// [`ChildProcess::kill_with_signal`].
#[derive(Debug)]
pub enum KillSignal<'s> {
    /// Send the default termination signal.
    None,
    /// Send `SIGKILL`, forcibly terminating the process.
    SIGKILL,
    /// Send `SIGTERM`, requesting a graceful termination.
    SIGTERM,
    /// Send a signal identified by name (e.g. `"SIGHUP"`).
    Message(&'s str),
    /// Send a signal identified by its numeric value.
    Code(u32),
}

impl ChildProcess {
    /// Sends a termination request to the child process using the given
    /// [`KillSignal`], returning whether the signal was delivered.
    pub fn kill_with_signal(self: &ChildProcess, signal: KillSignal) -> bool {
        log_info!("kill_with_signal {:?}", signal);
        match signal {
            KillSignal::None => self.kill(),
            KillSignal::SIGKILL => self.kill_with_signal_impl(JsValue::from("SIGKILL")),
            KillSignal::SIGTERM => self.kill_with_signal_impl(JsValue::from("SIGTERM")),
            KillSignal::Message(str) => self.kill_with_signal_impl(JsValue::from(str)),
            KillSignal::Code(code) => self.kill_with_signal_impl(JsValue::from(code)),
        }
    }
}

impl From<Vec<&str>> for SpawnArgs {
    fn from(list: Vec<&str>) -> Self {
        let array = Array::new();
        for (index, value) in list.iter().enumerate() {
            array.set(index as u32, JsValue::from(*value));
        }

        #[allow(unused_mut)]
        let mut args: Self = ::wasm_bindgen::JsCast::unchecked_into(array);
        args
    }
}

impl From<&[&str]> for SpawnArgs {
    fn from(list: &[&str]) -> Self {
        let array = Array::new();
        for (index, value) in list.iter().enumerate() {
            array.set(index as u32, JsValue::from(*value));
        }

        #[allow(unused_mut)]
        let mut args: Self = ::wasm_bindgen::JsCast::unchecked_into(array);
        args
    }
}

impl From<&[String]> for SpawnArgs {
    fn from(list: &[String]) -> Self {
        let array = Array::new();
        for (index, value) in list.iter().enumerate() {
            array.set(index as u32, JsValue::from(value));
        }

        #[allow(unused_mut)]
        let mut args: Self = ::wasm_bindgen::JsCast::unchecked_into(array);
        args
    }
}

impl Default for SpawnOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl SpawnOptions {
    /// "Construct a new `SpawnOptions`.
    ///
    /// [NODEJS Documentation](https://nodejs.org/api/child_process.html#child_processspawncommand-args-options)
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(Object::new());
        ret
    }

    /// Sets an arbitrary option `key` to `value` on the underlying options
    /// object, returning `self` for chaining.
    pub fn set(&self, key: &str, value: JsValue) -> &Self {
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from(key), &value);
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }

    /// Sets the current working directory of the child process.
    pub fn cwd(&self, cwd: &str) -> &Self {
        self.set("cwd", JsValue::from(cwd))
    }

    /// Sets the environment variables exposed to the child process.
    pub fn env(&self, env: ProcessEnv) -> &Self {
        self.set("env", JsValue::from(env))
    }

    /// Overrides the value sent to the child as `argv[0]` (the process name).
    pub fn argv0(&self, argv0: &str) -> &Self {
        self.set("argv0", JsValue::from(argv0))
    }

    /// Runs the child in its own process group, detached from the parent, when
    /// `true`.
    pub fn detached(&self, detached: bool) -> &Self {
        self.set("detached", JsValue::from(detached))
    }

    /// Sets the user identity under which the child process is run.
    pub fn uid(&self, uid: &str) -> &Self {
        self.set("uid", JsValue::from(uid))
    }

    /// Sets the group identity under which the child process is run.
    pub fn gid(&self, gid: &str) -> &Self {
        self.set("gid", JsValue::from(gid))
    }

    /// Sets the serialization format used for messages exchanged with the
    /// child process (e.g. `"json"` or `"advanced"`).
    pub fn serialization(&self, serialization: &str) -> &Self {
        self.set("serialization", JsValue::from(serialization))
    }

    /// Runs the command inside a shell when `true`, using the platform's
    /// default shell.
    pub fn shell(&self, shell: bool) -> &Self {
        self.set("shell", JsValue::from(shell))
    }

    /// Runs the command inside the shell at the given path.
    pub fn shell_str(&self, shell: &str) -> &Self {
        self.set("shell", JsValue::from(shell))
    }

    /// Controls whether arguments are passed verbatim (without automatic
    /// quoting/escaping) on Windows.
    pub fn windows_verbatim_arguments(&self, args: bool) -> &Self {
        self.set("windowsVerbatimArguments", JsValue::from(args))
    }

    /// Hides the subprocess console window that would normally be created on
    /// Windows.
    pub fn windows_hide(&self, windows_hide: bool) -> &Self {
        self.set("windowsHide", JsValue::from(windows_hide))
    }

    /// Sets the maximum time, in milliseconds, the process is allowed to run
    /// before it is killed.
    pub fn timeout(&self, timeout: u32) -> &Self {
        self.set("timeout", JsValue::from(timeout))
    }

    // TODO: AbortSignal

    /// Sets the signal used to terminate the child when it is killed, as a
    /// numeric signal value.
    pub fn kill_signal(&self, signal: u32) -> &Self {
        self.set("killSignal", JsValue::from(signal))
    }

    /// Sets the signal used to terminate the child when it is killed, as a
    /// signal name (e.g. `"SIGTERM"`).
    pub fn kill_signal_str(&self, signal: &str) -> &Self {
        self.set("killSignal", JsValue::from(signal))
    }

    /// Sets the child's stdio configuration using a shorthand string
    /// (e.g. `"pipe"`, `"inherit"`, `"ignore"`).
    pub fn stdio(&self, stdio: &str) -> &Self {
        self.set("stdio", JsValue::from(stdio))
    }

    /// Sets the child's stdio configuration from an array describing each
    /// standard stream (stdin, stdout, stderr, and any extras) individually.
    pub fn stdio_with_array(&self, array: js_sys::Array) -> &Self {
        self.set("stdio", array.into())
    }
}
