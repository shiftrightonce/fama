//! Fama provides a series of functionalities that makes it easy to layout the steps
//! require to accomplish a task.
//! Each step in the process is refer to as a pipe. The data or content is passed through
//! the "pipes". At any stage the flow can be stopped.
//!
//! This pattern is usually refer to as the "pipeline" pattern. It is very similar to the
//! middleware pattern.
//!
//! This implementation remove the responsibility of the current "pipe" calling the next pipe like
//! in the middleware patten. A "pipe" can apply it's changes/logic or stop the flow. It is the "pipeline"
//! that initiates the next call.
//!
//! The follow example is illustrating a "New User" flow through the pipeline.
//!
//! ```rust
//!#  #![allow(dead_code)]
//!
//! #[tokio::main]
//! async fn main() {
//!   // A new user instance is being passed through the pipeline
//!   let new_user = fama::Pipeline::pass(NewUser::default()) // The input/content
//!       .through(ValidateUserName) // pipe "ValidateUserName" will stop the flow if the user does not have a "username"
//!       .through(GenerateUserId)   // pipe "GenerateUserId" generates and set the user ID.  
//!       .through(ApplyDefaultRole) // pipe "ApplyDefaultRole" will give the user the "Basic" role if the list of roles is empty
//!       .through(SaveNewUserData)  // pipe "SaveNewUserData"  saves the data to the database. At this stage, we know all is well
//!       .deliver() // starts the process
//!       .await;
//!
//!   // Fails because "new user" does not have a "username"
//!   println!("fails validation: {:#?}", &new_user);
//!
//!   println!("----------------------------------------");
//!
//!   let new_user2 = fama::Pipeline::pass(NewUser {
//!         username: Some("user1".into()),  // "new user" has a username
//!         ..NewUser::default()
//!     })
//!     .through(ValidateUserName)
//!     .through(GenerateUserId)
//!     .through(ApplyDefaultRole)
//!     .through(SaveNewUserData)
//!     .deliver()
//!     .await;
//!
//!   println!(
//!         "passes validation and all fields are set : {:#?}",
//!         &new_user2
//!    );
//! }
//!
//! // The content or the pipeline input. Could be any type
//! #[derive(Debug)]
//! struct NewUser {
//!   internal_id: i32,
//!   id: Option<String>,
//!   username: Option<String>,
//!   role: Option<Vec<UserRole>>,
//! }
//!
//! impl Default for NewUser {
//!   fn default() -> Self {
//!       Self {
//!           internal_id: 0,
//!           id: None,
//!           username: None,
//!           role: None,
//!       }
//!   }
//! }
//!
//! // The various roles a user can have
//! #[derive(Debug)]
//! enum UserRole {
//!   Admin,
//!   ContentCreator,
//!   Moderator,
//!   Basic,
//! }
// !
//! struct ValidateUserName;
//!
//! // A struct becomes a pipe when it implements "fama::FamaPipe"
//! #[fama::async_trait]
//! impl fama::FamaPipe for ValidateUserName {
//!
//!   // The only requirement is to implement "receive_pipe_content" method
//!    async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
//!        let new_user: &mut NewUser = content.inner_mut().unwrap(); // get pipeline content
//!  
//!        if new_user.username.is_none() {
//!            println!("User name cannot be empty");
//!            content.stop_the_flow(); // Stop the pipeline flow. Pipes below this pipe will not get call
//!        }
//!
//!
//!       content
//!   }
//! }
//!
//! struct GenerateUserId;

//! #[fama::async_trait]
//! impl fama::FamaPipe for GenerateUserId {
//!     async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
//!         let new_user: &mut NewUser = content.inner_mut().unwrap();
//!
//!         if new_user.id.is_none() {
//!             new_user.id = Some(uuid::Uuid::new_v4().to_string()); // Generate an set the ID
//!         }
//!
//!         content
//!     }
//! }
//!
//! struct ApplyDefaultRole;
//!
//! #[fama::async_trait]
//! impl fama::FamaPipe for ApplyDefaultRole {
//!     async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
//!         let new_user: &mut NewUser = content.inner_mut().unwrap();
//!
//!         if new_user.role.is_none() {
//!             new_user.role = Some(vec![UserRole::Basic]); // Apply default role
//!        }
//!
//!         content
//!     }
//! }
//!
//! struct SaveNewUserData;
//! #[fama::async_trait]
//! impl fama::FamaPipe for SaveNewUserData {
//!     async fn receive_pipe_content(&self, mut content: fama::PipeContent) -> fama::PipeContent {
//!         let new_user: &mut NewUser = content.inner_mut().unwrap();
//!
//!         println!(">> saving new user: {:?}", &new_user);
//!
//!         new_user.internal_id = 1; // pretend we persisted the data to a database
//!
//!         content
//!     }
//! }
//! ```
//!
mod content;
mod pipeline;

pub use content::PipeContent;
pub use pipeline::FamaPipe;
pub use pipeline::Pipeline;

pub use async_trait::async_trait;
