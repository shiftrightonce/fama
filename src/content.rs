use busybody::ServiceContainer;
use std::sync::Arc;

#[derive(Clone)]
pub struct PipeContent(pub(crate) Arc<ServiceContainer>);

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum PipeState {
    Run,
    Stop,
}

impl Default for PipeContent {
    fn default() -> Self {
        let container = Arc::new(ServiceContainer::proxy());
        container.set(PipeState::Run);

        let pipe = Self(container);
        pipe.container().set_type(pipe.clone()).get_type().unwrap()
    }
}

#[busybody::async_trait]
impl busybody::Injectable for PipeContent {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().unwrap_or_default()
    }
}

impl PipeContent {
    pub fn new<T>(content: T) -> Self
    where
        T: Clone + Send + Sync + 'static,
    {
        let pipe = Self::default();
        pipe.container().set_type(content);
        pipe
    }
    /// Returns a busybody's ServiceContainer
    pub fn container(&self) -> &Arc<ServiceContainer> {
        &self.0
    }

    pub fn store<T: Clone + Send + Sync + 'static>(&self, data: T) -> &Self {
        self.container().set_type(data);
        self
    }

    /// Notify the pipeline to stop flowing the content
    pub fn stop_the_flow(&self) {
        self.container().set(PipeState::Stop);
    }

    /// Alias for `stop_the_flow`
    pub fn stop(&self) {
        self.stop_the_flow();
    }
}

#[cfg(test)]
mod test {
    use busybody::helpers::provide;

    use super::*;

    #[tokio::test]
    async fn test_flow_is_running() {
        let pipe: PipeContent = provide().await;

        assert_eq!(
            *pipe.container().get::<PipeState>().unwrap(),
            PipeState::Run,
            "The pipe state should have been 'run'"
        );
    }

    #[tokio::test]
    async fn test_flow_stop() {
        let pipe: PipeContent = provide().await;
        pipe.stop();

        assert_eq!(
            *pipe.container().get::<PipeState>().unwrap(),
            PipeState::Stop
        );
    }
}
