#![allow(dead_code)]
use fama::PipeContent;

#[tokio::main]
async fn main() {
    // 1. Create a pipeline
    let new_user = fama::Pipeline::pass(NewUser::default()) // Start of the pipeline
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
            pipe.container().set_type(new_user);

            None
        })
        .await
        .deliver();

    println!("fails validation: {:#?}", &new_user); // The flow is stopped by the "ValidateUserName" pipe because the user does not have a "username"
}

// pipeline input
// Must be clonable. A clone of the data is passed to any pipe that requires it
// The type must also be injectable. See futher below
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

// making the pipe input type injectable
#[fama::async_trait]
impl busybody::Injectable for NewUser {
    async fn inject(c: &busybody::ServiceContainer) -> Self {
        c.proxy_value().unwrap_or_else(|| Self::default())
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

// A pipe must implemenet the `FamaPipe' trait
// In this case, ValidateUserName is expecting a tuple with two fields (NewUser, PipeContent)
// A pipe can specify a parent parameter that is injectable. A tuple can be use when you
// need to specifiy more arguements like in this case.
#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent)> for ValidateUserName {
    async fn receive_pipe_content(
        &self,
        (new_user, mut content): (NewUser, PipeContent),
    ) -> Option<fama::PipeContent> {
        // When the username is "none", stop the flow
        if new_user.username.is_none() {
            println!("User name cannot be empty");
            content.stop_the_flow(); // notify the pipeline to stop flowing.
        }

        Some(content)
    }
}

struct GenerateUserId;
#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent)> for GenerateUserId {
    async fn receive_pipe_content(
        &self,
        (mut new_user, pipe): (NewUser, PipeContent),
    ) -> Option<fama::PipeContent> {
        if new_user.id.is_none() {
            new_user.id = Some(uuid::Uuid::new_v4().to_string());
            pipe.store(new_user); // Store the changes to the input
        }

        None
    }
}

struct ApplyDefaultRole;

#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent)> for ApplyDefaultRole {
    async fn receive_pipe_content(
        &self,
        (mut new_user, pipe): (NewUser, PipeContent),
    ) -> Option<PipeContent> {
        if new_user.role.is_none() {
            new_user.role = Some(vec![UserRole::Basic]);
            pipe.store(new_user);
        }

        None
    }
}
