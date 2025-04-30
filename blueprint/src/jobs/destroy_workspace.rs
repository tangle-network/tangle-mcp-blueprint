use crate::MyContext;
use blueprint_sdk::extract::Context;
use blueprint_sdk::tangle::extract::{ServiceId, TangleArg, TangleResult};
use docktopus::bollard::container::RemoveContainerOptions;
use std::fs;
use std::io;
use std::path::Path;

#[blueprint_sdk::macros::debug_job]
pub async fn destroy_workspace(
    Context(ctx): Context<MyContext>,
    ServiceId(service_id): ServiceId,
    TangleArg(_): TangleArg<bool>,
) -> Result<TangleResult<bool>, Box<dyn std::error::Error + Send + Sync>> {
    // The container name follows the pattern: mcp-svc-{service_id}
    let container_name = format!("mcp-svc-{}", service_id);

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
        let service_data_dir = data_dir.join("workspaces").join(service_id.to_string());
        // Check if the directory exists before attempting to remove it
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
