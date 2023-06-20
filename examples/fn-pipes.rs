#![allow(dead_code)]

#[tokio::main]
async fn main() {
    // 1. Create a pipeline
    let created = fama::Pipeline::pass(NewUser {
        username: Some("james".into()),
        ..NewUser::default()
    }) // Start of the pipeline
    .through_fn(validate_user) // Use "through_fn" in order to add a function or closure as a pipe
    // Use a closure that generates the new user's ID
    .through_fn(|mut content: fama::PipeContent| async {
        let new_user: &mut NewUser = content.inner_mut().unwrap();

        if new_user.id.is_none() {
            new_user.id = Some(uuid::Uuid::new_v4().to_string());
        }

        content
    })
    .through(ApplyDefaultRole) // A struct pipe
    .through(SaveNewUserData) // Another struct pipe
    .confirm() // Confirm returns true when the conten flows through all the pipes
    .await;

    println!("new user created? {}", created);
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

async fn validate_user(mut content: fama::PipeContent) -> fama::PipeContent {
    let new_user: &mut NewUser = content.inner_mut().unwrap();

    // When the username is "none", stop the flow
    if new_user.username.is_none() {
        println!("User name cannot be empty");
        content.stop_the_flow(); // notify the pipeline to stop flowing.
    }

    content
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
        new_user.internal_id = 1;

        println!(">> saving new user: {:?}", &new_user);
        content.set_inner(200);

        content
    }
}
