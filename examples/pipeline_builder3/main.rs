use fama::PipelineBuilderTrait;

mod crate_a;
mod crate_b;
mod crate_c;

#[tokio::main]
async fn main() {
    // mock example of setting up external crates in your application
    crate_c::setup().await;
    crate_b::setup().await;
    crate_a::setup().await;

    // Get the instance of "CreateUser" pipeline builder
    // CreateUser implements "fama::PipelineBuilderTrait"
    let builder = crate_a::CreateUser::pipeline_builder().await;

    // Create the content that will be passed through the pipeline
    let user = crate_a::CreateUser::default();

    // We are append a "pipe/function" using the CreateUser's PipelineBuilder
    let user = builder
        .register(|p| {
            Box::pin(async {
                p.next_fn(|user: crate_a::CreateUser| async move {
                    println!("saving user record to persistent storage: {:?}", &user.id);
                    true
                })
                .await
            })
        })
        .await
        .build(user) // Calling build on the builder will start the flow/process
        .await
        .deliver()
        .await;

    println!("user one created: {:#?}", user);

    // Another way to pass the content through the pipeline
    let mut second_user = crate_a::CreateUser::default();
    second_user.add_role("General Manager");

    let user = second_user.pipeline().await.deliver().await;
    println!("user two created: {:#?}", user);
}
