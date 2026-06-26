use crate::require;
use js_sys::{Array, Object};
use lazy_static::lazy_static;
use node_sys::*;
use wasm_bindgen::prelude::*;
use workflow_log::log_info;

lazy_static! {
    static ref CP: Cp = require("child_process").unchecked_into();
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type Cp;

    #[wasm_bindgen(js_name = spawn, method)]
    pub fn cp_spawn(this: &Cp, cmd: &str) -> ChildProcess;

    #[wasm_bindgen(js_name = spawn, method)]
    pub fn cp_spawn_with_args(this: &Cp, cmd: &str, args: &SpawnArgs) -> ChildProcess;

    #[wasm_bindgen(js_name = spawn, method)]
    pub fn cp_spawn_with_args_and_options(
        this: &Cp,
        cmd: &str,
        args: &SpawnArgs,
        options: &SpawnOptions,
    ) -> ChildProcess;

    #[wasm_bindgen(extends = Array, js_namespace = child_process)]
    #[derive(Debug, Clone, PartialEq)]
    pub type SpawnArgs;

    #[wasm_bindgen(extends = Object, js_namespace = child_process)]
    #[derive(Debug, Clone, PartialEq)]
    pub type SpawnOptions;

    #[wasm_bindgen(extends = EventEmitter, js_namespace = child_process)]
    #[derive(Clone, Debug)]
    pub type ChildProcess;

    #[wasm_bindgen(method, getter)]
    pub fn exit_code(this: &ChildProcess) -> u64;

    #[wasm_bindgen(method, getter)]
    pub fn pid(this: &ChildProcess) -> u64;

    #[wasm_bindgen(method, getter)]
    pub fn stdout(this: &ChildProcess) -> ReadableStream;

    #[wasm_bindgen(method, getter)]
    pub fn stderr(this: &ChildProcess) -> ReadableStream;

    #[wasm_bindgen(method, getter)]
    pub fn stdin(this: &ChildProcess) -> WritableStream;

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

#[inline(always)]
pub fn spawn(cmd: &str) -> ChildProcess {
    CP.cp_spawn(cmd)
}

#[inline(always)]
pub fn spawn_with_args(cmd: &str, args: &SpawnArgs) -> ChildProcess {
    CP.cp_spawn_with_args(cmd, args)
}

#[inline(always)]
pub fn spawn_with_args_and_options(
    cmd: &str,
    args: &SpawnArgs,
    options: &SpawnOptions,
) -> ChildProcess {
    CP.cp_spawn_with_args_and_options(cmd, args, options)
}

#[derive(Debug)]
pub enum KillSignal<'s> {
    None,
    SIGKILL,
    SIGTERM,
    Message(&'s str),
    Code(u32),
}

impl ChildProcess {
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

    pub fn set(&self, key: &str, value: JsValue) -> &Self {
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from(key), &value);
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }

    pub fn cwd(&self, cwd: &str) -> &Self {
        self.set("cwd", JsValue::from(cwd))
    }

    pub fn env(&self, env: ProcessEnv) -> &Self {
        self.set("env", JsValue::from(env))
    }

    pub fn argv0(&self, argv0: &str) -> &Self {
        self.set("argv0", JsValue::from(argv0))
    }

    pub fn detached(&self, detached: bool) -> &Self {
        self.set("detached", JsValue::from(detached))
    }

    pub fn uid(&self, uid: &str) -> &Self {
        self.set("uid", JsValue::from(uid))
    }

    pub fn gid(&self, gid: &str) -> &Self {
        self.set("gid", JsValue::from(gid))
    }

    pub fn serialization(&self, serialization: &str) -> &Self {
        self.set("serialization", JsValue::from(serialization))
    }

    pub fn shell(&self, shell: bool) -> &Self {
        self.set("shell", JsValue::from(shell))
    }

    pub fn shell_str(&self, shell: &str) -> &Self {
        self.set("shell", JsValue::from(shell))
    }

    pub fn windows_verbatim_arguments(&self, args: bool) -> &Self {
        self.set("windowsVerbatimArguments", JsValue::from(args))
    }

    pub fn windows_hide(&self, windows_hide: bool) -> &Self {
        self.set("windowsHide", JsValue::from(windows_hide))
    }

    pub fn timeout(&self, timeout: u32) -> &Self {
        self.set("timeout", JsValue::from(timeout))
    }

    // TODO: AbortSignal

    pub fn kill_signal(&self, signal: u32) -> &Self {
        self.set("killSignal", JsValue::from(signal))
    }

    pub fn kill_signal_str(&self, signal: &str) -> &Self {
        self.set("killSignal", JsValue::from(signal))
    }

    pub fn stdio(&self, stdio: &str) -> &Self {
        self.set("stdio", JsValue::from(stdio))
    }

    pub fn stdio_with_array(&self, array: js_sys::Array) -> &Self {
        self.set("stdio", array.into())
    }
}
