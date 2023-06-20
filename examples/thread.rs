#![allow(dead_code)]

use std::thread;

#[tokio::main]
async fn main() {
    let handle = thread::spawn(move || async {
        for id in 1..=2 {
            let n = fama::Pipeline::pass(id).through_fn(add_one).deliver().await;

            println!("thread 2 number {} + 1 and doubled {}", id, n);
        }
    });
    for id in 1..=2 {
        let n = fama::Pipeline::pass(id)
            .through_fn(add_one)
            .through(DoubleNumber)
            .deliver()
            .await;

        println!("thread 1 number {} + 1 and doubled {}", id, n);
    }

    _ = handle.join().unwrap().await;
}

struct DoubleNumber;

#[fama::async_trait]
impl fama::FamaPipe for DoubleNumber {
    async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
        let number = content.inner_ref::<i32>().unwrap();
        content.set_inner(2 * number);
        content
    }
}

async fn add_one(mut content: fama::PipeContent) -> fama::PipeContent {
    let number = content.inner_mut::<i32>().unwrap();
    *number += 1;
    content
}
