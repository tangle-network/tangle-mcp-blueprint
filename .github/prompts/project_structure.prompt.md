# AI Prompt: Project Structure Guidelines

**Objective:** Ensure all generated or modified code adheres strictly to the Tangle Blueprint project structure.

**Mandatory Rules:**

1.  **Root Directory Structure:** Adhere _exactly_ to this layout:

    ```bash
    .
    ├── .cargo/
    ├── .github/
    ├── app/         # Optional: Frontend app
    ├── contracts/   # Solidity contracts
    ├── bin/         # Binary crate (main.rs only)
    │   └── src/
    │       └── main.rs
    ├── blueprint/   # Library crate (all logic)
    │   ├── src/
    │   │   └── lib.rs
    │   └── tests/
    ├── .dockerignore
    ├── .envrc
    ├── .gitignore
    ├── Cargo.lock
    ├── Cargo.toml
    ├── Dockerfile   # Optional: Runtime setup
    ├── LICENSE-APACHE
    ├── LICENSE-MIT
    └── README.md
    ```

2.  **Crate Responsibilities:**

    - `bin/`: **ONLY** for `BlueprintRunner` initialization and environment setup. **NO** application logic.
    - `blueprint/`: Contains **ALL** jobs, context structs, utilities, tests, producers, consumers. Expose modules via `blueprint/src/lib.rs`.

3.  **File Placement:**

    - Place `BlueprintRunner` setup in `bin/src/main.rs`.
    - Create job modules under `blueprint/src/jobs/` (e.g., `blueprint/src/jobs/create_project.rs`).
    - Define the main `Context` struct in `blueprint/src/context.rs`.
    - Keep Solidity contracts strictly within the `/contracts` directory.
    - Place integration tests in `blueprint/tests/` or `blueprint/src/jobs/tests.rs`.

4.  **Special Files:**

    - `README.md`: Must include project summary, job descriptions, and testing instructions (`TangleTestHarness`).
    - `Dockerfile` (Optional): If present, must support the blueprint runtime environment.

5.  **Frontend (Optional):**
    - If `/app/` exists, it must be self-contained and not interfere with the blueprint layer.

**Enforcement:**

- **DO NOT** place any business logic, job handlers, or context definitions in the `bin` crate.
- **MUST** follow the `kebab-case` for folders/crates and `snake_case` for Rust files convention.
- **MUST** initialize projects using `cargo tangle blueprint create --name <service-name>`.
