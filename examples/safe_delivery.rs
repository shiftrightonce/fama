#![allow(dead_code)]

#[tokio::main]
async fn main() {
    let pipeline = fama::Pipeline::pass(0)
        .store_fn(|n: i32| async move { n + 20 })
        .await
        .store_fn(|n: i32| async move { n > 20 })
        .await;

    println!("total: {:#?}", pipeline.try_deliver_as::<i32>());
    println!(
        "total is above 20 ?: {:#?}",
        pipeline.try_deliver_as::<bool>()
    );
    println!("person: {:#?}", pipeline.try_deliver_as::<Person>());
}

#[derive(Debug, Clone)]
struct Person {
    id: i32,
}
