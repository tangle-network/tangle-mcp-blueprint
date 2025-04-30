use blueprint_sdk::runner::config::BlueprintEnvironment;
use docktopus::bollard::Docker;
use std::sync::Arc;

// Re-export jobs
mod jobs;
pub use jobs::*;

// Blueprint context
#[derive(Clone)]
pub struct MyContext {
    pub env: BlueprintEnvironment,
    pub docker: Arc<Docker>,
}

impl MyContext {
    pub fn new(
        env: BlueprintEnvironment,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let docker = Docker::connect_with_local_defaults()?;

        Ok(Self {
            env,
            docker: Arc::new(docker),
        })
    }
}
