use fama::PipelineBuilderTrait;

use crate::crate_a::CreateUser;
pub async fn setup() {
    // Get the builder for the "create user pipeline"
    let builder = CreateUser::pipeline_builder().await;

    // Add your pipes by passing a callback to the builder "register" method.
    // In this case we are pretending to do some "logging" here
    builder
        .register(|p| {
            // We need to return a pin box because at this stage
            // our function is yet to be called
            // Note: the "async" block
            Box::pin(async {
                // None: we are returning the pipeline instance
                p.through_fn(|user: CreateUser| async move {
                    println!(">>>>> logging >> creating user with ID: {:?}", &user.id);
                })
                .await
                // Chain as many pipes as you like
            })
        })
        .await;
}
