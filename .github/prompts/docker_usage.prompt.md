# AI Prompt: Docker Container Management (Docktopus)

**Objective:** Ensure consistent and robust use of the `docktopus` crate for managing Docker containers within Blueprints.

**Core Principles:**

1.  **Initialization:**

    - Use the fluent builder pattern: `Container::new(docker_client, "image:tag").method(...).method(...)`.
    - Configure `env`, `cmd`, `binds`, `port_bindings`, `restart_policy`, `extra_hosts` etc., _before_ calling `.create()` or `.start()`.
    - Share the Docker client (`Arc<Docker>`) across the application; **DO NOT** create a new client per container. Initialize it once, likely in the `Context`.

2.  **Lifecycle Management:**

    - Follow the standard lifecycle: **Initialize → Create → Start → (Monitor/Wait) → Stop → Remove**.
    - Use `container.create().await?` to prepare the container.
    - Use `container.start(wait_for_exit).await?` to run it.
    - Use `container.stop().await?` before removal.
    - Use `container.remove(options).await?` for cleanup.
    - **DO NOT** skip `.create()` if manual start control is needed.
    - **DO NOT** call `.remove()` without stopping first, unless using `force: true`.

3.  **Context Integration:**

    - Manage `Container` instances within the Blueprint's `Context` struct.
    - Include `docker: Arc<Docker>` in the context alongside the `Container`.
    - Initialize the `DockerBuilder` and `Container` within the `Context::new()` async function.

4.  **Configuration:**

    - **Volume Bindings (`.binds([...])`):** Use `"host_path:container_path[:options]"` format (e.g., `"./data:/app/data"`, `"/logs:/var/log:ro"`).
    - **Port Mappings (`.port_bindings(map)`):** Use a `HashMap<String, Option<Vec<PortBinding>>>`. Key: `"container_port/tcp"`, Value: `vec![PortBinding { host_ip: ..., host_port: ... }]`.
    - **Restart Policies (`.restart_policy(...)`):** Explicitly set if needed (e.g., `RestartPolicyNameEnum::UNLESS_STOPPED`, `ON_FAILURE`). Default is no restart.
    - **Environment Variables (`.env([...])`):** Use `"KEY=value"` format.
    - **Commands (`.cmd([...])`):** Provide the command and arguments as a slice of strings.
    - **Extra Hosts (`.extra_hosts([...])`):** Use `"hostname:IP"` format (e.g., `"host.docker.internal:host-gateway"`). Optional.

5.  **Status Monitoring & Health Checks:**

    - Use `container.status().await?` to check the container state (`ContainerStatus`).
    - Implement logic to handle different states (e.g., remove unusable containers forcefully).
    - Integrate Docker's built-in health checks if the image supports them, or implement custom monitoring logic within the Blueprint job. Define health check parameters (interval, timeout, retries) during container creation if applicable via Dockerfile or container config.

6.  **Resource Management (Tiers):**
    - If implementing resource tiers (Small, Medium, Large), configure CPU, memory, and storage limits during container creation using appropriate `docktopus` or `bollard` options (e.g., within `HostConfig`). Refer to `memory-bank/systemPatterns.md` for tier definitions.

**Enforcement:**

- **MUST** use the fluent builder pattern for container creation.
- **MUST** follow the correct container lifecycle.
- **MUST** integrate container management within the Blueprint `Context`.
- **MUST** share the `Arc<Docker>` client.
- **MUST** explicitly define necessary configurations (ports, volumes, restart policies).
- **DO NOT** rely on implicit Docker defaults for critical settings.
- **DO NOT** ignore errors from `docktopus` operations.
