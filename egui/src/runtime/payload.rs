use crate::imports::*;

struct Inner<T>
where
    T: Send,
{
    #[allow(dead_code)]
    id: String,
    payload: Mutex<Option<T>>,
    pending: AtomicBool,
}

pub struct Payload<T = ()>
where
    T: Send,
{
    inner: Arc<Inner<T>>,
}

impl<T> Clone for Payload<T>
where
    T: Send,
{
    fn clone(&self) -> Payload<T> {
        Payload {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Payload<T>
where
    T: Send + 'static,
{
    pub fn new<S: std::fmt::Display>(id: S) -> Self {
        let id = id.to_string();

        let mut registry = REGISTRY.lock().unwrap();

        if let Some(payload) = registry.get(&id) {
            if let Some(p) = payload.downcast_ref::<Payload<T>>() {
                let inner = p.inner.clone();
                Self { inner }
            } else {
                panic!("Unable to downcast Payload `{id}`");
            }
        } else {
            let inner = Arc::new(Inner {
                id: id.clone(),
                payload: Mutex::new(None),
                pending: AtomicBool::new(false),
            });

            registry.insert(
                id,
                Box::new(Payload {
                    inner: inner.clone(),
                }),
            );
            Self { inner }
        }
    }

    pub fn store(&self, data: T) {
        *self.inner.payload.lock().unwrap() = Some(data);
    }

    pub fn is_pending(&self) -> bool {
        self.inner.pending.load(Ordering::SeqCst)
    }

    pub fn mark_pending(&self) {
        self.inner.pending.store(true, Ordering::SeqCst);
    }

    pub fn clear_pending(&self) {
        self.inner.pending.store(false, Ordering::SeqCst);
    }

    pub fn is_some(&self) -> bool {
        self.inner.payload.lock().unwrap().is_some()
    }

    pub fn take(&self) -> Option<T> {
        if let Some(result) = self.inner.payload.lock().unwrap().take() {
            self.clear_pending();
            Some(result)
        } else {
            None
        }
    }

    pub fn inner_clone(&self) -> Option<T>
    where
        T: Clone,
    {
        self.inner.payload.lock().unwrap().clone()
    }
}

static REGISTRY: LazyLock<Mutex<HashMap<String, Box<dyn Any + Sync + Send>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
