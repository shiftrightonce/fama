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
    .through(ValidateUserName) // pipe
    .await
    .through(GenerateUserId) // pipe
    .await
    .through(ApplyDefaultRole) // ...
    .await
    .through(SaveNewUserData)
    .await
    .confirm(); // Confirm returns true when the content flows through all the pipes

    println!("User 1 was created: {:#?}", &created);
    println!("-----------------------------------------");

    let created = fama::Pipeline::pass(NewUser::default()) // Start of the pipeline
        .await
        .through(ValidateUserName) // pipe
        .await
        .through(GenerateUserId) // pipe
        .await
        .through(ApplyDefaultRole) // ...
        .await
        .through(SaveNewUserData)
        .await
        .confirm(); // Confirm returns true when the content flows through all the pipes

    println!("User 2 was created: {:#?}", &created);
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

struct ValidateUserName;

// A struct that can be used as a pipe must implement "fama::FamaPipe"
#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent), Option<PipeContent>> for ValidateUserName {
    async fn receive_pipe_content(
        &self,
        (new_user, content): (NewUser, PipeContent),
    ) -> Option<PipeContent> {
        // When the username is "none", stop the flow
        if new_user.username.is_none() {
            println!("User name cannot be empty");
            content.stop_the_flow().await; // notify the pipeline to stop flowing.
        }

        Some(content)
    }
}

struct GenerateUserId;

#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent), Option<PipeContent>> for GenerateUserId {
    async fn receive_pipe_content(
        &self,
        (mut new_user, content): (NewUser, PipeContent),
    ) -> Option<PipeContent> {
        if new_user.id.is_none() {
            new_user.id = Some(uuid::Uuid::new_v4().to_string());
            content.store(new_user).await;
        }

        Some(content)
    }
}

struct ApplyDefaultRole;

#[fama::async_trait]
impl fama::FamaPipe<(NewUser, PipeContent), Option<PipeContent>> for ApplyDefaultRole {
    async fn receive_pipe_content(
        &self,
        (mut new_user, content): (NewUser, PipeContent),
    ) -> Option<PipeContent> {
        if new_user.role.is_none() {
            new_user.role = Some(vec![UserRole::Basic]);
            content.store(new_user).await;
        }

        Some(content)
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
