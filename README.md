# Argent

**Zero-knowledge autonomous coding agent with WASM sandboxing.**

Argent is a Rust-based autonomous coding agent: a zero-knowledge,
hardened engine that executes agent actions inside a WebAssembly
sandbox, with encrypted local storage. See `Argent.md` for the vision
statement and `Development_sequence.md` for the build milestones.

## Quick start

```bash
# 1. Build the release binary
make build        # or: cargo build --release

# 2. Set the vault key (required by the encrypted storage)
export ARGENT_KEY="$(cat ~/.argent/key)"

# 3. Run
./target/release/argent --help
```

Default configuration lives in `configs/config.toml`; the container
image copies it to `/home/argent/.argent/`.

## Build

Prerequisites: Rust toolchain (the container build uses `rust:1.86-bookworm`).

```bash
make build        # cargo build --release
make container    # docker build -t argent:latest -f container/Dockerfile .
make container-build  # bash container/build.sh — Docker image + trivy scan + smoke test
```

## Test

```bash
make test         # cargo test
```

Full integration tests live in `FullIntegrationTest/` (16 core tests; see
`FullIntegrationTest/README.md`, `TEST_RESULTS.md`, `DETAILED_TESTS.md`).

## Usage

```bash
make run          # not defined — run ./target/release/argent directly
make container-run  # docker run -it argent:latest
```

Environment:

| Variable | Purpose |
|---|---|
| `ARGENT_KEY` | Key for the encrypted vault (`security/key_store`) |
| `ARGENT_STORAGE` | Storage directory (container default: `/home/argent/.argent`) |
| `RUST_LOG` | Log level (container default: `info`) |

Code quality gates used by this repo:

```bash
make fmt          # cargo fmt --check
make clippy       # cargo clippy -- -D warnings
make audit        # cargo audit
```

## Project layout

| Path | Purpose |
|---|---|
| `src/` | Engine source: `crypto`, `sandbox`, `llm`, `mcp`, `tools`, `prompts` |
| `configs/config.toml` | Container/runtime configuration |
| `container/` | Dockerfile, image build script, Firecracker config |
| `FullIntegrationTest/` | Integration test workspace (results + docs) |
| `Argent.md` | Vision statement |
| `Development_sequence.md` | Milestone build instructions |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) — draft PR → tests green → owner
merges; no direct pushes to `master`; every PR adds a `CHANGELOG` entry
and bumps the version (patch).

## License

**License status is undeclared — see the flag in the certification PR.**
No `LICENSE` file is present and none is declared in `Cargo.toml`; license
choice is the owner's decision. Source files therefore carry no license
headers yet.
