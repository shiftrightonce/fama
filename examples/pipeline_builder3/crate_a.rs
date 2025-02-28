use fama::{PipelineBuilder, PipelineBuilderTrait};

pub async fn setup() {
    println!("setting up crate a");
}

#[derive(Debug, Clone, Default)]
pub struct CreateUser {
    pub id: String,
    pub roles: Vec<String>,
}

impl CreateUser {
    pub fn add_role(&mut self, role: &str) -> &Self {
        self.roles.push(role.to_string());

        self
    }
}

#[fama::async_trait]
impl PipelineBuilderTrait for CreateUser {
    async fn setup_pipeline_builder(builder: PipelineBuilder<Self>) -> PipelineBuilder<Self> {
        builder
            .register(|p| {
                Box::pin(async {
                    p.store_fn(|mut user: CreateUser| async {
                        let id = "xyz-123".to_string();
                        println!("generating user Id: {}", &id);

                        user.id = id;
                        user
                    })
                    .await
                    .store_fn(|mut user: CreateUser| async {
                        println!("applying 'User' role to the new record");
                        user.roles.push("User".to_string());
                        user
                    })
                    .await
                })
            })
            .await;

        builder
    }
}
