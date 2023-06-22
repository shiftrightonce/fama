use async_trait::async_trait;
use busybody::ServiceContainer;
use std::{future::Future, marker::PhantomData, sync::Arc};

use crate::PipeContent;

/// The pipes manager
pub struct Pipeline<T: Send + Sync + 'static> {
    fluid: PipeContent,
    phantom: PhantomData<T>,
    container: Arc<ServiceContainer>,
    went_through: bool,
}

impl<T: Clone + Send + Sync + 'static> Pipeline<T> {
    /// Accepts the pipeline content/input.
    /// This is the beginning of the pipeline
    pub fn pass(content: T) -> Self {
        let mut fluid = PipeContent::default();

        let container = Arc::new(ServiceContainer::proxy());
        container.set_type(content);

        fluid.1 = Some(container.clone());
        container.set_type(fluid.clone());

        Self {
            fluid,
            container,
            phantom: PhantomData,
            went_through: false,
        }
    }

    /// Accepts a closure or function as a pipe.
    /// The closure can accept zero or more arguements.
    /// Unlike a struct pipe, a closure does not have to use a tuple
    /// for multiple arguments. Arguments can be up to 17
    pub async fn through_fn<P, Args>(mut self, pipe: P) -> Self
    where
        P: PipeFn<Args>,
        Args: busybody::Injectable + 'static,
    {
        if !self.fluid.0 {
            let args = Args::inject(&self.container).await;
            if let Some(f) = pipe.call(args).await {
                self.fluid = f;
            }
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    /// Accepts an instance of a struct that implements `fama::FamaPipe`
    pub async fn through<P, Args>(mut self, pipe: P) -> Self
    where
        P: FamaPipe<Args>,
        Args: busybody::Injectable + 'static,
    {
        if !self.fluid.0 {
            let args = Args::inject(&self.container).await;
            if let Some(f) = pipe.receive_pipe_content(args).await {
                self.fluid = f
            }
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    /// Returns the passed variable
    pub fn deliver(self) -> T {
        self.container.get_type().unwrap()
    }

    /// Returns true if the content went through all the registered pipes
    pub fn confirm(self) -> bool {
        self.went_through
    }
}

#[async_trait]
pub trait FamaPipe<Args = ()> {
    /// Where a pipe logic resides
    async fn receive_pipe_content(&self, args: Args) -> Option<PipeContent>;

    /// Wraps the type in a Box
    fn to_pipe(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub trait PipeFn<Args>: Send + Sync + 'static {
    type Future: Future<Output = Option<PipeContent>> + Send;

    fn call(&self, args: Args) -> Self::Future;
}

impl<Func, Fut> PipeFn<()> for Func
where
    Func: Send + Sync + Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<PipeContent>> + Send,
{
    type Future = Fut;
    fn call(&self, _: ()) -> Self::Future {
        (self)()
    }
}

impl<Func, Arg1, Fut> PipeFn<(Arg1,)> for Func
where
    Func: Send + Sync + Fn(Arg1) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<PipeContent>> + Send,
{
    type Future = Fut;
    fn call(&self, (c,): (Arg1,)) -> Self::Future {
        (self)(c)
    }
}

macro_rules! pipe_func{
    ($($T: ident),*) => {
        impl<Func, $($T),+, Fut> PipeFn <($($T),+)> for Func
         where Func: Fn($($T),+) -> Fut + Send + Sync + 'static,
         Fut: Future<Output = Option<PipeContent>> + Send,
        {
            type Future = Fut;

            #[allow(non_snake_case)]
            fn call(&self, ($($T),+): ($($T),+)) -> Self::Future {
                (self)($($T),+)
            }
        }
    };
}

pipe_func! {Arg1, Arg2}
pipe_func! {Arg1, Arg2, Arg3}
pipe_func! {Arg1, Arg2, Arg3, Arg4}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16}
pipe_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17}
