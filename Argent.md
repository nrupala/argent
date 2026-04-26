# Project Argent: Hardened Autonomous Engine

## Vision
A zero-knowledge, autonomous coding agent built in Rust, utilizing WASM for cross-platform sandboxing and high-speed, encrypted execution.

## Core Directives
- **Identity**: Argent.
- **Base Language**: Rust (Static binaries).
- **Execution Boundary**: WASI (WebAssembly System Interface).
- **Security Protocols**: 
  - Zero-Knowledge local state via `XChaCha20-Poly1305` encryption.
  - No direct syscalls; all I/O must be proxied through the Argent-WASM bridge.
  - Zero-Trust networking via TLS 1.3 certificate pinning.

## Component Map
1. **Argent-Core**: The Rust-based event loop (Tokio).
2. **Argent-Vault**: The encrypted local memory and session store.
3. **Argent-Bridge**: The WASM runtime (Wasmtime) that executes inferred code.
4. **Argent-Vision**: The MIME/multimodal processing module (`image` & `ffmpeg` crates).

## Development Workflow
1. Initialize project with `cargo init --bin`.
2. Implement the WASM execution sandbox first to ensure safety.
3. Integrate the ReAct loop for unattended code generation and testing.

# Autonomous Coding Engine Blueprint: "Core-Hardened Agent" called Argent

## Goal
Build a cross-platform, autonomous coding engine in Rust that runs in a zero-trust, containerized WASM environment.

## Core Architectural Rules
- **Language**: Strictly use **Rust** for the core binary and logic.
- **Isolation**: Use **Wasmtime/WASI** for sandboxed execution. No direct host access.
- **Security**: 
  - Implementation must be **zero-knowledge**. Use `RustCrypto` for local AES-256-GCM encryption of all code and state.
  - Enforce **TLS 1.3** for all LLM API calls.
- **MIME Processing**: Use the `image` and `ffmpeg-next` crates to handle multimodal inputs safely before inference.

## Autonomous Operation (Unattended)
- Use the `--dangerously-skip-permissions` flag during execution to allow unattended task completion.
- Maximize reliability by writing **Plan-and-Execute** roadmaps before every major code change.

## Technical Stack Requirements
1. **Runtime**: `tokio` for high-concurrency event loops.
2. **Tools**: Implement [Model Context Protocol (MCP)](https://modelcontextprotocol.io) in Rust for standardized tool use.
3. **Containerization**: Target OCI-compliant lightweight micro-VMs (Firecracker) for hosting.

## Testing & Verification
- No code change is complete without a successful `cargo test` and `cargo fmt`.
- Automated security audits must be part of every build cycle.

## Bootstrapping Sequence
1. Generate `Cargo.toml` with dependencies: `tokio`, `wasmtime`, `aes-gcm` or `chacha20poly1305`, `rustls`, `image`, and `mcp-sdk-rust`.
2. Create `src/main.rs` as the entry point for the **Argent-Core** event loop.
3. Establish the `src/sandbox/` module to initialize the Wasmtime engine with restricted WASI capabilities.
4. Setup `src/crypto/` for the zero-knowledge vault implementation.

## Tooling & MCP Constraints
- All "Tools" (File I/O, Terminal, Network) must be implemented via the **Model Context Protocol (MCP)**.
- The Agent must never call `std::fs` directly. It must request a tool call to the `Argent-Bridge`, which validates the path against a whitelist before execution.
## Persistence & Recovery
- Implement a **State Machine** that serializes the current execution plan to the `Argent-Vault` after every step.
- On startup, Argent must check for an unfinished plan and prompt to "Resume" or "Rollback."
## LLM Orchestration
- The engine must use a **ReAct (Reasoning + Acting)** pattern.
- Before any code modification, the agent must output a `thought` block explaining the security implications of the change.
- Mandatory: The agent must verify its own work by generating and running a test suite for every new feature.
## Multimodal Inference
- When a MIME input (Image/PDF) is received, Argent must:
  1. Hash the file for integrity.
  2. Use the `Argent-Vision` module to extract text/layout/UI metadata.
  3. Append this metadata to the LLM context as an "Environmental Observation."
## Cargo Manifest Directives
- **Async**: `tokio` (full features)
- **WASM**: `wasmtime`, `wasi-common`
- **Crypto**: `ring`, `chacha20poly1305`
- **Serialization**: `serde`, `serde_json`
- **Terminal**: `ratatui` (for the CLI dashboard)

//Instructions to opencode: "Read ARGENT.md. Start by scaffolding the Rust project structure. Initialize the Argent-Core with a Tokio loop and create a placeholder module for the Argent-Bridge WASM runtime. Do not implement host-level file access yet; focus on the secure communication layer."
