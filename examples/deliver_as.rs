#![allow(dead_code)]
use fama::PipeContent;

#[tokio::main]
async fn main() {
    type AnswerResult = Result<String, String>;
    let counter = 0;
    let answer = fama::Pipeline::pass(counter)
        .through_fn(|mut c: i32, pipe: PipeContent| async move {
            c += 1;
            pipe.container().set_type(c);
            Some(pipe)
        })
        .await
        .through_fn(|mut c: i32, pipe: PipeContent| async move {
            c *= 20;
            pipe.container().set_type(c);
            Some(pipe)
        })
        .await
        .through_fn(|c: i32, pipe: PipeContent| async move {
            // Change counter to a string
            pipe.container().set_type(c.to_string());
            pipe.container()
                .set_type(Ok::<String, String>(c.to_string()));
            Some(pipe)
        })
        .await
        .deliver_as::<AnswerResult>(); // request that a string be returned

    println!("example one answer is: {:?}", answer);

    // request that a string be returned
    let answer: String = fama::Pipeline::pass(counter)
        .through_fn(|mut c: i32, pipe: PipeContent| async move {
            c += 13;
            pipe.container().set_type(c);
            Some(pipe)
        })
        .await
        .through_fn(|mut c: i32, pipe: PipeContent| async move {
            c *= 80;
            pipe.container().set_type(c);
            Some(pipe)
        })
        .await
        .through_fn(|c: i32, pipe: PipeContent| async move {
            // Change counter to a string
            pipe.container().set_type(c.to_string());
            Some(pipe)
        })
        .await
        .deliver_as(); // Note: we are using "deliver_as" instead of "deliver"

    println!("example two answer is: {:?}", answer);
}
