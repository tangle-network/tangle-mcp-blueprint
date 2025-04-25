use crate::MyContext;
use blueprint_sdk::extract::Context;
use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};
use docktopus::bollard::Docker;
use docktopus::bollard::container::{InspectContainerOptions, RemoveContainerOptions};
use docktopus::bollard::models::PortBinding;
use docktopus::bollard::secret::{ContainerStateStatusEnum, HealthStatusEnum};
use docktopus::container::Container;
use std::collections::HashMap;
use std::sync::Arc;

// Resource tiers for container allocation
#[derive(Debug, Clone, serde::Deserialize)]
pub enum ResourceTier {
    Small,
    Medium,
    Large,
}

impl ResourceTier {
    fn cpu_limit(&self) -> f64 {
        match self {
            ResourceTier::Small => 1.0,
            ResourceTier::Medium => 2.0,
            ResourceTier::Large => 4.0,
        }
    }

    fn memory_limit(&self) -> i64 {
        match self {
            ResourceTier::Small => 1024 * 1024 * 1024,      // 1GB
            ResourceTier::Medium => 2 * 1024 * 1024 * 1024, // 2GB
            ResourceTier::Large => 4 * 1024 * 1024 * 1024,  // 4GB
        }
    }

    fn storage_limit(&self) -> u64 {
        match self {
            ResourceTier::Small => 5 * 1024 * 1024 * 1024,   // 5GB
            ResourceTier::Medium => 10 * 1024 * 1024 * 1024, // 10GB
            ResourceTier::Large => 20 * 1024 * 1024 * 1024,  // 20GB
        }
    }
}

// Input parameters for create_project job
#[derive(Debug, serde::Deserialize)]
pub struct CreateProjectParams {
    pub owner_public_key: String,
    pub tier: ResourceTier,
}

// Project container configuration
struct ProjectContainer {
    container: Container,
    port: u16,
    docker: Arc<Docker>,
}

impl ProjectContainer {
    async fn new(
        docker: Arc<Docker>,
        params: &CreateProjectParams,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Allocate a random port between 10000-20000
        let port = 20000;

        // Set up port bindings
        let mut port_bindings = HashMap::new();
        port_bindings.insert(
            "3000/tcp".to_string(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".into()),
                host_port: Some(port.to_string()),
            }]),
        );

        // Set up environment variables
        let env = vec![
            format!("OWNER_PUBLIC_KEY={}", params.owner_public_key),
            "PORT=3000".to_string(),
        ];

        // Set up bind volumes
        let bind = format!("size={}", params.tier.storage_limit());

        // Create a new container - first create with the image
        let mut container = Container::new(docker.clone(), "mcp-server:latest");

        // Now add command
        container = container.cmd(&[String::from("serve")]);

        // Add environment variables
        container = container.env(&env);

        // Add bind volumes
        container = container.binds(&[bind]);

        // Create the container
        container.create().await?;

        // Return the object after successful container creation
        Ok(Self {
            container,
            port,
            docker: docker.clone(),
        })
    }

    async fn start_and_wait_healthy(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Start the container
        self.container.start(false).await?;

        // Wait for container to be healthy (timeout after 30 seconds)
        let start_time = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(30);

        while start_time.elapsed() < timeout {
            // Get container status using Docker API directly
            if let Some(container_id) = self.container.id() {
                let inspect_options = InspectContainerOptions { size: false };

                if let Ok(info) = self
                    .docker
                    .inspect_container(container_id, Some(inspect_options))
                    .await
                {
                    // Check if running and healthy
                    if let Some(state) = &info.state {
                        if let Some(status) = &state.status {
                            // Use proper enum comparison for container status
                            if *status == ContainerStateStatusEnum::RUNNING {
                                if let Some(health) = &state.health {
                                    if let Some(health_status) = &health.status {
                                        // Use proper enum comparison for health status
                                        if *health_status == HealthStatusEnum::HEALTHY {
                                            return Ok(());
                                        }
                                    }
                                }
                            } else if *status == ContainerStateStatusEnum::EXITED
                                || *status == ContainerStateStatusEnum::DEAD
                            {
                                break;
                            }
                        }
                    }
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        // Container didn't become healthy in time, clean up with force
        self.container.stop().await?;

        // Use a cloned reference to the container ID for removal
        if let Some(container_id) = self.container.id() {
            // Use the Docker client directly for removal to avoid borrowing issues
            self.docker
                .remove_container(
                    container_id,
                    Some(RemoveContainerOptions {
                        force: true,
                        ..Default::default()
                    }),
                )
                .await?;
        }

        Err("Container failed to become healthy within timeout".into())
    }

    fn get_sse_url(&self, domain: &str) -> String {
        format!("http://{}:{}/sse", domain, self.port)
    }
}

#[blueprint_sdk::macros::debug_job]
pub async fn create_project(
    Context(ctx): Context<MyContext>,
    TangleArg(params): TangleArg<CreateProjectParams>,
) -> Result<TangleResult<String>, Box<dyn std::error::Error + Send + Sync>> {
    let docker = ctx.docker.clone();
    let mut project = ProjectContainer::new(docker, &params).await?;

    // Wait for container to be healthy
    project.start_and_wait_healthy().await?;

    // Return the SSE URL
    let sse = project.get_sse_url("localhost");
    Ok(TangleResult(sse))
}

#[cfg(test)]
mod tests {
    use blueprint_sdk::runner::config::BlueprintEnvironment;

    use super::*;
    use crate::MyContext;

    #[tokio::test]
    #[ignore] // Ignored by default as it requires Docker
    async fn test_create_project() {
        let env = BlueprintEnvironment::load().unwrap();
        let _ctx = MyContext::new(env).unwrap();

        let _params = CreateProjectParams {
            owner_public_key: "test_key".to_string(),
            tier: ResourceTier::Small,
        };

        // This is just a test skeleton - actual test implementation would depend
        // on having Docker and the required images available
    }
}
