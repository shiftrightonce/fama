#![allow(dead_code)]

#[tokio::main]
async fn main() {
    // 1. Create a pipeline
    let new_user = fama::Pipeline::pass(NewUser {
        username: Some("james".into()),
        ..NewUser::default()
    }) // Start of the pipeline
    .through(ValidateUserName) // pipe
    .through(GenerateUserId) // pipe
    .through(ApplyDefaultRole) // ...
    .through(SaveNewUserData)
    .through(ReturnNewUserInternalId) // This pipe is changing the returned type from "NewUser" to an i32
    // By default the pipeline will return the same type as the input content
    // If is the returned type is different from the input type, use "deliver_as" and
    // and specify the type
    .deliver_as::<i32>() // Starts the flow but return the specified type
    .await;

    println!("User ID: {:#?}", &new_user);
}

// pipeline input
#[derive(Debug)]
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

#[derive(Debug)]
enum UserRole {
    Admin,
    ContentCreator,
    Moderator,
    Basic,
}

struct ValidateUserName;

#[fama::async_trait]
impl fama::FamaPipe for ValidateUserName {
    async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
        let new_user: &mut NewUser = content.inner_mut().unwrap();

        // When the username is "none", stop the flow
        if new_user.username.is_none() {
            println!("User name cannot be empty");
            content.stop_the_flow(); // notify the pipeline to stop flowing.
        }

        content
    }
}

struct GenerateUserId;

#[fama::async_trait]
impl fama::FamaPipe for GenerateUserId {
    async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
        let new_user: &mut NewUser = content.inner_mut().unwrap();

        if new_user.id.is_none() {
            new_user.id = Some(uuid::Uuid::new_v4().to_string());
        }

        content
    }
}

struct ApplyDefaultRole;

#[fama::async_trait]
impl fama::FamaPipe for ApplyDefaultRole {
    async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
        let new_user: &mut NewUser = content.inner_mut().unwrap();

        if new_user.role.is_none() {
            new_user.role = Some(vec![UserRole::Basic]);
        }

        content
    }
}

struct SaveNewUserData;
#[fama::async_trait]
impl fama::FamaPipe for SaveNewUserData {
    async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
        let new_user: &mut NewUser = content.inner_mut().unwrap();
        new_user.internal_id = 78;

        println!(">> saving new user: {:?}", &new_user);

        content
    }
}

struct ReturnNewUserInternalId;
#[fama::async_trait]
impl fama::FamaPipe for ReturnNewUserInternalId {
    async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
        let new_user: &NewUser = content.inner_ref().unwrap();

        content.set_inner(new_user.internal_id);
        content
    }
}
