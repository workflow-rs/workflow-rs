use crate::imports::*;

/// Implemented by types that can be asked to shut down.
pub trait Shutdown {
    /// Initiates shutdown of the implementor.
    fn shutdown(&self);
}

/// Installs an OS interrupt (Ctrl-C / SIGTERM) handler that terminates a runtime.
pub struct Signals {
    runtime: Runtime,
    iterations: AtomicU64,
}

impl Signals {
    /// Installs a signal handler that terminates `runtime` on the first
    /// interrupt and forcibly exits the process on a subsequent one.
    pub fn bind(runtime: &Runtime) {
        let signals = Arc::new(Signals {
            runtime: runtime.clone(),
            iterations: AtomicU64::new(0),
        });

        ctrlc::set_handler(move || {
            let v = signals.iterations.fetch_add(1, Ordering::SeqCst);

            match v {
                0 => {
                    println!("^SIGTERM - shutting down...");
                    if let Err(e) = signals.runtime.terminate() {
                        println!("Error terminating runtime: {}", e);
                    }
                }
                _ => {
                    println!("^SIGTERM - halting");
                    std::process::exit(1);
                }
            }
        })
        .expect("Error setting signal handler");
    }
}
