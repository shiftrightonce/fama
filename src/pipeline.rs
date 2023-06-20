use crate::content::PipeContent;
use async_trait::async_trait;
use std::{future::Future, marker::PhantomData};

/// The pipes manager
pub struct Pipeline<T: Send + Sync + 'static> {
    pipes: Vec<Box<dyn FamaPipe>>,
    fluid: PipeContent,
    phantom: PhantomData<T>,
    went_through: bool,
}

impl<T: Send + Sync + 'static> Pipeline<T> {
    /// Accepts the pipeline content/input.
    /// This is the beginning of the pipeline
    pub fn pass(fluid: T) -> Self {
        Self {
            pipes: Vec::new(),
            fluid: PipeContent::new(Box::new(fluid)),
            phantom: PhantomData,
            went_through: false,
        }
    }

    /// Adds a pipe to the pipeline. Call this method each time you want to add a pipe to the pipeline
    pub fn through<P: FamaPipe>(mut self, p: P) -> Self {
        self.pipes.push(Box::new(p));
        self
    }

    /// Starts flowing the content through the pipes
    pub async fn deliver(mut self) -> Box<T> {
        for a_pipe in self.pipes.iter() {
            self.fluid = a_pipe.receive_pipe_content(self.fluid).await;
            if self.fluid.1 {
                break;
            }
        }
        self.fluid.inner()
    }
}

/// The trait that must be implemented by a pipe
///
/// `async_trait` makes it easy to implement this trait
///
///```rust
///#  #![allow(dead_code)]
///
///# #[tokio::main]
///# async fn main() {}
///
/// struct MyPipe;
///
/// #[fama::async_trait]
/// impl fama::FamaPipe for MyPipe {
///     async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
///         // Your logic here
///        
///        content
///    }
/// }
///
/// ```
///
#[async_trait]
pub trait FamaPipe: Send + Sync + 'static {
    /// Where a pipe logic resides
    async fn receive_pipe_content(&self, content: PipeContent) -> PipeContent;

    /// Wraps the type in a Box
    fn to_pipe(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
