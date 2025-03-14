#![allow(dead_code)]
use fama::PipeContent;

#[tokio::main]
async fn main() {
    type AnswerResult = Result<String, String>;
    let counter = 0;
    let answer = fama::Pipeline::pass(counter)
        .await
        .through_fn(|mut c: i32, pipe: PipeContent| async move {
            c += 1;
            pipe.container().set_type(c).await;
            Some(pipe)
        })
        .await
        .through_fn(|mut c: i32, pipe: PipeContent| async move {
            c *= 20;
            pipe.container().set_type(c).await;
            Some(pipe)
        })
        .await
        .through_fn(|c: i32, pipe: PipeContent| async move {
            // Change counter to a string
            pipe.container().set_type(c.to_string()).await;
            pipe.container()
                .set_type(Ok::<String, String>(c.to_string()))
                .await;
            Some(pipe)
        })
        .await
        .deliver_as::<AnswerResult>()
        .await; // request that a string be returned

    println!("example one answer is: {:?}", answer);

    // request that a string be returned
    let answer: String = fama::Pipeline::pass(counter)
        .await
        .through_fn(|mut c: i32, pipe: PipeContent| async move {
            c += 13;
            pipe.container().set_type(c).await;
            Some(pipe)
        })
        .await
        .through_fn(|mut c: i32, pipe: PipeContent| async move {
            c *= 80;
            pipe.container().set_type(c).await;
            Some(pipe)
        })
        .await
        .through_fn(|c: i32, pipe: PipeContent| async move {
            // Change counter to a string
            // the container now has an i32 and string with the value 1040
            pipe.container().set_type(c.to_string()).await;
            Some(pipe)
        })
        .await
        .deliver_as()
        .await; // Note: we are using "deliver_as" instead of "deliver"

    println!("example two answer is: {:?}", answer);
}
