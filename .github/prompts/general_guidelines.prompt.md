# AI Prompt: General Project Guidelines & Goals

**Objective:** Provide high-level context, goals, and common pitfalls to guide AI contributions effectively.

**Project Goal (Based on `productContext.md`):**

- The primary goal is to create a Tangle Blueprint service that manages an MCP (Multi-Party Computation) server running inside a Docker container.
- Key Jobs:
  - `create_project`: Starts the MCP server container, potentially configuring it based on inputs.
  - `destroy_project`: Stops and removes the MCP server container and associated state.
- Focus on robust Docker integration using `docktopus`, resource management (tiers), and health monitoring.

**Key Architectural Concepts:**

- **Modularity:** Follow the strict `bin`/`blueprint` crate separation. Keep logic within the `blueprint` library crate.
- **Microservice Pattern:** Treat the Blueprint as a self-contained service triggered by on-chain events (jobs).
- **Producer/Consumer:** Understand the flow: `TangleProducer` reads blocks -> `Router` dispatches jobs -> Handlers execute logic (e.g., manage Docker) -> `TangleConsumer` submits results.
- **State Management:** Use the `Context` struct for shared state/clients. Persist service-specific state (like container IDs) in the `data_dir` or a database, not directly in the main `Context` fields after initialization.

**Common Pitfalls & "Don'ts":**

- **❌ Logic in `bin` Crate:** Do NOT put any application logic, job handlers, or context definitions in `bin/src/main.rs`. It's strictly for initialization.
- **❌ Ignoring Errors:** Do NOT silently ignore `Result` errors from SDK functions, `docktopus`, file operations, etc. Propagate them using `?`.
- **❌ New Docker Clients:** Do NOT create a new `Docker` client for each operation or container. Initialize it once (likely in `Context::new`) and share via `Arc<Docker>`.
- **❌ Implicit Docker Behavior:** Do NOT rely on Docker defaults for critical settings like restart policies or resource limits. Specify them explicitly using `docktopus` builder methods.
- **❌ Manual Block Decoding:** Do NOT attempt to manually parse block data in Tangle Blueprints. Use the `TangleArg`/`TangleArgsN` extractors provided by the SDK.
- **❌ Blocking Operations:** Avoid long-running synchronous operations within async job handlers. Use `tokio::spawn_blocking` if necessary, but prefer async APIs.
- **❌ Naming Collisions:** Ensure Job IDs (`const JOB_ID: u64`) are unique. Use descriptive names for handlers and variables.
- **❌ Incorrect Producer/Consumer:** Ensure the chosen Producer/Consumer pair matches the event source and target chain (e.g., `TangleProducer`/`TangleConsumer` for standard Tangle).

**Development Process:**

1.  **Understand Requirements:** Refer to `productContext.md` and `activeContext.md` for current goals and status.
2.  **Structure:** Adhere to `projectStructure.md`.
3.  **Implement:** Write code following `coding_standards.md`, `docker_usage.md`, `tangle_blueprint.md`, and `context_and_state.md`.
4.  **Test:** Add integration tests as per `testing.md`.
5.  **Document:** Update `README.md` and potentially `memory-bank` files if significant changes or decisions are made.

**Focus Areas for this Project:**

- Robust Docker container lifecycle management (`create`, `start`, `stop`, `remove`, status checks).
- Implementing resource tiers (Small/Medium/Large) for containers.
- Setting up health checks and potentially SSE endpoints for monitoring.
- Correctly handling job inputs and returning outputs (`TangleResult`).
- Writing comprehensive integration tests covering job logic and Docker interactions.
