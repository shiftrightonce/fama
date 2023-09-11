# Fama

**Fama is a pipeline manager**

> Fama means "pass to someone" or "pass me"

 Fama makes it easy to layout the steps
 require to accomplish a task.
 Each step in the process is refer to as a pipe. The data or content is passed through
 the "pipes". At any stage the flow can be stopped.

 This pattern is usually refer to as the "pipeline" pattern. It is very similar to the
 middleware pattern.

 This implementation remove the responsibility of the current "pipe" calling the next pipe like
 in the middleware patten. A "pipe" can apply it's changes/logic or stop the flow. It is the "pipeline"
 that initiates the next call.

 The following example is illustrating a "New User" flow through the pipeline.

 ```rust
 #[tokio::main]
 async fn main() {
   // A new user instance is being passed through the pipeline
   let new_user = fama::Pipeline::pass(NewUser::default()) // The input/content
       .through(ValidateUserName).await  // pipe "ValidateUserName" will stop the flow if the user does not have a "username"
       .through(GenerateUserId).await    // pipe "GenerateUserId" generates and set the user ID.  
       .through(ApplyDefaultRole).await  // pipe "ApplyDefaultRole" will give the user the "Basic" role if the list of roles is empty
       .through(SaveNewUserData).await   // pipe "SaveNewUserData"  saves the data to the database. At this stage, we know all is well
       .through_fn(|c: fama::PipeContent| async {
           println!("yep, you can pass a closure or function too");
          Some(c)
       }).await
       .deliver();                       // starts the process or use
       // .confirm()                     // Return true when the content passes throug all the pipes

   // Fails because "new user" does not have a "username"
   println!("fails validation: {:#?}", &new_user);

   println!("----------------------------------------");

   let new_user2 = fama::Pipeline::pass(NewUser {
         username: Some("user1".into()),  // "new user" has a username
         ..NewUser::default()
     })
     .through(ValidateUserName).await
     .through(GenerateUserId).await
     .through(ApplyDefaultRole).await
     .through(SaveNewUserData).await
     .deliver();

   println!(
         "passes validation and all fields are set : {:#?}",
         &new_user2
    );
 }

 // The content or the pipeline input. Could be any type
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
     // get the instance of the type in the current scope or
     // create a new instance
     c.proxy_value().unwrap_or_else(|| Self::default())
  }
 }

 // The various roles a user can have
 #[derive(Debug, Clone)]
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
    async fn receive_pipe_content(&self, (new_user, mut content): (NewUser, fama::PipeContent)) -> Option<fama::PipeContent> {
  
//1        // When the username is "none", stop the flow
        if new_user.username.is_none() {
            println!("User name cannot be empty");
            content.stop_the_flow(); // Stop the pipeline flow. Pipes below this pipe will not get call
        }

       Some(content)
   }
 }

 struct GenerateUserId;

 #[fama::async_trait]
 impl fama::FamaPipe<(NewUser, fama::PipeContent), ()> for GenerateUserId {
     async fn receive_pipe_content(&self, (mut new_user, content): (NewUser,fama::PipeContent)) {
         if new_user.id.is_none() {
             new_user.id = Some(uuid::Uuid::new_v4().to_string()); // Generate and set the ID
             content.store(new_user);
         }

     }
 }

 struct ApplyDefaultRole;

 #[fama::async_trait]
 impl fama::FamaPipe<(NewUser, fama::PipeContent), Option<fama::PipeContent>> for ApplyDefaultRole {
     async fn receive_pipe_content(&self, (mut new_user, content): (NewUser,fama::PipeContent)) -> Option<fama::PipeContent> {

         if new_user.role.is_none() {
             new_user.role = Some(vec![UserRole::Basic]); // Apply default role
             content.store(new_user);
        }

         Some(content)
     }
 }

 struct SaveNewUserData;
 #[fama::async_trait]
 impl fama::FamaPipe<(NewUser, fama::PipeContent), Option<fama::PipeContent>> for SaveNewUserData {
     async fn receive_pipe_content(&self, (mut new_user, content): (NewUser,fama::PipeContent)) -> Option<fama::PipeContent> {

         println!(">> saving new user: {:?}", &new_user);

         new_user.internal_id = 1; // pretend we persisted the data to a database
         content.store(new_user);

         Some(content)
     }
 }
 ```
## Examples
The [examples](https://github.com/shiftrightonce/fama/tree/main/examples) folder contains some examples. If none of the examples are helpful,
please reach out with your use case and I  try to provide one.


## Feedback
If you find this crate useful, please star the repository. Submit your issues and recommendations as well.


## License

### The MIT License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.