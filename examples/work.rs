#[tokio::main]
async fn main() {
    // A new user instance is being passed through the pipeline
    let new_user = fama::Pipeline::pass(NewUser::default()) // The input/content
        .await
        .through(ValidateUserName)
        .await // pipe "ValidateUserName" will stop the flow if the user does not have a "username"
        .through(GenerateUserId)
        .await // pipe "GenerateUserId" generates and set the user ID.
        .through(ApplyDefaultRole)
        .await // pipe "ApplyDefaultRole" will give the user the "Basic" role if the list of roles is empty
        .through(SaveNewUserData)
        .await // pipe "SaveNewUserData"  saves the data to the database. At this stage, we know all is well
        .through_fn(|c: fama::PipeContent| async {
            println!("yep, you can pass a closure or function too");
            Some(c)
        })
        .await
        .deliver()
        .await; // starts the process or use
                // .confirm()                     // Return true when the content passes throug all the pipes

    // Fails because "new user" does not have a "username"
    println!("fails validation: {:#?}", &new_user);

    println!("----------------------------------------");

    let new_user2 = fama::Pipeline::pass(NewUser {
        username: Some("user1".into()), // "new user" has a username
        ..NewUser::default()
    })
    .await
    .through(ValidateUserName)
    .await
    .through(GenerateUserId)
    .await
    .through(ApplyDefaultRole)
    .await
    .through(SaveNewUserData)
    .await
    .deliver()
    .await;

    println!(
        "passes validation and all fields are set : {:#?}",
        &new_user2
    );
}

// The content for the pipeline input. Can be any type
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

// The various roles a user can have
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum UserRole {
    Admin,
    ContentCreator,
    Moderator,
    Basic,
}

struct ValidateUserName;

// A struct becomes a pipe when it implements "fama::FamaPipe"
#[fama::async_trait]
impl fama::FamaPipe<(NewUser, fama::PipeContent), Option<fama::PipeContent>> for ValidateUserName {
    // The only requirement is to implement "receive_pipe_content" method
    async fn receive_pipe_content(
        &self,
        (new_user, content): (NewUser, fama::PipeContent),
    ) -> Option<fama::PipeContent> {
        //1        // When the username is "none", stop the flow
        if new_user.username.is_none() {
            println!("User name cannot be empty");
            content.stop_the_flow().await; // Stop the pipeline flow. Pipes below this pipe will not get call
        }

        Some(content)
    }
}

struct GenerateUserId;

#[fama::async_trait]
impl fama::FamaPipe<(NewUser, fama::PipeContent), ()> for GenerateUserId {
    async fn receive_pipe_content(&self, (mut new_user, content): (NewUser, fama::PipeContent)) {
        if new_user.id.is_none() {
            new_user.id = Some(uuid::Uuid::new_v4().to_string()); // Generate and set the ID
            content.store(new_user).await;
        }
    }
}

struct ApplyDefaultRole;

#[fama::async_trait]
impl fama::FamaPipe<(NewUser, fama::PipeContent), Option<fama::PipeContent>> for ApplyDefaultRole {
    async fn receive_pipe_content(
        &self,
        (mut new_user, content): (NewUser, fama::PipeContent),
    ) -> Option<fama::PipeContent> {
        if new_user.role.is_none() {
            new_user.role = Some(vec![UserRole::Basic]); // Apply default role
            content.store(new_user).await;
        }

        Some(content)
    }
}

struct SaveNewUserData;
#[fama::async_trait]
impl fama::FamaPipe<(NewUser, fama::PipeContent), Option<fama::PipeContent>> for SaveNewUserData {
    async fn receive_pipe_content(
        &self,
        (mut new_user, content): (NewUser, fama::PipeContent),
    ) -> Option<fama::PipeContent> {
        println!(">> saving new user: {:?}", &new_user);

        new_user.internal_id = 1; // pretend we persisted the data to a database
        content.store(new_user).await;

        Some(content)
    }
}
