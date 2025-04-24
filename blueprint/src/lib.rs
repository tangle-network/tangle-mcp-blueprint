use blueprint_sdk::JobResult;
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
    pub config: BlueprintConfig,
    pub docker: Arc<Docker>,
}

// Blueprint configuration
#[derive(Clone, Debug)]
pub struct BlueprintConfig {
    pub domain: Option<String>,
}

impl MyContext {
    pub async fn new(config: BlueprintConfig) -> Result<Self, TangleError> {
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| TangleError::ContainerError(e.to_string()))?;

        Ok(Self {
            config,
            docker: Arc::new(docker),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_context_creation() {
        let config = BlueprintConfig {
            domain: Some("localhost".to_string()),
        };
        let context = MyContext::new(config).await.unwrap();
        assert!(context.docker.ping().await.is_ok());
    }
}
