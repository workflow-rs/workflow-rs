use crate::imports::*;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        pub const TARGET_FPS: u64 = 30;
    } else {
        pub const TARGET_FPS: u64 = 24;
    }
}
pub const REPAINT_INTERVAL_MILLIS: u64 = 1000 / TARGET_FPS;

pub enum RepaintServiceEvents {
    Exit,
}

pub struct RepaintService {
    pub service_events: Channel<RepaintServiceEvents>,
    pub task_ctl: Channel<()>,
    pub repaint: Arc<AtomicBool>,
}

impl Default for RepaintService {
    fn default() -> Self {
        Self {
            service_events: Channel::unbounded(),
            task_ctl: Channel::oneshot(),
            repaint: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl RepaintService {
    pub fn trigger(&self) {
        self.repaint.store(true, Ordering::SeqCst);
    }

    pub fn clear(&self) {
        self.repaint.store(false, Ordering::SeqCst);
    }
}

#[async_trait]
impl Service for RepaintService {
    fn name(&self) -> &'static str {
        "repaint-service"
    }

    async fn spawn(self: Arc<Self>, runtime: Runtime) -> Result<()> {
        let mut interval = task::interval(Duration::from_millis(REPAINT_INTERVAL_MILLIS));

        loop {
            select! {
                _ = interval.next().fuse() => {
                    // TODO use compare_exchange()
                    if self.repaint.load(Ordering::SeqCst) {
                        self.repaint.store(false, Ordering::SeqCst);
                        runtime.egui_ctx().request_repaint();
                    }
                },
                msg = self.as_ref().service_events.receiver.recv().fuse() => {
                    if let Ok(event) = msg {
                        match event {
                            RepaintServiceEvents::Exit => {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        self.task_ctl.send(()).await.unwrap();
        Ok(())
    }

    fn terminate(self: Arc<Self>) {
        self.service_events
            .sender
            .try_send(RepaintServiceEvents::Exit)
            .unwrap();
    }

    async fn join(self: Arc<Self>) -> Result<()> {
        self.task_ctl.recv().await.unwrap();
        Ok(())
    }
}
