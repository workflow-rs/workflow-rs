use crate::imports::*;

pub type ServiceResult = Result<()>;

/// Service is a core component of the Kaspa NG application responsible for
/// running application services and communication between these services.
#[async_trait]
pub trait Service: Sync + Send {
    fn name(&self) -> &'static str;

    /// Start the service
    async fn spawn(self: Arc<Self>, runtime: Runtime) -> ServiceResult;

    /// Signal the service termination (post a shutdown request)
    fn terminate(self: Arc<Self>);

    /// Block until the service is terminated
    async fn join(self: Arc<Self>) -> ServiceResult;
}
