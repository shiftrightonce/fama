#![allow(dead_code)]

use fama::PipeContent;
use std::thread;

#[tokio::main]
async fn main() {
    let handle = thread::spawn(move || async {
        for id in 1..=2 {
            let n = fama::Pipeline::pass(id).through_fn(add_one).await.deliver();

            println!("thread 2 number {} + 1 and doubled {}", id, n);
        }
    });
    for id in 1..=2 {
        let n = fama::Pipeline::pass(id)
            .through_fn(add_one)
            .await
            .through(DoubleNumber)
            .await
            .deliver();

        println!("thread 1 number {} + 1 and doubled {}", id, n);
    }

    _ = handle.join().unwrap().await;
}

struct DoubleNumber;

#[fama::async_trait]
impl fama::FamaPipe<(i32, PipeContent)> for DoubleNumber {
    async fn receive_pipe_content(
        &self,
        (number, content): (i32, PipeContent),
    ) -> Option<fama::PipeContent> {
        content.store(2 * number);

        None
    }
}

async fn add_one(mut number: i32, content: fama::PipeContent) -> Option<fama::PipeContent> {
    number += 1;
    content.store(number);

    None
}
