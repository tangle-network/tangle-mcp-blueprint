# AI Prompt: Rust Coding Standards & Blueprint Patterns

**Objective:** Ensure all generated Rust code follows idiomatic practices and adheres to Blueprint SDK conventions.

**General Rust Standards:**

1.  **Naming Conventions:**

    - Crates & Folders: `kebab-case`
    - Rust Files: `snake_case.rs`
    - Types (Structs, Enums, Traits): `PascalCase`
    - Functions & Variables: `snake_case`
    - Constants & Statics: `SCREAMING_SNAKE_CASE`
    - Job IDs: `pub const JOB_NAME_ID: u64 = ...;`

2.  **Error Handling:**

    - Propagate errors using `Result<T, E>`. Avoid `unwrap()` or `expect()` in production code. Use `?` operator.
    - Define custom error types where appropriate, implementing `std::error::Error` and `From` traits.

3.  **Modularity:**
    - Organize code into logical modules within the `blueprint/src/` directory (e.g., `jobs`, `context`, `utils`, `docker`).
    - Use `pub mod module_name;` in `lib.rs` or parent modules to expose functionality.

**Blueprint SDK Patterns:**

1.  **Blueprint Runner (`main.rs`):**

    - Use `BlueprintRunner::builder(config, env)` for initialization.
    - Configure the `Router`, `Producer`, `Consumer`, and `Context`.
    - Pass the appropriate config (`TangleConfig`, `EigenlayerBLSConfig`, etc.) based on the target chain for results.

2.  **Router & Job Routing (`main.rs` / `blueprint/src/lib.rs`):**

    - Use `Router::new().route(JOB_ID, handler.layer(...)).with_context(ctx)`.
    - Apply appropriate layers: `TangleLayer`, `FilterLayer::new(MatchesServiceId(...))`, `FilterLayer::new(MatchesContract(...))`.
    - Ensure Job IDs are unique constants.

3.  **Job Handlers (`blueprint/src/jobs/`):**

    - Signature: `async fn handler_name(Context<MyContext>, TangleArg<T>, ...) -> Result<TangleResult<U>>` or `Result<(), Error>`.
    - Use extractors: `Context`, `TangleArg`, `TangleArgsN`, `BlockEvents`.
    - Return `TangleResult<T>` for jobs submitting results back to Tangle.
    - Use `#[debug_job]` macro for automatic entry/exit logging.
    - Group related jobs into modules (e.g., `jobs/mod.rs`, `jobs/create_project.rs`).

4.  **Keystore & Signers:**

    - Load keys and signers from `BlueprintEnvironment` (`env.keystore()`).
    - Use `TanglePairSigner` for Tangle interactions.
    - Handle BLS keys appropriately for Eigenlayer blueprints if applicable.

5.  **Producer/Consumer:**
    - Select compatible producers (`TangleProducer`, `PollingProducer`, `CronJob`, `RoundBasedAdapter`) and consumers (`TangleConsumer`, `EVMConsumer`) based on event source and result destination.
    - Remember: `TangleConfig` determines result submission, not event source.

**Enforcement:**

- **MUST** adhere strictly to naming conventions.
- **MUST** use Blueprint SDK patterns for runner, router, jobs, and context.
- **MUST NOT** use `TangleConsumer` or `TangleProducer` outside Tangle-specific blueprints (unless explicitly justified for hybrid scenarios).
- **MUST** handle errors gracefully using `Result`.
