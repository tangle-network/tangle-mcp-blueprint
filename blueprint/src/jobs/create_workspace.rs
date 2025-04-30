use crate::MyContext;
use blueprint_sdk::crypto::BytesEncoding;
use blueprint_sdk::crypto::sp_core::SpSr25519Public;
use blueprint_sdk::extract::Context;
use blueprint_sdk::std::{Rng, rand};
use blueprint_sdk::tangle::extract::{ServiceId, TangleArg, TangleResult};
use docktopus::bollard::Docker;
use docktopus::bollard::container::{InspectContainerOptions, RemoveContainerOptions};
use docktopus::bollard::models::PortBinding;
use docktopus::bollard::secret::{ContainerStateStatusEnum, HealthStatusEnum};
use docktopus::container::Container;
use std::collections::HashMap;
use std::sync::Arc;

// Resource tiers for container allocation
#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceTier {
    Small,
    #[default]
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

// Input parameters for create workspace job
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateWorkspaceParams {
    /// Substrate Sr25519 Public Key in ss58 format.
    pub owner_public_key: SpSr25519Public,
    pub tier: ResourceTier,
    pub workspace_name: String,
}

impl Default for CreateWorkspaceParams {
    fn default() -> Self {
        Self {
            owner_public_key: SpSr25519Public::from_bytes(&[0u8; 32]).unwrap(),
            tier: Default::default(),
            workspace_name: Default::default(),
        }
    }
}

// Project container configuration
struct WorkspaceContainer {
    container: Container,
    service_id: u64,
    port: u16,
    docker: Arc<Docker>,
}

impl WorkspaceContainer {
    async fn new(
        ctx: &MyContext,
        service_id: u64,
        params: &CreateWorkspaceParams,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut rng = rand::rngs::OsRng;
        // Allocate a random port between 10000-20000
        let port = rng.gen_range(10000..20000);

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

        // Create a new container - first create with the image
        let mut container = Container::new(ctx.docker.clone(), "tangle-mcp:0.1.0");

        // Add environment variables
        container = container.env(&env);

        // Add port bindings
        container = container.port_bindings(port_bindings);

        // Add container name
        container = container.with_name(format!("mcp-svc-{}", service_id));

        // Set up bind volumes
        let project_id = format!("workspaces/{}", service_id);

        if let Some(ref data_dir) = ctx.env.data_dir {
            let host_path = format!("{}/{}", data_dir.display(), project_id);
            std::fs::create_dir_all(&host_path)?;
            let host_path = std::fs::canonicalize(&host_path)?;
            let host_path = host_path.display().to_string();
            // Set up the container path
            let bind = format!("{}:/blueprint:rw", host_path);

            // Add bind volumes
            container = container.binds(&[bind]);
        }

        // Create the container
        container.create().await?;

        // Return the object after successful container creation
        Ok(Self {
            container,
            service_id,
            port,
            docker: ctx.docker.clone(),
        })
    }

    async fn start_and_wait_healthy(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Start the container
        blueprint_sdk::info!("Starting container for service ID: {}", self.service_id);
        self.container.start(false).await?;

        blueprint_sdk::info!("Container started, waiting for health check...");
        // Wait for container to be healthy (timeout after 30 seconds)
        let start_time = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(30);

        while start_time.elapsed() < timeout {
            // Get container status using Docker API directly
            if let Some(container_id) = self.container.id() {
                blueprint_sdk::info!("Inspecting container: {}", container_id);
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

            blueprint_sdk::info!("Container not healthy yet, sleeping...");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        // Container didn't become healthy in time, clean up with force
        blueprint_sdk::error!(
            "Container failed to become healthy within timeout, stopping and removing..."
        );
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
pub async fn create_workspace(
    Context(ctx): Context<MyContext>,
    ServiceId(service_id): ServiceId,
    TangleArg(params): TangleArg<CreateWorkspaceParams>,
) -> Result<TangleResult<String>, Box<dyn std::error::Error + Send + Sync>> {
    blueprint_sdk::info!("Creating workspace with params: {:?}", params);
    blueprint_sdk::info!("Service ID: {}", service_id);
    let mut workspace = WorkspaceContainer::new(&ctx, service_id, &params).await?;

    // Wait for container to be healthy
    workspace.start_and_wait_healthy().await?;

    // Return the SSE URL
    let sse = workspace.get_sse_url("localhost");
    blueprint_sdk::info!("SSE URL: {}", sse);
    Ok(TangleResult(sse))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_args() {
        let inputs = include_str!("../../tests/create_workspace.json");
        let parsed_args = serde_json::from_str::<Vec<CreateWorkspaceParams>>(inputs).unwrap();
        assert!(
            !parsed_args.is_empty(),
            "Parsed arguments should not be empty"
        );
    }
}
