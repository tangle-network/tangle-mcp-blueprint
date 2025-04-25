use crate::MyContext;
use blueprint_sdk::extract::Context;
use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};
use docktopus::bollard::container::RemoveContainerOptions;
use docktopus::bollard::models::ContainerSummary;
use std::fs;
use std::io;
use std::path::Path;

// Input parameters for destroy_project job
#[derive(Debug, serde::Deserialize)]
pub struct DestroyProjectParams {
    pub service_id: String,
}

#[blueprint_sdk::macros::debug_job]
pub async fn destroy_project(
    Context(ctx): Context<MyContext>,
    TangleArg(params): TangleArg<DestroyProjectParams>,
) -> Result<TangleResult<bool>, Box<dyn std::error::Error + Send + Sync>> {
    // Get service ID to identify the container
    let service_id = &params.service_id;

    // The container name follows the pattern: mcp-{service_id}
    let container_name = format!("mcp-{}", service_id);

    // Get all running containers that match our pattern
    let containers = ctx
        .docker
        .list_containers::<String>(None)
        .await
        .map_err(|e| {
            Box::<dyn std::error::Error + Send + Sync>::from(format!(
                "Failed to list containers: {}",
                e
            ))
        })?;

    let mut container_found = false;

    // Find and remove the container by name
    for container in containers {
        if let Some(names) = container.names {
            for name in names {
                // Container names from Docker API include a leading slash
                let normalized_name = name.trim_start_matches('/');
                if normalized_name == container_name {
                    container_found = true;

                    if let Some(container_id) = container.id {
                        // First try to stop the container
                        let _ = ctx.docker.stop_container(&container_id, None).await;

                        // Then remove it with force option to ensure it's gone
                        ctx.docker
                            .remove_container(
                                &container_id,
                                Some(RemoveContainerOptions {
                                    force: true,
                                    ..Default::default()
                                }),
                            )
                            .await
                            .map_err(|e| {
                                Box::<dyn std::error::Error + Send + Sync>::from(format!(
                                    "Failed to remove container: {}",
                                    e
                                ))
                            })?;

                        tracing::info!(
                            "Container {} successfully stopped and removed",
                            container_name
                        );
                        break;
                    }
                }
            }
        }
    }

    if !container_found {
        tracing::warn!("No container found with name: {}", container_name);
    }

    // Clean up any persistent data associated with this service
    if let Some(data_dir) = ctx.env.data_dir.as_ref() {
        let service_data_dir = data_dir.join(service_id);
        if service_data_dir.exists() {
            match remove_dir_all(&service_data_dir) {
                Ok(_) => tracing::info!("Removed data directory for service: {}", service_id),
                Err(e) => tracing::warn!("Failed to remove data directory: {}", e),
            }
        }
    }

    // Return success even if container wasn't found, to ensure idempotency
    Ok(TangleResult(true))
}

// Helper function to recursively remove a directory
fn remove_dir_all(path: &Path) -> io::Result<()> {
    if path.exists() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                remove_dir_all(&entry_path)?;
            } else {
                fs::remove_file(entry_path)?;
            }
        }
        fs::remove_dir(path)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blueprint_sdk::runner::config::BlueprintEnvironment;
    use docktopus::container::Container;
    use uuid::Uuid;

    #[tokio::test]
    #[ignore] // Ignored by default as it requires Docker
    async fn test_destroy_project() {
        // Setup
        let env = BlueprintEnvironment::load().unwrap();
        let ctx = MyContext::new(env).unwrap();

        // Create a test container to destroy
        let service_id = format!("test-{}", Uuid::new_v4());
        let container_name = format!("mcp-{}", service_id);

        // Create a simple container
        let container = Container::new(ctx.docker.clone(), "hello-world:latest")
            .name(&container_name)
            .create()
            .await;

        // Only run the actual test if container creation succeeded
        if let Ok(_) = container {
            // Now destroy it
            let params = DestroyProjectParams {
                service_id: service_id.clone(),
            };

            let result = destroy_project(Context(ctx.clone()), TangleArg(params)).await;

            assert!(result.is_ok());

            // Verify container is gone
            let containers = ctx.docker.list_containers(None).await.unwrap();
            let container_exists = containers.iter().any(|c| {
                if let Some(names) = &c.names {
                    names.iter().any(|n| n.contains(&container_name))
                } else {
                    false
                }
            });

            assert!(!container_exists, "Container should have been removed");
        }
    }
}
