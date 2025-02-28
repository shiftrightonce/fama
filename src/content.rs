use busybody::ServiceContainer;
use std::sync::Arc;

#[derive(Clone)]
pub struct PipeContent(pub(crate) Arc<ServiceContainer>);

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum PipeState {
    Run,
    Stop,
}

#[busybody::async_trait]
impl busybody::Resolver for PipeContent {
    async fn resolve(c: &ServiceContainer) -> Self {
        if let Some(instance) = c.get_type::<Self>().await {
            return instance;
        }
        Self::make().await
    }
}

impl PipeContent {
    pub async fn make() -> Self {
        let container = Arc::new(ServiceContainer::proxy());
        let pipe = Self(container);
        pipe.container().set(PipeState::Run).await;
        pipe.container().set_type(pipe.clone()).await;
        pipe
    }
    pub async fn new<T>(content: T) -> Self
    where
        T: Clone + Send + Sync + 'static,
    {
        let pipe = Self::make().await;
        pipe.container().set_type(content).await;

        pipe
    }
    /// Returns a busybody's ServiceContainer
    pub fn container(&self) -> &Arc<ServiceContainer> {
        &self.0
    }

    pub async fn store<T: Clone + Send + Sync + 'static>(&self, data: T) -> &Self {
        self.container().set_type(data).await;
        self
    }

    /// Notify the pipeline to stop flowing the content
    pub async fn stop_the_flow(&self) {
        self.container().set(PipeState::Stop).await;
    }

    /// Alias for `stop_the_flow`
    pub async fn stop(&self) {
        self.stop_the_flow().await;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn test_flow_is_running() {
        let pipe: PipeContent = PipeContent::make().await;

        assert_eq!(
            *pipe.container().get::<PipeState>().await.unwrap(),
            PipeState::Run,
            "The pipe state should have been 'run'"
        );
    }

    #[tokio::test]
    async fn test_flow_stop() {
        let pipe = PipeContent::make().await;
        pipe.stop().await;

        assert_eq!(
            *pipe.container().get::<PipeState>().await.unwrap(),
            PipeState::Stop
        );
    }
}
