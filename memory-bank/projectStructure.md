## 🧭 Tangle Blueprint Project Structure

> These rules are mandatory for all Blueprint projects generated or modified by AI. The structure is non-negotiable unless explicitly overridden by a user instruction.

---

### 1. 🗂 Required Directory and File Structure

Every Blueprint must follow this layout exactly at the root level:

```bash
.
├── .cargo/
├── .github/
├── app/                       # Front-end app monorepo or impl
├── contracts/                 # Solidity contracts used for job triggers or verification
├── bin/                       # Binary crate, entry point for blueprint execution
│   └── src/
│       └── main.rs
├── blueprint/                 # Library crate, contains logic, jobs, context, tests
│   ├── src/
│   │   └── lib.rs
│   └── tests/
├── .dockerignore
├── .envrc
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── Dockerfile                 # Optional runtime container setup
├── LICENSE-APACHE
├── LICENSE-MIT
└── README.md
```

> 🔒 **File placement is enforced. AI must not create Rust logic outside the `blueprint` crate. `main.rs` only initializes the runner.**

---

### 2. 🧰 Crate Responsibilities

#### bin/

- Initializes the Blueprint environment and runner.
- Registers producers, consumers, router, and context.
- No application logic allowed.

#### blueprint/

- All jobs, context structs, consumers, producers, utilities, and tests reside here.
- Use `src/lib.rs` to expose modules such as `jobs`, `context`, `utils`.

---

### 3. 📄 Special File Requirements

#### `README.md`

- Must include:
  - Summary of blueprint service w/ directory structure and breakdown of app, service, smart contract system.
  - Description of job responsibilities.
  - Instructions for testing using `TangleTestHarness`.

#### `Dockerfile`

- Optional.
- If included, must support blueprint runtime (e.g., installing `solc`, `forge`, or `cargo`).

---

### 4. 🛠 Contracts Directory (`/contracts`)

- Contains Solidity contracts used by the Blueprint service.
- All contracts must compile using `forge build` or a supported browser compiler.

---

### 5. 🌐 Optional Frontend

- A `/frontend/` directory may be added if the Blueprint includes a user interface.
- Must be a self-contained app (e.g., Vite + TS + Tailwind).
- Must not interfere with the blueprint execution layer.
- If present, frontend build artifacts should be ignored by `.gitignore`.

---

### 6. 🧪 Testing Requirements

- All jobs must include at least one integration test using `TangleTestHarness`.
- Tests must live in `blueprint/tests/` or `blueprint/src/jobs/tests.rs`.
- Tests must:
  - Initialize a test harness.
  - Register the job.
  - Submit inputs and validate results.

---

### 7. ✨ Naming Conventions

- All crates and folders use `kebab-case`.
- All Rust files use `snake_case`.
- All exported types use `PascalCase`.
- All constants (e.g., job IDs) use `SCREAMING_SNAKE_CASE`.

---

### 8. 📦 Valid Project Initialization

Blueprints must be initialized using the following canonical command:

```bash
cargo tangle blueprint create --name my_service
```

This ensures:

- Proper bin`and`blueprint` separation
- Hook and Dockerfile generation

---

### 9. 🧠 AI Enforcement Rules

- ✅ MUST place `BlueprintRunner` setup in `main.rs`.
- ✅ MUST create one module per job in `blueprint/src/jobs/`.
- ✅ MUST define a `Context` struct in `blueprint/src/context.rs`.
- ✅ MUST keep smart contract code isolated in `/contracts`.
- ❌ MUST NOT place any Blueprint logic in the `bin` crate besides initialization.
