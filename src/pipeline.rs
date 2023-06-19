use crate::content::PipeContent;
use async_trait::async_trait;
use std::marker::PhantomData;

pub struct Pipeline<T: Send + Sync + 'static> {
    pipes: Vec<Box<dyn FamaPipe>>,
    fluid: PipeContent,
    inner: PhantomData<T>,
}

impl<T: Send + Sync + 'static> Pipeline<T> {
    pub fn pass(fluid: T) -> Self {
        Self {
            pipes: Vec::new(),
            fluid: PipeContent::new(Box::new(fluid)),
            inner: PhantomData,
        }
    }

    pub fn through<P: FamaPipe>(mut self, p: P) -> Self {
        self.pipes.push(Box::new(p));
        self
    }

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

#[async_trait]
pub trait FamaPipe: Send + Sync + 'static {
    async fn receive_pipe_content(&self, content: PipeContent) -> PipeContent;

    fn to_pipe(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
