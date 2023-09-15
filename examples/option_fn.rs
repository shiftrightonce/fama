#![allow(dead_code)]

#[tokio::main]
async fn main() {
    let pipeline = fama::Pipeline::pass(0)
        .store_fn(|n: i32| async move { Some(n + 21) })
        .await
        .some_fn(|n: Option<i32>| async move {
            if n.is_some() && n.unwrap() > 20 {
                Some(true)
            } else {
                None
            }
        })
        .await;

    println!("total: {:#?}", pipeline.deliver_as::<Option<i32>>());
    println!(
        "total is above 20 ?: {:#?}",
        pipeline.deliver_as::<Option<bool>>()
    );
}
