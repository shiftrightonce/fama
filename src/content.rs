use busybody::ServiceContainer;
use std::sync::Arc;

#[derive(Clone)]
pub struct PipeContent(pub(crate) bool, pub(crate) Option<Arc<ServiceContainer>>);

impl Default for PipeContent {
    fn default() -> Self {
        Self(false, None)
    }
}

#[busybody::async_trait]
impl busybody::Injectable for PipeContent {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().unwrap_or_else(|| Self::default())
    }
}

impl PipeContent {
    /// Returns a busybody's ServiceContainer
    pub fn container(&self) -> &Arc<ServiceContainer> {
        &self.1.as_ref().unwrap()
    }

    pub fn store<T: Clone + Send + Sync + 'static>(&self, data: T) -> &Self {
        self.container().set_type(data);
        &self
    }

    /// Notify the pipeline to stop flowing the content
    pub fn stop_the_flow(&mut self) {
        self.0 = true;
    }
}
