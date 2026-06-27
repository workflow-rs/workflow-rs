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

/// A globally-registered, cheaply-cloneable slot for handing a value of type
/// `T` between async tasks and the UI, with an associated pending flag to track
/// in-flight operations. Instances sharing the same id share the same storage.
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
    /// Returns the payload registered under `id`, creating and registering a
    /// new empty one if none exists yet, so callers sharing an id share state.
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

    /// Stores a value in the slot, replacing any previously stored value.
    pub fn store(&self, data: T) {
        *self.inner.payload.lock().unwrap() = Some(data);
    }

    /// Returns `true` if the pending flag is set.
    pub fn is_pending(&self) -> bool {
        self.inner.pending.load(Ordering::SeqCst)
    }

    /// Sets the pending flag, indicating an operation is in progress.
    pub fn mark_pending(&self) {
        self.inner.pending.store(true, Ordering::SeqCst);
    }

    /// Clears the pending flag, indicating no operation is in progress.
    pub fn clear_pending(&self) {
        self.inner.pending.store(false, Ordering::SeqCst);
    }

    /// Returns `true` if a value is currently stored in the slot.
    pub fn is_some(&self) -> bool {
        self.inner.payload.lock().unwrap().is_some()
    }

    /// Removes and returns the stored value, clearing the pending flag, or
    /// returns `None` if the slot is empty.
    pub fn take(&self) -> Option<T> {
        match self.inner.payload.lock().unwrap().take() {
            Some(result) => {
                self.clear_pending();
                Some(result)
            }
            _ => None,
        }
    }

    /// Returns a clone of the stored value without removing it, or `None` if
    /// the slot is empty.
    pub fn inner_clone(&self) -> Option<T>
    where
        T: Clone,
    {
        self.inner.payload.lock().unwrap().clone()
    }
}

static REGISTRY: LazyLock<Mutex<HashMap<String, Box<dyn Any + Sync + Send>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
