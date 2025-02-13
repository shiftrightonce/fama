#![allow(dead_code)]

use fama::PipeContent;

#[tokio::main]
async fn main() {
    let patron_age = 16;

    let pass: ClubPass = fama::Pipeline::pass(patron_age)
        .await
        // 1. Returning false will halt the pipe flow
        .next_fn(|age: i32, pipe: PipeContent| async move {
            if age < 18 {
                pipe.store(ClubPass { age, id: 0 }).await;
                return false;
            }
            true
        })
        .await
        .store_fn(|age: i32| async move { ClubPass { age, id: 56455 } })
        .await
        .deliver_as()
        .await;

    println!("Patron is under 18: {:#?}", pass);

    let patron_age = 34;

    let pass: ClubPass = fama::Pipeline::pass(patron_age)
        .await
        .next_fn(|age: i32, pipe: PipeContent| async move {
            if age < 18 {
                pipe.store(ClubPass { age, id: 0 }).await;
                return false;
            }
            true
        })
        .await
        .store_fn(|age: i32| async move { ClubPass { age, id: 56455 } })
        .await
        .deliver_as()
        .await;

    println!("Patron is over 18: {:#?}", pass);
}

#[derive(Debug, Clone)]
struct ClubPass {
    age: i32,
    id: i32,
}
