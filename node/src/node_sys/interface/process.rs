use crate::node_sys::{
    class::{EventEmitter, stream},
    interface::{
        CpuUsage, Domain, HrTime, MemoryUsage, NodeModule, ProcessFeatures, ProcessRelease,
        ProcessSendOptions, ProcessVersions,
    },
};
use js_sys::{Function, JsString, Number, Object, Set};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug)]
    pub type Process;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, catch)]
    pub fn abort(this: &Process) -> Result<(), JsValue>;

    #[wasm_bindgen(method)]
    pub fn chdir(this: &Process, directory: JsString);

    // FIXME
    #[wasm_bindgen(method)]
    pub fn config(this: &Process) -> Object;

    #[wasm_bindgen(method, js_name = "cpuUsage")]
    pub fn cpu_usage(this: &Process, previous_value: Option<&CpuUsage>) -> CpuUsage;

    #[wasm_bindgen(method)]
    pub fn cwd(this: &Process) -> JsString;

    #[wasm_bindgen(method)]
    pub fn disconnect(this: &Process);

    #[wasm_bindgen(method, js_name = "emitWarning")]
    pub fn emit_warning(
        this: &Process,
        warning: &JsString,
        name: Option<&JsString>,
        ctor: Option<&Function>,
    );

    #[wasm_bindgen(method)]
    pub fn exit(this: &Process);

    #[wasm_bindgen(method, js_name = "getegid")]
    pub fn get_egid(this: &Process) -> i32;

    #[wasm_bindgen(method, js_name = "geteuid")]
    pub fn get_euid(this: &Process) -> i32;

    #[wasm_bindgen(method, js_name = "getgid")]
    pub fn get_gid(this: &Process) -> i32;

    #[wasm_bindgen(method, js_name = "getgroups")]
    pub fn get_groups(this: &Process) -> Box<[JsValue]>;

    #[wasm_bindgen(method, js_name = "getuid")]
    pub fn get_uid(this: &Process) -> i32;

    #[wasm_bindgen(method, catch)]
    pub fn kill(this: &Process, pid: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "kill")]
    pub fn kill_with_signal_name(
        this: &Process,
        pid: u32,
        signal_name: &JsString,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "kill")]
    pub fn kill_with_signal_id(this: &Process, pid: u32, signal_id: i32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = "memoryUsage")]
    pub fn memory_usage(this: &Process) -> MemoryUsage;

    #[wasm_bindgen(method, variadic, js_name = "nextTick")]
    pub fn next_tick(this: &Process, callback: &Function, args: Box<[JsValue]>);

    // FIXME: https://nodejs.org/api/process.html#process_process_resourceusage
    #[wasm_bindgen(method, js_name = "resourceUsage")]
    pub fn resource_usage(this: &Process) -> Object;

    // FIXME: might be undefined; how to handle nicely
    #[wasm_bindgen(method)]
    pub fn send(
        this: &Process,
        message: &JsValue,
        send_handle: Option<&Object>,
        options: Option<ProcessSendOptions>,
        callback: Option<&Function>,
    ) -> bool;

    #[wasm_bindgen(method, catch, js_name = "setegid")]
    pub fn set_egid(this: &Process, id: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "seteuid")]
    pub fn set_euid(this: &Process, id: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "setgid")]
    pub fn set_gid(this: &Process, id: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "setuid")]
    pub fn set_uid(this: &Process, id: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "setgroups")]
    pub fn set_groups(this: &Process, id: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = "setUncaughtExceptionCaptureCallback")]
    pub fn set_uncaught_exception_capture_callback(this: &Process, callback: &Function);

    #[wasm_bindgen(method)]
    pub fn umask(this: &Process, mask: Option<u32>) -> u32;

    #[wasm_bindgen(method)]
    pub fn uptime(this: &Process) -> f64;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "allowedNodeEnvironmentFlags")]
    pub fn allowed_node_environment_flags(this: &Process) -> Set;

    #[wasm_bindgen(method, getter)]
    pub fn arch(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn argv(this: &Process) -> Box<[JsValue]>;

    #[wasm_bindgen(method, getter)]
    pub fn argv0(this: &Process) -> JsString;

    // FIXME
    #[wasm_bindgen(method, getter)]
    pub fn connected(this: &Process) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn debug_port(this: &Process) -> Number;

    #[wasm_bindgen(method, getter)]
    pub fn domain(this: &Process) -> Option<Domain>;

    #[wasm_bindgen(method, getter)]
    pub fn env(this: &Process) -> Object;

    #[wasm_bindgen(method, getter, js_name = "execArgv")]
    pub fn exec_argv(this: &Process) -> Box<[JsValue]>;

    #[wasm_bindgen(method, getter, js_name = "execPath")]
    pub fn exec_path(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "exitCode")]
    pub fn exit_code(this: &Process) -> Option<i32>;

    #[wasm_bindgen(method, getter)]
    pub fn features(this: &Process) -> ProcessFeatures;

    #[wasm_bindgen(method, getter, js_name = "hrtime")]
    pub fn hr_time(this: &Process) -> HrTime;

    #[wasm_bindgen(method, getter, js_name = "mainModule")]
    pub fn main_module(this: &Process) -> Option<NodeModule>;

    #[wasm_bindgen(method, getter, js_name = "noDeprecation")]
    pub fn no_deprecation(this: &Process) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn pid(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn platform(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn ppid(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn release(this: &Process) -> ProcessRelease;

    // FIXME: https://nodejs.org/api/process.html#process_process_report
    #[wasm_bindgen(method, getter)]
    pub fn report(this: &Process) -> Object;

    #[wasm_bindgen(method, getter)]
    pub fn stderr(this: &Process) -> stream::Writable;

    #[wasm_bindgen(method, getter)]
    pub fn stdin(this: &Process) -> stream::Readable;

    #[wasm_bindgen(method, getter)]
    pub fn stdout(this: &Process) -> stream::Writable;

    #[wasm_bindgen(method, getter, js_name = "throwDeprecation")]
    pub fn throw_deprecation(this: &Process) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn title(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "traceDeprecation")]
    pub fn trace_deprecation(this: &Process) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn version(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn versions(this: &Process) -> ProcessVersions;
}
