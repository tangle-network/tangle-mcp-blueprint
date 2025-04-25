# AI Prompt: Context Definition and State Management

**Objective:** Ensure the Blueprint `Context` struct is correctly defined, initialized, and used for managing application state and clients.

**Context Definition (`blueprint/src/context.rs`):**

1.  **Structure:**

    - Define a `pub struct MyContext { ... }` (replace `MyContext` with a descriptive name).
    - **MUST** include `#[config] pub env: BlueprintEnvironment`. This field provides access to configuration, keystore, data directories, and client builders.
    - Include other necessary state fields:
      - `data_dir: PathBuf` (often derived from `env.data_dir`).
      - Shared clients like `docker: Arc<Docker>` (for `docktopus`).
      - RPC clients, database connections (wrapped in `Arc` if shared across tasks/threads).
      - Signers if needed directly in context (e.g., `signer: TanglePairSigner`).
      - Any other shared application state.

2.  **Traits:**

    - **MUST** derive `#[derive(Clone)]`. Contexts need to be cloneable to be passed into job handlers.
    - Derive necessary context traits for routing and SDK integration:
      - `TangleClientContext` (if using Tangle client features).
      - `ServicesContext` (if interacting with service registry).
      - `KeystoreContext` (if directly accessing keystore methods beyond `env.keystore()`).
      - Other traits as required by specific SDK components or layers.

3.  **Initialization:**
    - Implement an `impl MyContext { pub async fn new(env: BlueprintEnvironment) -> Result<Self> { ... } }`.
    - Initialization **MUST** be `async` to allow for async setup of clients (like Docker, RPC connections).
    - Initialize all fields within `new()`.
    - Properly handle potential errors during initialization and return `Result`.
    - Example: Initialize `DockerBuilder` and store `docker.client()` in the context.
    - Example: Determine `data_dir` using `env.data_dir.clone().unwrap_or_else(default_data_dir)`.

**Context Usage:**

1.  **Injection:**

    - The `Context` is injected into job handlers using the `Context(ctx): Context<MyContext>` extractor.
    - The runner automatically clones and passes the context based on the `.with_context(context)` call during router setup.

2.  **Accessing State:**
    - Inside job handlers, access fields via `ctx.field_name`.
    - Access environment config via `ctx.env`.
    - Access keystore via `ctx.env.keystore()`.
    - Access Tangle client via `ctx.env.tangle_client().await?`.
    - Access Docker client via `ctx.docker`.

**State Management Principles:**

- The `Context` is the primary mechanism for sharing state and clients across job handlers.
- Use `Arc` for thread-safe sharing of clients or state that needs to be accessed concurrently (though job execution is typically sequential per service).
- For persistent state associated with a specific service instance (like a running Docker container ID or configuration), consider storing it:
  - In files within a service-specific subdirectory under `ctx.data_dir`.
  - In a database keyed by service ID.
  - _Avoid_ storing highly dynamic, per-job-instance state directly in the main `Context` struct itself, as it's cloned for each job.

**Enforcement:**

- **MUST** define the context struct in `blueprint/src/context.rs`.
- **MUST** include `#[config] pub env: BlueprintEnvironment`.
- **MUST** derive `Clone` and necessary SDK context traits.
- **MUST** have an `async fn new(...) -> Result<Self>` constructor.
- **MUST** initialize shared clients (like Docker) within the `new` function and store them (often wrapped in `Arc`).
- **MUST** access context in handlers via the `Context<MyContext>` extractor.
