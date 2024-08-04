use crate::imports::*;

struct Inner {
    services: Mutex<Vec<Arc<dyn Service>>>,
    is_running: Arc<AtomicBool>,
    termination: Channel<()>,
}

impl Shutdown for Inner {
    fn shutdown(&self) {
        self.termination.try_send(()).unwrap();
    }
}

#[derive(Clone)]
pub struct Runtime {
    inner: Arc<Inner>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            inner: Arc::new(Inner {
                services: Mutex::new(Vec::new()),
                is_running: Arc::new(AtomicBool::new(false)),
                termination: Channel::oneshot(),
            }),
        }
    }
}

impl Runtime {
    pub fn bind(&self, service: Arc<dyn Service>) {
        self.inner.services.lock().unwrap().push(service);
    }

    fn services(&self) -> Vec<Arc<dyn Service>> {
        self.inner.services.lock().unwrap().clone()
    }

    async fn start_services(&self) -> Result<()> {
        let services = self.services();
        let mut active = vec![];
        for service in services {
            let runtime = self.clone();
            if debug() {
                println!("✨ {}", service.name());
            }
            match service.clone().spawn(runtime).await {
                Ok(_) => active.push(service),
                Err(err) => {
                    log_error!("Service spawn error: {err}");
                    self.stop_services(Some(active.clone()));
                    self.join_services(Some(active)).await;
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    fn stop_services(&self, services: Option<Vec<Arc<dyn Service>>>) {
        services
            .unwrap_or_else(|| self.services())
            .into_iter()
            .for_each(|service| {
                if debug() {
                    println!("⛬ {}", service.name());
                }
                service.terminate();
            });
    }

    async fn join_services(&self, services: Option<Vec<Arc<dyn Service>>>) {
        let services = services
            .unwrap_or_else(|| self.services())
            .into_iter()
            .rev();

        if debug() {
            for service in services {
                let name = service.name();
                println!("⚡ {name}");
                service.join().await.expect("service join failure");
                println!("💀 {name}");
            }
        } else {
            let futures = services.map(|service| service.join()).collect::<Vec<_>>();
            join_all(futures).await;
        }
    }

    /// Start the runtime runtime.
    async fn start(&self) -> Result<()> {
        self.inner.is_running.store(true, Ordering::SeqCst);
        self.start_services().await
    }

    /// Shutdown runtime runtime.
    async fn shutdown(&self) {
        if self.inner.is_running.load(Ordering::SeqCst) {
            self.inner.is_running.store(false, Ordering::SeqCst);
            self.stop_services(None);
            self.join_services(None).await;
        }
    }

    pub async fn run(&self) -> Result<()> {
        self.start().await?;
        let (finish_sender, finish_receiver) = oneshot();
        let runtime = self.clone();
        spawn(async move {
            runtime.inner.termination.recv().await.unwrap();
            runtime.shutdown().await;
            finish_sender.send(()).await.unwrap();
        });

        finish_receiver.recv().await.unwrap();
        Ok(())
    }

    pub fn terminate(&self) {
        self.inner.termination.try_send(()).unwrap();
    }
}
