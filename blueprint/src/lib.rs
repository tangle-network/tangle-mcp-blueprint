use blueprint_sdk::JobResult;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use docktopus::bollard::Docker;
use std::sync::Arc;

// Re-export jobs
mod jobs;
pub use jobs::*;

// Error type for the blueprint
#[derive(Debug)]
pub enum TangleError {
    ContainerError(String),
    ConfigError(String),
}

impl std::fmt::Display for TangleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TangleError::ContainerError(msg) => write!(f, "Container error: {}", msg),
            TangleError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

// Helper for successful results
pub fn tangle_result_to_job_result<T>(result: T) -> JobResult<T> {
    JobResult::Ok {
        head: Default::default(),
        body: result,
    }
}

// Blueprint context
#[derive(Clone)]
pub struct MyContext {
    pub env: BlueprintEnvironment,
    pub docker: Arc<Docker>,
}

impl MyContext {
    pub fn new(env: BlueprintEnvironment) -> Result<Self, TangleError> {
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| TangleError::ContainerError(e.to_string()))?;

        Ok(Self {
            env,
            docker: Arc::new(docker),
        })
    }
}
