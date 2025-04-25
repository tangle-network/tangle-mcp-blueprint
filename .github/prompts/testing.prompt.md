# AI Prompt: Blueprint Testing Requirements

**Objective:** Ensure all Blueprint jobs are accompanied by robust integration tests using the appropriate testing frameworks.

**General Requirements:**

1.  **Location:** Tests **MUST** reside in `blueprint/tests/` or within job modules at `blueprint/src/jobs/tests.rs`.
2.  **Coverage:** Every job handler (`fn` routed in `main.rs`) **MUST** have at least one corresponding integration test.
3.  **Framework:**
    - For Tangle Blueprints: **MUST** use `TangleTestHarness`.
    - For EVM/Hybrid Blueprints: Use `Anvil` + `alloy` or `TangleTestHarness` depending on the primary interaction mode.
    - For Docker interactions: Tests should interact with a local Docker daemon via `bollard` (often implicitly through `docktopus` within the test).

**TangleTestHarness Usage (`blueprint/tests/`):**

1.  **Setup:**

    - Use `#[tokio::test]` attribute.
    - Create a temporary directory: `let temp_dir = tempfile::tempdir()?;`.
    - Initialize the harness: `let harness = TangleTestHarness::setup(temp_dir.path()).await?;`.
    - Set up services: `let (mut test_env, service_id, _) = harness.setup_services::<N>(is_multi_tenant).await?;` (where `N` is the number of services).
    - Initialize the test environment: `test_env.initialize().await?;`.

2.  **Job Registration:**

    - Register the job handler(s) with the test environment's router: `test_env.add_job(my_job_handler.layer(TangleLayer)).await;`. Ensure the layer matches the one used in `main.rs`.

3.  **Execution:**

    - Start the test environment runner: `test_env.start(()).await?;` (or provide a shutdown future).
    - Submit a job call: `let call = harness.submit_job(service_id, JOB_ID, vec![InputValue::Uint64(5)]).await?;`. Match `InputValue` types to the job signature.
    - Wait for the job to execute: `let result = harness.wait_for_job_execution(service_id, call).await?;`.

4.  **Verification:**
    - Verify the job result: `harness.verify_job(&result, vec![OutputValue::Uint64(25)]);`. Match `OutputValue` types and values to the expected outcome.
    - Assert success: `assert!(result.is_success());` or check for specific errors if testing failure cases.
    - Add assertions for any side effects (e.g., check files created in `temp_dir`, query Docker container status if applicable).

**Docker Container Testing:**

- If jobs interact with Docker (via `docktopus`), tests **MUST** verify the container lifecycle:
  - Creation, start, status checks, stop, removal.
  - Use `bollard` directly or `docktopus` methods within the test setup or assertions to interact with the local Docker daemon.
  - Ensure proper cleanup (container removal) even if tests fail (use `defer` or RAII patterns).
- Example (`docktopus.md`):
  ```rust
  #[tokio::test]
  async fn container_lifecycle_test() -> Result<(), docktopus::container::Error> {
      let docker = DockerBuilder::new().await?; // Connects to local Docker
      let mut container = Container::new(docker.client(), "alpine:latest")
          .cmd(["echo", "test"])
          .create().await?;
      container.start(true).await?; // Start and wait
      assert_eq!(container.status().await?, Some(ContainerStatus::Exited));
      container.remove(None).await?;
      Ok(())
  }
  ```

**Enforcement:**

- **MUST** include integration tests for all job handlers.
- **MUST** use `TangleTestHarness` for Tangle Blueprints.
- **MUST** follow the setup -> register -> execute -> verify pattern.
- **MUST** clean up resources (like Docker containers) created during tests.
- **MUST** verify both job results (`OutputValue`) and any significant side effects.
