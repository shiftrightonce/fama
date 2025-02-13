use fama::{PipelineBuilder, PipelineBuilderTrait};

#[tokio::main]
async fn main() {
    // Get the builder instance for the "NewUser" type
    let builder = NewUser::pipeline_builder().await;

    // Register a pipe/function
    builder
        .register(|line| {
            // Note: Box::pin because this function will be called in the "future"
            Box::pin(async {
                line.next_fn(|| async {
                    println!(">> pipe 1 was called");
                    true
                })
                .await
            })
        })
        .await;

    // You can get the builder as many times as you like
    let builder = NewUser::pipeline_builder().await;

    builder
        .register(|p| {
            Box::pin(async {
                p.next_fn(|| async {
                    println!(">> pipe 2 was called");
                    true
                })
                .await
                .next_fn(|| async {
                    println!(">> I am the last pipe");
                    true
                })
                .await
            })
        })
        .await;

    // calling "pipeline" on the type instance kick off the flow
    let result = NewUser::default().pipeline().await.deliver().await;

    println!("result: {:#?}", result);
}

#[derive(Debug, Clone, Default)]
struct NewUser {
    id: Option<String>,
    role: Option<Vec<String>>,
}

#[fama::async_trait]
impl busybody::Injectable for NewUser {
    async fn inject(c: &busybody::ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[fama::async_trait]
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
