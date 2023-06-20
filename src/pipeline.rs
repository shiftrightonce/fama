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

    /// Adds a async function as a pipe
    pub fn through_fn<F: PipeFn>(mut self, f: F) -> Self {
        self.pipes.push(Box::new(WrapFn(f)));
        self
    }

    /// Starts flowing the content through the pipes
    pub async fn deliver(self) -> Box<T> {
        self.try_delivering().await.fluid.inner()
    }

    /// Starts flowing but return the specified type
    pub async fn deliver_as<R: Send + Sync + 'static>(self) -> Box<R> {
        self.try_delivering().await.fluid.inner()
    }

    /// Start flowing and return a confirmation if we got to the end
    pub async fn confirm(self) -> bool {
        self.try_delivering().await.went_through
    }

    async fn try_delivering(mut self) -> Self {
        self.went_through = true;
        for a_pipe in self.pipes.iter() {
            self.fluid = a_pipe.receive_pipe_content(self.fluid).await;
            if self.fluid.1 {
                self.went_through = false;
                break;
            }
        }

        self
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

struct WrapFn<T: PipeFn + Send + Sync>(T);

#[async_trait]
impl<T: PipeFn + Send + Sync> FamaPipe for WrapFn<T> {
    async fn receive_pipe_content(&self, content: PipeContent) -> PipeContent {
        self.0.call((content,)).await
    }
}

pub trait PipeFn: Send + Sync + 'static {
    type Future: Future<Output = PipeContent> + Send;
    fn call(&self, args: (PipeContent,)) -> Self::Future;
}

impl<Func, Fut> PipeFn for Func
where
    Func: Send + Sync + Fn(PipeContent) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = PipeContent> + Send,
{
    type Future = Fut;
    fn call(&self, (arg1,): (PipeContent,)) -> Self::Future {
        (self)(arg1)
    }
}
