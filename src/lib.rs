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
//! The following example is illustrating a "New User" flow through the pipeline.
//!
//! ```rust
//!#  #![allow(dead_code)]
//!
//! #[tokio::main]
//! async fn main() {
//!   // A new user instance is being passed through the pipeline
//!   let new_user = fama::Pipeline::pass(NewUser::default()) // The input/content
//!       .await
//!       .through(ValidateUserName).await  // pipe "ValidateUserName" will stop the flow if the user does not have a "username"
//!       .through(GenerateUserId).await    // pipe "GenerateUserId" generates and set the user ID.  
//!       .through(ApplyDefaultRole).await  // pipe "ApplyDefaultRole" will give the user the "Basic" role if the list of roles is empty
//!       .through(SaveNewUserData).await   // pipe "SaveNewUserData"  saves the data to the database. At this stage, we know all is well
//!       .through_fn(|_pipe: fama::PipeContent| async {
//!           println!("yep, you can pass a closure or function too");
//!       }).await
//!       .deliver().await;                       // starts the process or use
//!       // .confirm()                     // Return true when the content passes throug all the pipes
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
//!     .await
//!     .through(ValidateUserName).await
//!     .through(GenerateUserId).await
//!     .through(ApplyDefaultRole).await
//!     .through(SaveNewUserData).await
//!     .deliver().await;
//!
//!   println!(
//!         "passes validation and all fields are set : {:#?}",
//!         &new_user2
//!    );
//! }
//!
//! // The content for the pipeline input. Can be any type
//! #[derive(Debug, Clone)]
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
//!
//! // The various roles a user can have
//! #[derive(Debug, Clone)]
//! enum UserRole {
//!   Admin,
//!   ContentCreator,
//!   Moderator,
//!   Basic,
//! }
//!
//! struct ValidateUserName;
//!
//! // A struct becomes a pipe when it implements "fama::FamaPipe"
//! #[fama::async_trait]
//! impl fama::FamaPipe<(NewUser, fama::PipeContent), ()> for ValidateUserName {
//!
//!   // The only requirement is to implement "receive_pipe_content" method
//!    async fn receive_pipe_content(&self, (new_user, mut content): (NewUser, fama::PipeContent)) {
//!  
//1        // When the username is "none", stop the flow
//!        if new_user.username.is_none() {
//!            println!("User name cannot be empty");
//!            content.stop_the_flow(); // Stop the pipeline flow. Pipes below this pipe will not get call
//!        }
//!
//!   }
//! }
//!
//! struct GenerateUserId;
//!
//! #[fama::async_trait]
//! impl fama::FamaPipe<(NewUser, fama::PipeContent), Option<fama::PipeContent>> for GenerateUserId {
//!     async fn receive_pipe_content(&self, (mut new_user, content): (NewUser,fama::PipeContent)) -> Option<fama::PipeContent> {
//!
//!         if new_user.id.is_none() {
//!             new_user.id = Some(uuid::Uuid::new_v4().to_string()); // Generate and set the ID
//!             content.store(new_user);
//!         }
//!
//!       None
//!     }
//! }
//!
//! struct ApplyDefaultRole;
//!
//! #[fama::async_trait]
//! impl fama::FamaPipe<(NewUser, fama::PipeContent), Option<fama::PipeContent>> for ApplyDefaultRole {
//!     async fn receive_pipe_content(&self, (mut new_user, content): (NewUser,fama::PipeContent)) -> Option<fama::PipeContent> {
//!
//!         if new_user.role.is_none() {
//!             new_user.role = Some(vec![UserRole::Basic]); // Apply default role
//!             content.store(new_user);
//!        }
//!
//!         Some(content)
//!     }
//! }
//!
//! struct SaveNewUserData;
//! #[fama::async_trait]
//! impl fama::FamaPipe<(NewUser, fama::PipeContent), Option<fama::PipeContent>> for SaveNewUserData {
//!     async fn receive_pipe_content(&self, (mut new_user, content): (NewUser,fama::PipeContent)) -> Option<fama::PipeContent> {
//!
//!         println!(">> saving new user: {:?}", &new_user);
//!
//!         new_user.internal_id = 1; // pretend we persisted the data to a database
//!         content.store(new_user);
//!
//!         Some(content)
//!     }
//! }
//! ```
//!
mod content;
mod pipeline;
mod pipeline_builder;

pub use content::PipeContent;
pub use pipeline::FamaPipe;
pub use pipeline::Pipeline;

pub use async_trait::async_trait;
pub use busybody;
pub use pipeline_builder::PipelineBuilder;
pub use pipeline_builder::PipelineBuilderTrait;

#[async_trait::async_trait]
pub trait PipelineTrait {
    type Content: Clone + Send + Sync + 'static;

    async fn handle_pipe(&self, pipeline: Pipeline<Self::Content>) -> Pipeline<Self::Content>;

    async fn deliver(&self, subject: Self::Content) -> Self::Content {
        let pipeline = Pipeline::pass(subject).await;
        self.handle_pipe(pipeline).await.deliver().await
    }

    async fn try_to_deliver(&self, subject: Self::Content) -> Option<Self::Content> {
        let pipeline = Pipeline::pass(subject).await;
        self.handle_pipe(pipeline).await.try_deliver_as().await
    }

    async fn deliver_as<R: Clone + Send + Sync + 'static>(&self, subject: Self::Content) -> R
    where
        Self: Sized,
    {
        let pipeline = Pipeline::pass(subject).await;
        self.handle_pipe(pipeline).await.deliver_as().await
    }

    async fn try_deliver_as<R: Clone + Send + Sync + 'static>(
        &self,
        subject: Self::Content,
    ) -> Option<R>
    where
        Self: Sized,
    {
        let pipeline = Pipeline::pass(subject).await;
        self.handle_pipe(pipeline).await.try_deliver_as().await
    }

    async fn confirm(&self, subject: Self::Content) -> bool {
        let pipeline = Pipeline::pass(subject).await;
        self.handle_pipe(pipeline).await.confirm()
    }
}
