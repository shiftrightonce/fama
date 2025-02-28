use fama::PipelineBuilderTrait;

use crate::crate_a::CreateUser;

pub async fn setup() {
    // Get the builder for the "create user pipeline"
    let builder = CreateUser::pipeline_builder().await;

    // Add your pipes. In this case we are adding a "Content Manager" role to the list
    // of roles
    builder
        .register(|p| {
            // We need to return a pin box because at this stage
            // our function is yet to be called
            // Note: the "async" block
            Box::pin(async {
                // None: we are returning the pipeline instance
                p.store_fn(|mut user: CreateUser| async {
                    user.roles.push("Content Manager".to_owned());
                    user
                })
                .await
                // Chain as many pipes as you like
            })
        })
        .await;
}
