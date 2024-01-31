#![allow(dead_code)]

#[tokio::main]
async fn main() {
    let pipeline = fama::Pipeline::pass(0).some(Adder).await;
    // None will be returned if the requested is not in the container
    println!("result 1: {:#?}", pipeline.deliver_as::<Option<i32>>());

    let pipeline = fama::Pipeline::pass(10).some(Adder).await;
    println!("result 2: {:#?}", pipeline.deliver_as::<Option<i32>>());
}

struct Adder;

#[fama::async_trait]
impl fama::FamaPipe<i32, Option<i32>> for Adder {
    async fn receive_pipe_content(&self, number: i32) -> Option<i32> {
        if number > 0 {
            Some(number + 30)
        } else {
            None
        }
    }
}
