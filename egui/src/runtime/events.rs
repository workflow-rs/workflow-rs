use crate::imports::*;

/// The repaint-aware channel over which [`RuntimeEvent`]s are delivered to the
/// application event loop.
pub type ApplicationEventsChannel = crate::runtime::channel::Channel<RuntimeEvent>;

#[derive(Clone, Debug)]
/// A type-erased, cheaply-cloneable container for an application-defined event,
/// downcast back to its concrete type by the consumer.
pub struct ApplicationEvent(Arc<dyn Any + Send + Sync>);

impl ApplicationEvent {
    /// Wraps an arbitrary value as a type-erased application event.
    pub fn new<T>(event: T) -> Self
    where
        T: Any + Send + Sync + 'static,
    {
        ApplicationEvent(Arc::new(event))
    }

    #[allow(clippy::should_implement_trait)]
    /// Borrows the wrapped value as `&T`, panicking if the concrete type does
    /// not match `T`.
    pub fn as_ref<T>(&self) -> &T
    where
        T: Any,
    {
        self.0.downcast_ref::<T>().unwrap()
    }

    // pub fn into<T>(self) -> T
    // where
    //     T: Any + Send + Sync,
    // {
    //     let this = self
    //         .0
    //         .downcast::<T>()
    //         .expect("unknown application event type");
    //     // Arc::into_inner(this).expect("multiple references to application event type")
    //     Arc::unwrap_or_clone(this).expect("multiple references to application event type")
    // }

    /// Consumes the event and returns the wrapped value as an `Arc<T>`,
    /// panicking if the concrete type does not match `T`.
    pub fn as_arc<T>(self) -> Arc<T>
    where
        T: Any + Send + Sync,
    {
        self.0
            .downcast::<T>()
            .expect("unknown application event type")
    }
}

#[derive(Clone, Debug)]
/// Events delivered to the runtime event loop, covering runtime lifecycle
/// signals as well as custom application-defined events.
pub enum RuntimeEvent {
    /// An error condition reported to the runtime, carrying a message.
    Error(String),
    /// A request to gracefully shut down the application.
    Exit,
    /// A change in the application window's visibility state.
    VisibilityState(VisibilityState),
    /// A custom application-defined event.
    Application(ApplicationEvent),
}

impl RuntimeEvent {
    /// Wraps an arbitrary application-defined event in a
    /// [`RuntimeEvent::Application`] variant.
    pub fn new<T>(event: T) -> Self
    where
        T: Any + Send + Sync + 'static,
    {
        RuntimeEvent::Application(ApplicationEvent::new(event))
    }
}
