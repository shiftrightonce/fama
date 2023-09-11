#![allow(dead_code)]
use busybody::{helpers::service_container, ServiceContainer};
use fama::PipeContent;

#[tokio::main]
async fn main() {
    service_container().set_type(Config(100)); // Config in the "global" scope has the value 100

    // 1. Create a pipeline
    let score = fama::Pipeline::pass(500)
        .through_fn(|content: PipeContent| async move {
            content.container().set_type(Config(250)); // In this pipeline scope, the instance of config has the value 250
        })
        .await
        .through_fn(|config: Config, count: i32| async move {
            // there is an instance of an I32 type in this pipeline scope.
            // The value that was "pass".
            dbg!(count);

            // This  instance of config will have the value 250
            dbg!(config);
        })
        .await
        .deliver(); // Start of the pipeline

    println!("score: {:#?}", score);
}

#[derive(Debug, Clone)]
struct Config(i32);

impl Default for Config {
    fn default() -> Self {
        Self(44)
    }
}

#[busybody::async_trait]
impl busybody::Injectable for Config {
    async fn inject(c: &ServiceContainer) -> Self {
        c.proxy_value().unwrap_or_default()
    }
}
