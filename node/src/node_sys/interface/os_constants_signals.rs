use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type OsConstantsSignal;

    #[wasm_bindgen(method, getter)]
    pub fn SIGHUP(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGINT(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGQUIT(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGILL(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGTRAP(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGABRT(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGIOT(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGBUS(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGFPE(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGKILL(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGUSR1(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGUSR2(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGSEGV(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGPIPE(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGALRM(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGTERM(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGCHLD(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGTKFLT(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGCONT(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGSTOP(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGTSTP(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGBREAK(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGTTIN(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGTTOU(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGURG(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGXCPU(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGXFSZ(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGVTALRM(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGPROF(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGWINCH(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGIO(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGPOLL(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGLOST(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGPWR(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGINFO(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGSYS(this: &OsConstantsSignal) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn SIGUNUSED(this: &OsConstantsSignal) -> i32;
}
