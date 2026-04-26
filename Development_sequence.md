Since OpenCode performs best when tasks are broken into "digestible" technical milestones, you should provide these four markdown files in order. As soon as OpenCode finishes the instructions in one file, give it the next.
Phase 1: The Foundation (ARGENT_01_CORE.md)
Input this first. This forces OpenCode to set up the workspace, dependencies, and the encrypted "Vault" so that all following work is secure.
# Milestone 1: Core & Cryptography
**Objective**: Setup the Rust workspace and the Zero-Knowledge Vault.

1. **Initialize**: Run `cargo init` for a binary project.
2. **Dependencies**: Update `Cargo.toml` with: `tokio`, `chacha20poly1305`, `serde`, `serde_json`, `anyhow`, and `ring`.
3. **Vault Implementation**: 
   - Create `src/crypto.rs`. 
   - Implement a struct `ArgentVault` that uses `XChaCha20-Poly1305` to encrypt/decrypt local strings and files.
   - Ensure a local `.argent_key` is used for the master key (stored outside the repo).
4. **Main Loop**: Setup `src/main.rs` with a `tokio::main` async loop that initializes the Vault.

Phase 2: The Sandbox (ARGENT_02_SANDBOX.md)
Input this second. This builds the "isolation ward" where the autonomous agent will actually perform its coding tasks.

# Milestone 2: WASM Sandbox (Argent-Bridge)
**Objective**: Build the Wasmtime/WASI execution environment.

1. **Dependencies**: Add `wasmtime` and `wasi-common` to `Cargo.toml`.
2. **Sandbox Setup**: 
   - Create `src/sandbox.rs`.
   - Implement a function `init_sandbox()` that creates a `Wasmtime` engine.
   - **Constraint**: Configure WASI to have NO host access except for a specific `/workspace` virtual directory.
3. **Execution Logic**: Implement a method to take a string of code, compile it to WASM (or run a pre-compiled component), and return the STDOUT/STDERR.

Phase 3: The Brain (ARGENT_03_AGENT.md)
Input this third. This gives Argent its "Reasoning" loop (ReAct) so it can think, act, and observe.
# Milestone 3: The ReAct Engine
**Objective**: Implement the autonomous autonomous coding loop.

1. **Prompt Engineering**: Create `src/prompts.rs` containing the System Prompt for Argent. Define it as a "Self-Correcting Autonomous Engineer."
2. **Tool Definition**: Implement a tool-calling handler that maps LLM requests to:
   - `read_file`, `write_file`, `run_command`.
3. **Loop Logic**: Implement the "Think-Act-Observe" loop in `src/main.rs`. 
   - After every tool call, the result must be fed back to the LLM.
   - If a command fails (Observation), Argent must generate a fix (Next Thought).

Phase 4: Multimodal & Deployment (ARGENT_04_MIME.md)
Input this last. This enables the agent to "see" and run on mobile/desktop uniformly.
# Milestone 4: Vision & Portability
**Objective**: Add MIME processing and static compilation.

1. **Vision Module**:
   - Create `src/vision.rs` using the `image` crate.
   - Implement logic to downsample/preprocess screenshots into a format readable by multimodal LLMs.
2. **Persistence**: Ensure the ReAct state is saved to the `Argent-Vault` every 5 seconds.
3. **Cross-Platform**: Configure a `Makefile` or `justfile` to compile static binaries for `x86_64-unknown-linux-musl`, `aarch64-apple-darwin`, and Windows.

Phase 5: Communication Layer (ARGENT_05_LLM.md)
Objective: Connect the engine to the LLM providers with zero-knowledge security.
# Milestone 5: Secure LLM Integration
**Objective**: Implement the encrypted API client and context management.

1. **Client Setup**:
   - Create `src/llm.rs` using `reqwest` and `rustls`.
   - Implement **Certificate Pinning** to ensure zero-trust communication with the LLM provider.
2. **Context Compression**:
   - Implement a "Rolling Window" manager to handle large codebases without hitting token limits.
   - Summarize previous steps into the `Argent-Vault` to maintain long-term memory.
3. **Secret Management**:
   - API keys must never be stored in plaintext. Implement a prompt to ingest keys into the `Argent-Vault` with memory-only residence where possible.

Phase 6: Model Context Protocol (ARGENT_06_TOOLS.md)
Objective: Standardize tool use so Argent can use any MCP-compatible plugin.
markdown
# Milestone 6: MCP Tooling & Filesystem
**Objective**: Implement the Model Context Protocol (MCP) in Rust.

1. **Protocol Implementation**:
   - Create `src/mcp.rs`. Implement the JSON-RPC layer for MCP.
   - Map standard coding tools: `list_directory`, `search_code`, `apply_patch`.
2. **Atomic Writes**: 
   - Ensure `apply_patch` is atomic. If a file write fails mid-way, Argent must use the Vault to roll back to the previous state.
3. **Safety Filters**:
   - Implement a "Path Guard" that prevents the agent from accessing sensitive host directories (e.g., `.ssh`, `/etc`) even if the sandbox has a leak.

Phase 7: The Interface & CLI (ARGENT_07_CLI.md)
Objective: Build the "Claude Code" style terminal interface.

# Milestone 7: The Argent CLI
**Objective**: Create the user-facing terminal interface and global triggers.

1. **Terminal UI (TUI)**:
   - Use the `ratatui` crate to create a split-screen dashboard.
   - Left side: Live "Thinking" logs. Right side: File changes/diffs.
2. **Commands**:
   - Implement `/init` to index a new project.
   - Implement `/clear` to wipe the session vault.
   - Implement `argent --unattended` for headless CI/CD execution.
3. **Binary Packaging**:
   - Use `cargo-dist` configuration to generate installers for Windows, macOS, and Linux.

Phase 8: Final Integration & Self-Test (ARGENT_08_FINAL.md)
Objective: The "Bootstrap" test where Argent validates its own build.
# Milestone 8: Full System Integration
**Objective**: Final wiring and autonomous verification.

1. **Wiring**: Connect `Argent-Core`, `Bridge`, `Vault`, and `LLM` into a single async flow.
2. **Autonomous Test**: 
   - Command Argent to "Add a new feature to yourself" (e.g., a simple telemetry toggle).
   - The engine must: Plan -> Code -> Test -> Verify -> Commit.
3. **Security Audit**:
   - Run `cargo audit` and ensure all dependencies are patched.
   - Verify that the `Argent-Vault` remains encrypted even if the process is force-quit.

Thus: 
The Full Sequence for OpenCode:

    ARGENT_01_CORE.md (Foundation)
    ARGENT_02_SANDBOX.md (Isolation)
    ARGENT_03_AGENT.md (Reasoning)
    ARGENT_04_MIME.md (Multimodality)
    ARGENT_05_LLM.md (Communication)
    ARGENT_06_TOOLS.md (Tooling)
    ARGENT_07_CLI.md (Interface)
    ARGENT_08_FINAL.md (Verification)

Pro-tip for OpenCode: Before starting, tell it: "You are developing Project Argent. Maintain a shared state across all milestones. Refer to ARGENT.md as your primary directive at all times."