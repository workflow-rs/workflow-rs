use crate::imports::*;

pub trait Shutdown {
    fn shutdown(&self);
}

pub struct Signals {
    runtime: Runtime,
    iterations: AtomicU64,
}

impl Signals {
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
                    signals.runtime.terminate();
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
