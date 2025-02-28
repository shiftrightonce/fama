#![allow(dead_code)]
use fama::PipeContent;

#[tokio::main]
async fn main() {
    // 1. Create a pipeline
    let new_user = fama::Pipeline::pass(NewUser {
        username: Some("awesomeuser".into()),
        ..NewUser::default()
    }) // Start of the pipeline
    .await
    .through(ValidateUserName) // pipe
    .await
    .through(GenerateUserId) // pipe
    .await
    .through(ApplyDefaultRole) // ...
    .await
    // Using a closure or a function
    .through_fn(|mut new_user: NewUser, pipe: PipeContent| async move {
        println!(">> saving new user: {:?}", &new_user);

        new_user.internal_id = 1;
        pipe.store(new_user).await;
    })
    .await
    .deliver()
    .await;

    println!("fails validation: {:#?}", &new_user); // The flow is stopped by the "ValidateUserName" pipe because the user does not have a "username"
}

// Pipeline input
// Must be cloneable. A clone of the data is passed to any pipe that requires it
#[derive(Debug, Clone)]
struct NewUser {
    internal_id: i32,
    id: Option<String>,
    username: Option<String>,
    role: Option<Vec<UserRole>>,
}

impl Default for NewUser {
    fn default() -> Self {
        Self {
            internal_id: 0,
            id: None,
            username: None,
            role: None,
        }
    }
}

#[derive(Debug, Clone)]
enum UserRole {
    Admin,
    ContentCreator,
    Moderator,
    Basic,
}

struct ValidateUserName;

// A pipe must implement the `FamaPipe' trait
// In this case, ValidateUserName is expecting a tuple with two fields (NewUser, PipeContent)
// need to specify more arguments like in this case.
#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent), Option<PipeContent>> for ValidateUserName {
    async fn receive_pipe_content(
        &self,
        (new_user, content): (NewUser, PipeContent),
    ) -> Option<fama::PipeContent> {
        // When the username is "none", stop the flow
        if new_user.username.is_none() {
            println!("User name cannot be empty");
            content.stop_the_flow().await; // Notify the pipeline to stop flowing.
        }

        Some(content)
    }
}

struct GenerateUserId;
#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent), ()> for GenerateUserId {
    async fn receive_pipe_content(&self, (mut new_user, pipe): (NewUser, PipeContent)) {
        if new_user.id.is_none() {
            new_user.id = Some(uuid::Uuid::new_v4().to_string());
            pipe.store(new_user).await; // Store the changes to the input
        }
    }
}

struct ApplyDefaultRole;

#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent), ()> for ApplyDefaultRole {
    async fn receive_pipe_content(&self, (mut new_user, pipe): (NewUser, PipeContent)) {
        if new_user.role.is_none() {
            new_user.role = Some(vec![UserRole::Basic]);
            pipe.store(new_user).await;
        }
    }
}
