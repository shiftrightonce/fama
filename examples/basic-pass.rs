#![allow(dead_code)]

#[tokio::main]
async fn main() {
    let new_user2 = fama::Pipeline::pass(NewUser {
        username: Some("john".into()),
        ..NewUser::default()
    })
    .through(ValidateUserName)
    .through(GenerateUserId)
    .through(ApplyDefaultRole)
    .through(SaveNewUserData)
    .deliver()
    .await;

    println!(
        "passes validation and all fields are set : {:#?}",
        &new_user2
    );
}

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

        if new_user.username.is_none() {
            println!("User name cannot be empty");
            content.stop_the_flow();
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

        println!(">> saving new user: {:?}", &new_user);

        new_user.internal_id = 1;

        content
    }
}
