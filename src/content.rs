#![allow(dead_code)]
use std::any::Any;

/// The type that is pass to "receive_pipe_content"
#[derive(Debug)]
pub struct PipeContent(Box<dyn Any + Send + Sync + 'static>, pub(crate) bool);

impl PipeContent {
    /// Creates a new instance from the supplied type instance
    pub fn new<T: Send + Sync + 'static>(inner: T) -> Self {
        Self::from(Box::new(inner))
    }

    pub(crate) fn from(inner: Box<dyn Any + Send + Sync + 'static>) -> Self {
        Self(inner, false)
    }

    /// Returns a ref mut of the inner content
    pub fn inner_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }

    /// Returns the inner ref content
    pub fn inner_ref<T: 'static>(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    pub(crate) fn inner<T: 'static>(self) -> Box<T> {
        self.0.downcast::<T>().unwrap()
    }

    /// Replace the inner content with this new version
    pub fn set_inner<T: Send + Sync + 'static>(&mut self, inner: T) -> &mut Self {
        self.0 = Box::new(inner);
        self
    }

    /// Notify the pipeline to stop flowing the content
    pub fn stop_the_flow(&mut self) {
        self.1 = true;
    }
}
