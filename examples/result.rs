#![allow(dead_code)]

#[tokio::main]
async fn main() {
    let pipeline = fama::Pipeline::pass(0).await.ok(Adder).await;
    println!(
        "result 1: {:#?}",
        pipeline.deliver_as::<Result<i32, ()>>().await
    );

    let pipeline = fama::Pipeline::pass(10).await.ok(Adder).await;
    println!(
        "result 2: {:#?}",
        pipeline.deliver_as::<Result<i32, ()>>().await
    );
}

struct Adder;

#[fama::async_trait]
impl fama::FamaPipe<i32, Result<i32, ()>> for Adder {
    async fn receive_pipe_content(&self, number: i32) -> Result<i32, ()> {
        if number > 0 {
            Ok(number + 30)
        } else {
            Err(())
        }
    }
}
