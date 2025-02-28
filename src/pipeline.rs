use async_trait::async_trait;
use busybody::Resolver;
use futures::future::Future;
use std::marker::PhantomData;

use crate::{PipeContent, content::PipeState};

/// The pipes manager
#[derive(Clone)]
pub struct Pipeline<T: Send + Sync + 'static> {
    phantom: PhantomData<T>,
    pipe_content: PipeContent,
    went_through: bool,
}

impl<T: Clone + Send + Sync + 'static> Pipeline<T> {
    /// Accepts the pipeline content/input.
    /// This is the beginning of the pipeline
    pub async fn pass(content: T) -> Self {
        let pipe_content = PipeContent::new(content).await;

        Self {
            pipe_content,
            phantom: PhantomData,
            went_through: false,
        }
    }

    pub async fn pass_content(self, content: T) -> Self {
        self.container().set_type(content).await;
        self
    }

    /// Accepts a closure or function as a pipe.
    /// The closure can accept zero or more arguments.
    /// Unlike a struct pipe, a closure does not have to use a tuple
    /// for multiple arguments. Arguments can be up to 17
    pub async fn through_fn<H, Args, O>(mut self, mut handler: H) -> Self
    where
        H: PipeFnHandler<Args, O>,
        Args: busybody::Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            handler.pipe_fn_handle(args).await;
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    /// Accepts a closure or function as a pipe.
    /// The closure can accept zero or more arguments.
    /// Unlike a struct pipe, a closure does not have to use a tuple
    /// for multiple arguments. Arguments can be up to 17
    /// Closure must return a boolean. `False` will stop the pipe flow
    pub async fn next_fn<H, Args>(mut self, mut handler: H) -> Self
    where
        H: PipeFnHandler<Args, bool>,
        Args: busybody::Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            if !handler.pipe_fn_handle(args).await {
                self.container().set(PipeState::Stop).await;
            }
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    /// Stores the result from the pipe handler
    pub async fn store_fn<H, Args, O: Clone + Send + Sync + 'static>(
        mut self,
        mut handler: H,
    ) -> Self
    where
        H: PipeFnHandler<Args, O>,
        Args: busybody::Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            self.container()
                .set_type(handler.pipe_fn_handle(args).await)
                .await;
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    // Stores Option<T> returned by the handler
    // If option is `none` the pipe flow is stopped
    pub async fn some_fn<H, Args, O: Clone + Send + Sync + 'static>(
        mut self,
        mut handler: H,
    ) -> Self
    where
        H: PipeFnHandler<Args, Option<O>>,
        Args: busybody::Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            let option = handler.pipe_fn_handle(args).await;

            if option.is_none() {
                self.container().set(PipeState::Stop).await;
            }

            self.container().set_type(option).await;
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    // Stores Result<T, E> returned by the handler
    // If result is `err` the pipe flow is stopped
    pub async fn ok_fn<
        H,
        Args,
        O: Clone + Send + Sync + 'static,
        E: Clone + Send + Sync + 'static,
    >(
        mut self,
        mut handler: H,
    ) -> Self
    where
        H: PipeFnHandler<Args, Result<O, E>>,
        Args: busybody::Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            let result = handler.pipe_fn_handle(args).await;

            if result.is_err() {
                self.container().set(PipeState::Stop).await;
            }

            self.container().set_type(result).await;
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    /// Accepts an instance of a struct that implements `fama::FamaPipe`
    /// The returned result will be store for the next pipe handlers
    pub async fn through<H, Args, O>(mut self, handler: H) -> Self
    where
        H: FamaPipe<Args, O>,
        Args: busybody::Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            handler.receive_pipe_content(args).await;
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    /// Accepts an instance of a struct that implements `fama::FamaPipe`
    /// Must return a boolean. `False` will halt the flow
    pub async fn next<H, Args>(mut self, handler: H) -> Self
    where
        H: FamaPipe<Args, bool>,
        Args: busybody::Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            if !handler.receive_pipe_content(args).await {
                self.container().set(PipeState::Stop).await;
            }
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }
    pub async fn store<H, Args, O: Clone + Send + Sync + 'static>(mut self, handler: H) -> Self
    where
        H: FamaPipe<Args, O>,
        Args: busybody::Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            self.container()
                .set_type(handler.receive_pipe_content(args).await)
                .await;
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    // Stores Option<T> returned by the handler
    // If option is `none` the pipe flow is stopped
    pub async fn some<H, Args, O: Clone + Send + Sync + 'static>(mut self, handler: H) -> Self
    where
        H: FamaPipe<Args, Option<O>>,
        Args: Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            let option = handler.receive_pipe_content(args).await;

            if option.is_none() {
                self.container().set(PipeState::Stop).await;
            }

            self.container().set_type(option).await;
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    // Stores Result<T, E> returned by the handler
    // If result is `err` the pipe flow is stopped
    pub async fn ok<H, Args, O: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static>(
        mut self,
        handler: H,
    ) -> Self
    where
        H: FamaPipe<Args, Result<O, E>>,
        Args: busybody::Resolver + 'static,
    {
        if *self.container().get::<PipeState>().await.unwrap() == PipeState::Run {
            let args = Args::resolve(self.container()).await;
            let result = handler.receive_pipe_content(args).await;

            if result.is_err() {
                self.container().set(PipeState::Stop).await;
            }

            self.container().set_type(result).await;
            self.went_through = true;
        } else {
            self.went_through = false;
        }

        self
    }

    /// Returns the passed variable
    pub async fn deliver(&self) -> T {
        self.try_to_deliver().await.unwrap()
    }

    /// Returns the passed variable wrapped in an `Option<T>`
    pub async fn try_to_deliver(&self) -> Option<T> {
        self.container().get_type().await
    }

    /// Returns a different type that may have been set
    /// by one of the pipes
    pub async fn deliver_as<R: Clone + 'static>(&self) -> R {
        self.try_deliver_as().await.unwrap()
    }

    /// Returns a different type that may have been set
    /// by one of the pipes. The returned type will be wrapped
    /// in an `Option<T>`
    pub async fn try_deliver_as<R: Clone + 'static>(&self) -> Option<R> {
        self.container().get_type().await
    }

    /// Returns true if the content went through all the registered pipes
    pub fn confirm(&self) -> bool {
        self.went_through
    }

    fn container(&self) -> &busybody::ServiceContainer {
        self.pipe_content.container()
    }
}

#[async_trait]
pub trait FamaPipe<Args, O> {
    /// Where a pipe logic resides
    async fn receive_pipe_content(&self, args: Args) -> O;

    /// Wraps the type in a Box
    fn to_pipe(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub trait PipeFnHandler<Args, O>: Send + Sync + 'static {
    type Future: Future<Output = O> + Send;

    fn pipe_fn_handle(&mut self, args: Args) -> Self::Future;
}

impl<Func, Fut, O> PipeFnHandler<(), O> for Func
where
    Func: Send + Sync + FnMut() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = O> + Send,
{
    type Future = Fut;
    fn pipe_fn_handle(&mut self, _: ()) -> Self::Future {
        (self)()
    }
}

impl<Func, Arg1, Fut, O> PipeFnHandler<(Arg1,), O> for Func
where
    Func: Send + Sync + FnMut(Arg1) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = O> + Send,
{
    type Future = Fut;
    fn pipe_fn_handle(&mut self, (c,): (Arg1,)) -> Self::Future {
        (self)(c)
    }
}

macro_rules! pipe_func{
    ($($T: ident),*) => {
        impl<Func, $($T),+, Fut, O> PipeFnHandler <($($T),+), O> for Func
         where Func: FnMut($($T),+) -> Fut + Send + Sync + 'static,
         Fut: Future<Output = O> + Send,
        {
            type Future = Fut;

            #[allow(non_snake_case)]
            fn pipe_fn_handle(&mut self, ($($T),+): ($($T),+)) -> Self::Future {
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

#[cfg(test)]
mod test {
    use super::*;

    struct AddOne;
    #[async_trait]
    impl FamaPipe<i32, i32> for AddOne {
        async fn receive_pipe_content(&self, num: i32) -> i32 {
            num + 1
        }
    }

    struct AddTwo;
    #[async_trait]
    impl FamaPipe<i32, i32> for AddTwo {
        async fn receive_pipe_content(&self, num: i32) -> i32 {
            num + 2
        }
    }

    struct StoreAddOne;
    #[async_trait]
    impl FamaPipe<(i32, PipeContent), ()> for StoreAddOne {
        async fn receive_pipe_content(&self, (num, pipe): (i32, PipeContent)) {
            pipe.store(num + 1).await;
        }
    }

    struct StoreAddTwo;
    #[async_trait]
    impl FamaPipe<(i32, PipeContent), ()> for StoreAddTwo {
        async fn receive_pipe_content(&self, (num, pipe): (i32, PipeContent)) {
            pipe.store(num + 2).await;
        }
    }

    struct ValidateCount;
    #[async_trait]
    impl FamaPipe<i32, bool> for ValidateCount {
        async fn receive_pipe_content(&self, num: i32) -> bool {
            num >= 6
        }
    }

    #[tokio::test]
    async fn test_through() {
        let result = Pipeline::pass(0)
            .await
            .through(StoreAddOne)
            .await
            .through(StoreAddOne)
            .await
            .through(StoreAddTwo)
            .await
            .through(StoreAddTwo)
            .await
            .deliver()
            .await;

        assert_eq!(result, 6);
    }

    #[tokio::test]
    async fn test_store() {
        let result = Pipeline::pass(0)
            .await
            .store(AddOne)
            .await
            .store(AddOne)
            .await
            .store(AddTwo)
            .await
            .store(AddTwo)
            .await
            .deliver()
            .await;

        assert_eq!(result, 6);
    }

    #[tokio::test]
    async fn test_next() {
        let result = Pipeline::pass(0)
            .await
            .store(AddOne)
            .await
            .store(AddOne)
            .await
            .store(AddTwo)
            .await
            .next(ValidateCount)
            .await
            .store(AddTwo)
            .await
            .deliver()
            .await;

        assert_eq!(result, 4);

        let result = Pipeline::pass(0)
            .await
            .store(AddOne)
            .await
            .store(AddOne)
            .await
            .store(AddTwo)
            .await
            .store(AddTwo)
            .await
            .store(AddTwo)
            .await
            .next(ValidateCount)
            .await
            .store(AddTwo)
            .await
            .deliver()
            .await;

        assert_eq!(result, 10);
    }

    #[tokio::test]
    async fn test_through_fn1() {
        let result: bool = Pipeline::pass(33)
            .await
            .through_fn(|num: i32, pipe: PipeContent| async move {
                pipe.store(num + 2).await;
            })
            .await
            .through_fn(|num: i32, pipe: PipeContent| async move {
                pipe.store(num == 35).await;
            })
            .await
            .deliver_as()
            .await;

        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn test_next_fn() {
        let result: bool = Pipeline::pass(33)
            .await
            .next_fn(|num: i32, pipe: PipeContent| async move {
                pipe.store(num + 2).await;
                true
            })
            .await
            .next_fn(|num: i32| async move { num != 35 })
            .await
            .next_fn(|num: i32| async move { num == 35 })
            .await
            .confirm();

        assert_eq!(result, false);
    }

    #[tokio::test]
    async fn test_store_fn() {
        let total = Pipeline::pass(0)
            .await
            .store_fn(|num: i32| async move { num + 1 })
            .await
            .store_fn(|num: i32| async move { num + 4 })
            .await
            .store_fn(|num: i32| async move { num * 5 })
            .await
            .deliver()
            .await;

        assert_eq!(total, 25);
    }

    #[tokio::test]
    async fn test_store2_fn() {
        let total = Pipeline::<i32>::pass(0)
            .await
            .store_fn(|num: i32| async move { num + 1 })
            .await
            .store_fn(|num: i32| async move { num + 4 })
            .await
            .store_fn(|num: i32| async move { num * 5 })
            .await
            .deliver()
            .await;

        assert_eq!(total, 25);
    }

    #[tokio::test]
    async fn test_some_flow_fn() {
        let result1 = Pipeline::pass(0)
            .await
            .some_fn(|n: i32| async move { if n > 10 { Some(n) } else { None } })
            .await
            .deliver_as::<Option<i32>>()
            .await;

        assert_eq!(result1.is_some(), false);

        let result2 = Pipeline::pass(100)
            .await
            .some_fn(|n: i32| async move { if n > 10 { Some(n) } else { None } })
            .await
            .deliver_as::<Option<i32>>()
            .await;

        assert_eq!(result2.is_some(), true);
    }

    #[tokio::test]
    async fn test_some_flow() {
        struct SomeI32;
        #[async_trait::async_trait]
        impl FamaPipe<i32, Option<i32>> for SomeI32 {
            async fn receive_pipe_content(&self, n: i32) -> Option<i32> {
                if n > 10 { Some(n) } else { None }
            }
        }
        let result1 = Pipeline::pass(0)
            .await
            .some(SomeI32)
            .await
            .deliver_as::<Option<i32>>()
            .await;

        assert_eq!(result1.is_some(), false);

        let result2 = Pipeline::pass(100)
            .await
            .some(SomeI32)
            .await
            .deliver_as::<Option<i32>>()
            .await;

        assert_eq!(result2.is_some(), true);
    }

    #[tokio::test]
    async fn test_result_flow_fn() {
        let result1 = Pipeline::pass(0)
            .await
            .ok_fn(|n: i32| async move { if n > 10 { Ok::<i32, ()>(n) } else { Err(()) } })
            .await
            .deliver_as::<Result<i32, ()>>()
            .await;

        assert_eq!(result1.is_err(), true);

        let result2 = Pipeline::pass(100)
            .await
            .ok_fn(|n: i32| async move { if n > 10 { Ok::<i32, ()>(n) } else { Err(()) } })
            .await
            .deliver_as::<Result<i32, ()>>()
            .await;

        assert_eq!(result2.is_ok(), true);
    }

    #[tokio::test]
    async fn test_result_flow() {
        struct SomeI32;
        #[async_trait::async_trait]
        impl FamaPipe<i32, Result<i32, ()>> for SomeI32 {
            async fn receive_pipe_content(&self, n: i32) -> Result<i32, ()> {
                if n > 10 { Ok(n) } else { Err(()) }
            }
        }
        let result1 = Pipeline::pass(0)
            .await
            .ok(SomeI32)
            .await
            .deliver_as::<Result<i32, ()>>()
            .await;

        assert_eq!(result1.is_err(), true);

        let result2 = Pipeline::pass(100)
            .await
            .ok(SomeI32)
            .await
            .deliver_as::<Result<i32, ()>>()
            .await;

        assert_eq!(result2.is_ok(), true);
    }

    #[tokio::test]
    async fn test_deliver_as() {
        let result: bool = Pipeline::pass(0)
            .await
            .store_fn(|num: i32| async move { num + 1 })
            .await
            .store_fn(|num: i32| async move { num + 4 })
            .await
            .store_fn(|num: i32| async move { num * 5 })
            .await
            .store_fn(|num: i32| async move { num == 25 })
            .await
            .deliver_as()
            .await;

        assert_eq!(result, true);
    }
}
