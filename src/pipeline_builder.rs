use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::RwLock;

use crate::{pipeline::PipeFnHandler, Pipeline};

type PipeList<T> = Arc<RwLock<Vec<fn(Pipeline<T>) -> BoxFuture<'static, Pipeline<T>>>>>;

/// PipelineBuilder provides flexibility and extensibility to your pipelines
///
/// Pipes/function can be appended to your type pipeline from other places in your code or even
/// across crates.
///
/// ```rust
///# #![allow(dead_code)]
///# use fama::PipelineBuilder;
///
/// #[derive(Default, Clone)]
/// struct MeaningOfLife(i32);
///
///
///
/// #[tokio::main]
/// async fn main() {
///
///    let builder = PipelineBuilder::<MeaningOfLife>::new();
///
///    builder.register(|pipeline| {
///       Box::pin(async {
///         pipeline.store_fn(|mut instance: MeaningOfLife| async {
///             instance.0 = 42;
///            instance
///         }).await
///      })
///    }).await;
///  
///    let life = builder.build(MeaningOfLife::default()).await.deliver().await;
///    assert_eq!(life.0, 42);
/// }
///
/// ```
/// You can implement the PipelineBuilderTrait for your type as well
///
///
/// ```rust
///# #![allow(dead_code)]
///# use fama::{PipelineBuilder, PipelineBuilderTrait};
///
/// #[derive(Default, Clone)]
/// struct MeaningOfLife(i32);
///
///
/// #[fama::async_trait]
/// impl PipelineBuilderTrait for MeaningOfLife {
///
///    // we are overriding the default implementation of this method in order
///    // to append our pipe
///    async fn setup_pipeline_builder(builder: PipelineBuilder<Self>) -> PipelineBuilder<Self> {
///       builder.register(|pipeline| {
///          Box::pin(async {
///               pipeline.store_fn(|mut instance: MeaningOfLife| async {
///                    instance.0 = 42;
///                    instance
///              }).await
///          })
///        }).await;
///
///        builder
///    }
///    
/// }
///
///
/// #[tokio::main]
/// async fn main() {
///
///    let new_life = MeaningOfLife(0);
///
///    // Register/append a pipe/function to the pipeline
///    MeaningOfLife::pipeline_builder().await
///      .register(|pipeline| {
///           Box::pin(async {
///              pipeline.store_fn(|mut instance: MeaningOfLife| async {
///                  if instance.0 == 0  {
///                      instance.0 = 42 ;
///                   } else {
///                      instance.0 = instance.0 * 2;
///                   }
///                   instance
///              }).await
///          })
///       }).await;
///  
///    let life = new_life.pipeline().await.deliver().await;
///    assert_eq!(life.0, 84);
/// }
///
/// ```
#[derive(Clone)]
pub struct PipelineBuilder<T: Clone + Send + Sync + 'static> {
    pipes: PipeList<T>,
}

impl<T: Clone + Send + Sync + 'static> PipelineBuilder<T> {
    pub fn new() -> Self {
        futures::executor::block_on(async {
            let (_, instance) = Self::initial().await;
            instance
        })
    }

    pub async fn initial() -> (bool, Self) {
        if let Some(instance) = busybody::helpers::service_container()
            .get_type::<Self>()
            .await
        {
            (false, instance)
        } else {
            let instance = Self {
                pipes: Arc::default(),
            };
            busybody::helpers::service_container()
                .set_type(instance.clone())
                .await;
            (true, instance)
        }
    }

    pub async fn register(
        &self,
        callback: fn(Pipeline<T>) -> BoxFuture<'static, Pipeline<T>>,
    ) -> &Self {
        let mut lock = self.pipes.write().await;
        lock.push(callback);

        self
    }

    pub async fn build(&self, content: T) -> Pipeline<T> {
        let mut pipeline = Pipeline::pass(content).await;
        let lock = self.pipes.read().await;
        for pipe in lock.iter() {
            pipeline = pipe.pipe_fn_handle((pipeline,)).await;
        }

        pipeline
    }
}

impl<T: Clone + Send + Sync + 'static> Default for PipelineBuilder<T> {
    fn default() -> Self {
        Self {
            pipes: Default::default(),
        }
    }
}

#[busybody::async_trait]
pub trait PipelineBuilderTrait: Clone + Send + Sync {
    /// Will be called the first time an instance of the builder is instantiated
    /// Use this method to prepend pipes
    async fn setup_pipeline_builder(builder: PipelineBuilder<Self>) -> PipelineBuilder<Self> {
        builder
    }

    /// Returns the pipe builder instance for this type
    async fn pipeline_builder() -> PipelineBuilder<Self> {
        let (is_initial, builder) = PipelineBuilder::<Self>::initial().await;
        if is_initial {
            return Self::setup_pipeline_builder(builder).await;
        }

        builder
    }

    /// Pass the current instance of this type through the pipeline
    async fn pipeline(self) -> Pipeline<Self> {
        Self::pipeline_builder().await.build(self).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Clone, Default)]
    struct NewUser {
        id: Option<String>,
        role: Option<Vec<String>>,
    }

    #[crate::async_trait]
    impl busybody::Resolver for NewUser {
        async fn resolve(c: &busybody::ServiceContainer) -> Self {
            c.get_type().await.unwrap_or_default()
        }
    }

    #[crate::async_trait]
    impl PipelineBuilderTrait for NewUser {
        // implementing this method is optional
        async fn setup_pipeline_builder(builder: PipelineBuilder<Self>) -> PipelineBuilder<Self> {
            builder
                .register(|pipeline| {
                    Box::pin(async {
                        pipeline
                            .store_fn(|mut user: NewUser| async {
                                user.id = Some(format!("USR-{}", 200));
                                user.role = Some(vec!["Teacher".to_string()]);

                                user
                            })
                            .await
                    })
                })
                .await;
            builder
        }
    }

    #[tokio::test]
    async fn test_multiple_instances() {
        let builder = PipelineBuilder::<NewUser>::new();
        let builder2 = NewUser::pipeline_builder().await;

        builder2
            .register(|pipeline| {
                Box::pin(async {
                    pipeline
                        .store_fn(|mut user: NewUser| async {
                            user.id = Some("changed".to_string());
                            user
                        })
                        .await
                })
            })
            .await;

        let user_a = builder.build(NewUser::default()).await.deliver().await;
        let user_b = NewUser::default().pipeline().await.deliver().await;

        assert_eq!(user_a.id, user_b.id);
    }
}
