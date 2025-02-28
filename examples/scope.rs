#![allow(dead_code)]
use busybody::helpers::service_container;
use fama::PipeContent;

#[tokio::main]
async fn main() {
    service_container().set_type(Config(100)).await; // Config in the "global" scope has the value 100

    // 1. Create a pipeline
    let score = fama::Pipeline::pass(500)
        .await
        .through_fn(|content: PipeContent| async move {
            content.container().set_type(Config(250)).await; // In this pipeline scope, the instance of config has the value 250
        })
        .await
        .through_fn(|config: Config, count: i32| async move {
            // There is an instance of an i32 type in this pipeline scope.
            // The value that was "pass".
            println!("count value: {count}");

            // This instance of config will have the value 250
            println!("pipe 'local' config: {:#?}", config);
        })
        .await
        .deliver()
        .await; // Start of the pipeline

    println!("score: {:#?}", score);
    println!(
        "'global' config value: {:#?}",
        service_container().get_type::<Config>().await.unwrap()
    );
}

#[derive(Debug, Clone)]
struct Config(i32);

impl Default for Config {
    fn default() -> Self {
        Self(44)
    }
}
