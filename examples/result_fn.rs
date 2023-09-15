#![allow(dead_code)]

#[tokio::main]
async fn main() {
    let pipeline = fama::Pipeline::pass(0)
        .ok_fn(|n: i32| async move { Ok::<i32, ()>(n + 20) })
        .await
        .ok_fn(|n: Result<i32, ()>| async move {
            if n.is_ok() && n.unwrap() > 20 {
                Ok::<String, ()>("n > 20".into())
            } else {
                Err(())
            }
        })
        .await;

    println!(
        "i32 result: {:#?}",
        pipeline.deliver_as::<Result<i32, ()>>()
    );
    println!(
        "check i32 result: {:#?}",
        pipeline.deliver_as::<Result<String, ()>>()
    );
}
