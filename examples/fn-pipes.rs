#![allow(dead_code)]

use fama::PipeContent;

#[tokio::main]
async fn main() {
    // 1. Create a pipeline
    let created = fama::Pipeline::pass(NewUser {
        username: Some("james".into()),
        ..NewUser::default()
    }) // Start of the pipeline
    .await
    // Use "through_fn" in order to add a function or closure as a pipe
    .through_fn(validate_user)
    .await
    // Use a closure that generates the new user's ID
    .through_fn(|mut new_user: NewUser, content: PipeContent| async move {
        if new_user.id.is_none() {
            new_user.id = Some(uuid::Uuid::new_v4().to_string());
            content.store(new_user).await;
        }
        true
    })
    .await
    .through(ApplyDefaultRole)
    .await // A struct pipe
    .through(SaveNewUserData)
    .await // Another struct pipe
    .confirm(); // Confirm returns true when the content flows through all the pipes

    println!("new user created? {}", created);
}

// pipeline input
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
        c.get_type().await.unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
enum UserRole {
    Admin,
    ContentCreator,
    Moderator,
    Basic,
}

async fn validate_user(new_user: NewUser, pipe: PipeContent) -> Option<PipeContent> {
    // When the username is "none", stop the flow
    if new_user.username.is_none() {
        println!("User name cannot be empty");
        pipe.stop_the_flow().await;
    }

    Some(pipe)
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

struct SaveNewUserData;
#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent), ()> for SaveNewUserData {
    async fn receive_pipe_content(&self, (mut new_user, content): (NewUser, PipeContent)) {
        new_user.internal_id = 1;

        println!(">> saving new user: {:?}", &new_user);
        content.store(new_user).await;
    }
}
