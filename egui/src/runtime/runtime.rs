use super::*;
use crate::imports::*;
use workflow_wasm::callback::CallbackMap;

pub use payload::Payload;
use repaint::RepaintService;
pub use service::{Service, ServiceResult};

pub struct Inner {
    egui_ctx: egui::Context,
    events: ApplicationEventsChannel,
    services: RwLock<AHashMap<String, Arc<dyn Service>>>,
    repaint_service: Arc<RepaintService>,
    is_running: Arc<AtomicBool>,
    start_time: Instant,
    #[allow(dead_code)]
    callbacks: CallbackMap,
}

#[derive(Clone)]
pub struct Runtime {
    inner: Arc<Inner>,
}

impl Runtime {
    pub fn new(egui_ctx: &egui::Context, events: Option<ApplicationEventsChannel>) -> Self {
        let events = events.unwrap_or_else(channel::Channel::unbounded);

        let repaint_service = Arc::new(RepaintService::default());

        let runtime = Self {
            inner: Arc::new(Inner {
                services: Default::default(),
                events,
                repaint_service: repaint_service.clone(),
                egui_ctx: egui_ctx.clone(),
                is_running: Arc::new(AtomicBool::new(false)),
                start_time: Instant::now(),
                callbacks: Default::default(),
            }),
        };
        register_global(Some(runtime.clone()));

        runtime.bind(repaint_service);

        #[cfg(target_arch = "wasm32")]
        runtime.register_visibility_handler();

        runtime
    }

    pub fn bind(&self, service: Arc<dyn Service>) {
        self.inner
            .services
            .write()
            .unwrap()
            .insert(service.name().to_string(), service.clone());
        let runtime = self.clone();
        spawn(async move { service.spawn(runtime).await });
    }

    pub fn uptime(&self) -> Duration {
        self.inner.start_time.elapsed()
    }

    pub fn start_services(&self) {
        let services = self.services();
        for service in services {
            let runtime = self.clone();
            // service.spawn().await?;
            spawn(async move { service.spawn(runtime).await });
        }
    }

    pub fn services(&self) -> Vec<Arc<dyn Service>> {
        self.inner
            .services
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    pub fn stop_services(&self) {
        self.services()
            .into_iter()
            .for_each(|service| service.terminate());
    }

    pub async fn join_services(&self) {
        // for service in self.services() {
        //  let name = service.name();
        //  println!("⚡ {name}");
        //  service.join().await.expect("service join failure");
        //  println!("💀 {name}");
        // }

        let futures = self
            .services()
            .into_iter()
            .map(|service| service.join())
            .collect::<Vec<_>>();
        join_all(futures).await;
    }

    pub fn drop(&self) {
        register_global(None);
    }

    // / Start the runtime runtime.
    pub fn start(&self) {
        self.inner.is_running.store(true, Ordering::SeqCst);
        self.start_services();
    }

    /// Shutdown runtime runtime.
    pub async fn shutdown(&self) {
        if self.inner.is_running.load(Ordering::SeqCst) {
            self.inner.is_running.store(false, Ordering::SeqCst);
            self.stop_services();
            self.join_services().await;
            register_global(None);
        }
    }

    pub fn repaint_service(&self) -> &Arc<RepaintService> {
        &self.inner.repaint_service
    }

    /// Returns the reference to the application events channel.
    pub fn events(&self) -> &ApplicationEventsChannel {
        &self.inner.events
    }

    /// Send an application even to the UI asynchronously.
    pub async fn send<T>(&self, msg: T) -> Result<()>
    where
        T: Any + Send + Sync + 'static,
    {
        self.inner.events.send(RuntimeEvent::new(msg)).await?;
        Ok(())
    }

    pub async fn send_runtime_event(&self, msg: RuntimeEvent) -> Result<()> {
        self.inner.events.send(msg).await?;
        Ok(())
    }

    /// Send an application event to the UI synchronously.
    pub fn try_send<T>(&self, msg: T) -> Result<()>
    where
        T: Any + Send + Sync + 'static,
    {
        self.inner.events.sender.try_send(RuntimeEvent::new(msg))?;
        Ok(())
    }

    pub fn try_send_runtime_event(&self, msg: RuntimeEvent) -> Result<()> {
        self.inner.events.sender.try_send(msg)?;
        Ok(())
    }

    pub fn spawn_task<F>(&self, task: F)
    where
        F: Future<Output = Result<()>> + Send + 'static,
    {
        let sender = self.events().sender.clone();
        workflow_core::task::spawn(async move {
            if let Err(err) = task.await {
                sender
                    .send(RuntimeEvent::Error(err.to_string()))
                    .await
                    .unwrap();
            }
        });
    }

    pub fn spawn_task_with_result<R, F>(
        &self,
        payload: &Payload<std::result::Result<R, Error>>,
        task: F,
    ) where
        R: Clone + Send + 'static,
        F: Future<Output = Result<R>> + Send + 'static,
    {
        let payload = (*payload).clone();
        payload.mark_pending();
        workflow_core::task::spawn(async move {
            let result = task.await;
            match result {
                Ok(r) => payload.store(Ok(r)),
                Err(err) => {
                    payload.store(Err(err));
                }
            }
        });
    }

    pub fn egui_ctx(&self) -> &egui::Context {
        &self.inner.egui_ctx
    }

    pub fn request_repaint(&self) {
        self.repaint_service().trigger();
    }

    #[cfg(target_arch = "wasm32")]
    pub fn register_visibility_handler(&self) {
        use workflow_dom::utils::*;
        use workflow_wasm::callback::*;

        let sender = self.events().sender.clone();
        let callback = callback!(move || {
            let visibility_state = document().visibility_state();
            sender
                .try_send(RuntimeEvent::VisibilityState(visibility_state))
                .unwrap();
            runtime().egui_ctx().request_repaint();
        });

        document().set_onvisibilitychange(Some(callback.as_ref()));
        self.inner.callbacks.retain(callback).unwrap();
    }
}

static RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);

pub fn runtime() -> Runtime {
    if let Some(runtime) = RUNTIME.lock().unwrap().as_ref() {
        runtime.clone()
    } else {
        panic!("Error: `Runtime` is not initialized")
    }
}

pub fn try_runtime() -> Option<Runtime> {
    RUNTIME.lock().unwrap().clone()
}

fn register_global(runtime: Option<Runtime>) {
    match runtime {
        Some(runtime) => {
            let mut global = RUNTIME.lock().unwrap();
            if global.is_some() {
                panic!("runtime already initialized");
            }
            global.replace(runtime);
        }
        None => {
            RUNTIME.lock().unwrap().take();
        }
    };
}

/// Spawn an async task that will result in
/// egui redraw upon its completion.
pub fn spawn<F>(task: F)
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    runtime().spawn_task(task);
}

/// Spawn an async task that will result in
/// egui redraw upon its completion. Upon
/// the task completion, the supplied [`Payload`]
/// will be populated with the task result.
pub fn spawn_with_result<R, F>(payload: &Payload<std::result::Result<R, Error>>, task: F)
where
    R: Clone + Send + 'static,
    F: Future<Output = Result<R>> + Send + 'static,
{
    runtime().spawn_task_with_result(payload, task);
}

/// Gracefully halt the runtime runtime. This is used
/// to shutdown kaspad when the kaspa-ng process exit
/// is an inevitable eventuality.
#[cfg(not(target_arch = "wasm32"))]
impl Runtime {
    pub fn halt() {
        if let Some(runtime) = try_runtime() {
            runtime.try_send(RuntimeEvent::Exit).ok();
            // runtime.kaspa_service().clone().terminate();

            let handle = tokio::spawn(async move { runtime.shutdown().await });

            while !handle.is_finished() {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }

    /// Attempt to halt the runtime runtime but exit the process
    /// if it takes too long. This is used in attempt to shutdown
    /// kaspad if the kaspa-ng process panics, which can result
    /// in a still functioning zombie child process on unix systems.
    pub fn abort() {
        const TIMEOUT: u128 = 5000;
        let flag = Arc::new(AtomicBool::new(false));
        let flag_ = flag.clone();
        let thread = std::thread::Builder::new()
            .name("halt".to_string())
            .spawn(move || {
                let start = std::time::Instant::now();
                while !flag_.load(Ordering::SeqCst) {
                    if start.elapsed().as_millis() > TIMEOUT {
                        println!("halting...");
                        std::process::exit(1);
                    }
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            })
            .ok();

        Self::halt();

        flag.store(true, Ordering::SeqCst);
        if let Some(thread) = thread {
            thread.join().unwrap();
        }

        #[cfg(feature = "console")]
        {
            println!("Press Enter to exit...");
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
        }

        std::process::exit(1);
    }
}
