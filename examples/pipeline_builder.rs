#![allow(dead_code)]

use fama::PipelineBuilder;

#[tokio::main]
async fn main() {
    // Create a builder instance for the "NewUser" type
    let builder = PipelineBuilder::<NewUser>::new();

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

    // Creating a builder instance for the same type multiple times is allow.
    // All pipes will be applied to a single instance
    let builder = PipelineBuilder::<NewUser>::new();

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

    // Yet another instance of the pipeline builder for this type (NewUser)
    let builder = PipelineBuilder::<NewUser>::new();

    // calling "build" on the builder kick off the flow
    let result = builder.build(NewUser::default()).await.confirm();

    println!("result: {}", result);
}

#[derive(Debug, Clone, Default)]
struct NewUser {
    id: Option<String>,
    role: Option<Vec<String>>,
}
