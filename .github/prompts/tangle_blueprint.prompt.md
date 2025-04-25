# AI Prompt: Tangle Blueprint Implementation

**Objective:** Guide the development of Tangle-specific Blueprint components, ensuring correct integration with the Tangle network and SDK features.

**Core Components:**

1.  **`main.rs` Setup:**

    - Load `BlueprintEnvironment`.
    - Initialize `TanglePairSigner` using keys from the keystore (`env.keystore()`).
    - Initialize the Tangle client (`env.tangle_client().await?`).
    - Create `TangleProducer::finalized_blocks(...)`.
    - Create `TangleConsumer::new(...)` with the client and signer.
    - Initialize the application `Context`.
    - Build the `BlueprintRunner` with `TangleConfig`, router, producer, consumer, and context.

2.  **Job Handlers (`blueprint/src/jobs/`):**

    - **Signature:** Use `async fn`, inject `Context<MyContext>`, use `TangleArg`, `TangleArgsN`, `Optional`, `List` extractors for input deserialization.
    - **Return Type:** Must be `Result<TangleResult<T>>` where `T` is the output type defined in the job registration, or `Result<(), Error>` if no output is expected. `TangleResult` signals successful execution whose output should be submitted back to the Tangle network.
    - **Filtering:** Apply `TangleLayer` to handlers intended to react to standard Tangle job calls. Use `FilterLayer::new(MatchesServiceId(service_id))` if the job should only run for a specific service instance.

3.  **Context (`blueprint/src/context.rs`):**

    - Derive `#[derive(Clone, TangleClientContext, ServicesContext)]`.
    - Include `#[config] pub env: BlueprintEnvironment`.
    - Include necessary state like `data_dir: PathBuf`, `docker: Arc<Docker>`, etc.
    - Implement an async `new(env: BlueprintEnvironment) -> Result<Self>` constructor.

4.  **Job IDs & Naming:**
    - Define Job IDs as `pub const MY_JOB_ID: u64 = ...;`
    - Use descriptive `snake_case` names for handlers.
    - Use `#[debug_job]` macro for logging.

**Key Tangle Concepts:**

- **Producer:** `TangleProducer` reads finalized blocks from the Tangle RPC.
- **Consumer:** `TangleConsumer` takes `TangleResult` outputs from jobs and submits them as signed extrinsics back to the Tangle network.
- **`TangleLayer`:** Filters incoming job requests based on standard Tangle job call conventions (matching job ID, service ID if applicable).
- **Extractors (`TangleArg`, etc.):** Automatically deserialize job input arguments from the Tangle extrinsic data.

**Enforcement:**

- **MUST** use `TangleProducer` and `TangleConsumer` for standard Tangle interaction.
- **MUST** use `TanglePairSigner` initialized from the environment keystore.
- **MUST** apply `TangleLayer` or other appropriate filters to job handlers.
- **MUST** use `TangleArg`/`TangleArgsN` extractors for job inputs.
- **MUST** return `TangleResult<T>` for jobs that need to report results back to the chain.
- **DO NOT** manually decode block data; rely on the producer and extractors.
