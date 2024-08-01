use crate::imports::*;

pub type ApplicationEventsChannel = crate::runtime::channel::Channel<RuntimeEvent>;

#[derive(Clone, Debug)]
pub struct ApplicationEvent(Arc<dyn Any + Send + Sync>);

impl ApplicationEvent {
    pub fn new<T>(event: T) -> Self
    where
        T: Any + Send + Sync + 'static,
    {
        ApplicationEvent(Arc::new(event))
    }

    #[allow(clippy::should_implement_trait)]
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
pub enum RuntimeEvent {
    Error(String),
    Exit,
    VisibilityState(VisibilityState),
    Application(ApplicationEvent),
}

impl RuntimeEvent {
    pub fn new<T>(event: T) -> Self
    where
        T: Any + Send + Sync + 'static,
    {
        RuntimeEvent::Application(ApplicationEvent::new(event))
    }
}
