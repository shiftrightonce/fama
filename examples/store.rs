#![allow(dead_code)]

#[tokio::main]
async fn main() {
    let pipeline = fama::Pipeline::pass(0)
        .store_fn(|n: i32| async move { n + 20 })
        .await
        .store_fn(|n: i32| async move { n > 20 })
        .await;

    println!("total: {}", pipeline.deliver_as::<i32>());
    println!("total is above 20 ?: {}", pipeline.deliver_as::<bool>());
}
