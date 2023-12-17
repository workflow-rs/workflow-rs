//!
//! [`LookupHandler`] provides ability to queue multiple async requests for the same key
//! into a group of futures that resolve upon request completion.
//!
//! This functionality is useful when a client may be making multiple requests
//! for data that is not available and may need to be fetched over a transport
//! that may take time (such as network I/O). Each async request for the same
//! key will get queued into a set of futures all of which will resolve once
//! the initial request is resolved.
//!

use crate::channel::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

/// Custom result type used by [`LookupHandler`]
pub type LookupResult<V, E> = std::result::Result<V, E>;
pub enum RequestType<V, E> {
    New(Receiver<LookupResult<V, E>>),
    Pending(Receiver<LookupResult<V, E>>),
}

/// List of channel senders awaiting for the same key lookup.
pub type SenderList<V, E> = Vec<Sender<LookupResult<V, E>>>;

///
///  [`LookupHandler`] provides ability to queue multiple async requests for the same key
/// into a group of futures that resolve upon request completion.
///
/// To use [`LookupHandler`], you need to create a custom lookup function. The example below
/// declares a function `lookup()` that uses [`LookupHandler`] to queue requests
/// and if there are no pending requests (request is new) performs the actual
/// request by calling `lookup_impl()`. The [`LookupHandler::complete()`] will
/// resolve all pending futures for the specific key.
///
/// Example:
/// ```no_run
/// ...
/// pub lookup_handler : LookupHandler<Pubkey,Arc<Data>,Error>
/// ...
/// async fn lookup(&self, pubkey:&Pubkey) -> Result<Option<Arc<Data>>> {
///     let request_type = self.lookup_handler.queue(pubkey).await;
///     let result = match request_type {
///         RequestType::New(receiver) => {
///             // execute the actual lookup
///             let response = self.lookup_impl(pubkey).await;
///             // signal completion for all awaiting futures
///             lookup_handler.complete(pubkey, response).await;
///             // this request is queued like all the others
///             // so wait for your own notification as well
///             receiver.recv().await?
///         },
///         RequestType::Pending(receiver) => {
///             receiver.recv().await?
///         }
///     }
/// };
/// ```
pub struct LookupHandler<K, V, E> {
    pub map: Arc<Mutex<HashMap<K, SenderList<V, E>>>>,
    pending: AtomicUsize,
}

/// Default trait for the LookupHandler
impl<K, V, E> Default for LookupHandler<K, V, E>
where
    V: Clone,
    K: Clone + Eq + Hash + std::fmt::Debug,
    E: Clone,
{
    fn default() -> Self {
        LookupHandler::new()
    }
}

impl<K, V, E> LookupHandler<K, V, E>
where
    V: Clone,
    K: Clone + Eq + Hash + std::fmt::Debug,
    E: Clone,
{
    /// Create a new instance of the LookupHandler
    pub fn new() -> Self {
        LookupHandler {
            map: Arc::new(Mutex::new(HashMap::new())),
            pending: AtomicUsize::new(0),
        }
    }

    /// Returns the total number of pending requests
    pub fn pending(&self) -> usize {
        self.pending.load(Ordering::SeqCst)
    }

    /// Queue the request for key `K`. Returns [`RequestType::New`] if
    /// no other requests for the same key are pending and [`RequestType::Pending`]
    /// if there are pending requests. Both [`RequestType`] values contain a [`async_std::channel::Receiver`]
    /// that can be listened to for lookup completion. Lookup completion
    /// can be signaled by [`LookupHandler::complete()`]
    pub async fn queue(&self, key: &K) -> RequestType<V, E> {
        let mut pending = self.map.lock().unwrap();
        let (sender, receiver) = oneshot::<LookupResult<V, E>>();

        if let Some(list) = pending.get_mut(key) {
            list.push(sender);
            RequestType::Pending(receiver)
        } else {
            pending.insert(key.clone(), vec![sender]);
            self.pending.fetch_add(1, Ordering::Relaxed);
            RequestType::New(receiver)
        }
    }

    /// Signal the lookup completion for key `K` by supplying a [`LookupResult`]
    /// with a resulting value `V` or an error `E`.
    pub async fn complete(&self, key: &K, result: LookupResult<V, E>) {
        let list = { self.map.lock().unwrap().remove(key) };

        if let Some(list) = list {
            self.pending.fetch_sub(1, Ordering::Relaxed);
            for sender in list {
                sender
                    .send(result.clone())
                    .await
                    .expect("Unable to complete lookup result");
            }
        } else {
            panic!("Lookup handler failure while processing key {key:?}")
        }
    }
}

#[cfg(not(target_os = "solana"))]
#[cfg(any(test, feature = "test"))]
mod tests {
    use super::LookupHandler;
    use super::RequestType;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::PoisonError;
    use std::time::Duration;

    use crate::task::sleep;
    use futures::join;
    use std::collections::HashMap;
    use workflow_core::channel::RecvError;

    #[derive(thiserror::Error, Debug, Clone)]
    pub enum Error {
        #[error("{0}")]
        String(String),
    }

    impl<T> From<PoisonError<T>> for Error {
        fn from(_: PoisonError<T>) -> Self {
            Error::String("PoisonError".to_string())
        }
    }

    impl From<RecvError> for Error {
        fn from(_: RecvError) -> Self {
            Error::String("RecvError".to_string())
        }
    }

    type Result<T> = std::result::Result<T, Error>;

    #[derive(Debug, Eq, PartialEq)]
    enum RequestTypeTest {
        New = 0,
        Pending = 1,
    }

    struct LookupHandlerTest {
        pub lookup_handler: LookupHandler<u32, Option<u32>, Error>,
        pub map: Arc<Mutex<HashMap<u32, u32>>>,
        pub request_types: Arc<Mutex<Vec<RequestTypeTest>>>,
    }

    impl LookupHandlerTest {
        pub fn new() -> Self {
            Self {
                lookup_handler: LookupHandler::new(),
                map: Arc::new(Mutex::new(HashMap::new())),
                request_types: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn insert(self: &Arc<Self>, key: u32, value: u32) -> Result<()> {
            let mut map = self.map.lock()?;
            map.insert(key, value);
            Ok(())
        }

        pub async fn lookup_remote_impl(self: &Arc<Self>, key: &u32) -> Result<Option<u32>> {
            println!("[lh] lookup sleep...");
            sleep(Duration::from_millis(100)).await;
            println!("[lh] lookup wake...");
            let map = self.map.lock()?;
            Ok(map.get(&key).cloned())
        }

        pub async fn lookup_handler_request(self: &Arc<Self>, key: &u32) -> Result<Option<u32>> {
            let request_type = self.lookup_handler.queue(key).await;
            match request_type {
                RequestType::New(receiver) => {
                    self.request_types
                        .lock()
                        .unwrap()
                        .push(RequestTypeTest::New);
                    println!("[lh] new request");
                    let response = self.lookup_remote_impl(key).await;
                    println!("[lh] completing initial request");
                    self.lookup_handler.complete(key, response).await;
                    receiver.recv().await?
                }
                RequestType::Pending(receiver) => {
                    self.request_types
                        .lock()
                        .unwrap()
                        .push(RequestTypeTest::Pending);
                    println!("[lh] pending request");
                    receiver.recv().await?
                }
            }
        }
    }

    pub async fn lookup_handler_test() -> Result<()> {
        let lht = Arc::new(LookupHandlerTest::new());
        lht.insert(0xc0fee, 0xdecaf)?;

        let v0 = lht.lookup_handler_request(&0xc0fee);
        let v1 = lht.lookup_handler_request(&0xc0fee);
        let v2 = lht.lookup_handler_request(&0xc0fee);
        let f = join!(v0, v1, v2);

        println!("[lh] results: {:?}", f);
        let f = (
            f.0.unwrap().unwrap(),
            f.1.unwrap().unwrap(),
            f.2.unwrap().unwrap(),
        );
        assert_eq!(f, (0xdecaf, 0xdecaf, 0xdecaf));

        let request_types = lht.request_types.lock().unwrap();
        println!("[lh] request types: {:?}", request_types);
        assert_eq!(
            request_types[..],
            [
                RequestTypeTest::New,
                RequestTypeTest::Pending,
                RequestTypeTest::Pending
            ]
        );
        println!("all looks good ... 😎");

        Ok(())
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "solana")))]
    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        pub async fn lookup_handler_test() -> Result<()> {
            super::lookup_handler_test().await
        }
    }
}
