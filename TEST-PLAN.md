# Test plan — argent

## Scope

Testing covers the `argent` library and binary: unit tests inside the
main crate (vault cryptography round-trip in `src/crypto.rs`, prompt
construction in `src/prompts.rs`) and the separate
`FullIntegrationTest` crate, which exercises the vault, tool registry,
WASM sandbox, MCP model detection / path guard, LLM client and full
system integration against the library as a path dependency. The LLM
tests are written headlessly: they construct the LM Studio client and
tolerate the expected connection error when no LM Studio server is
running, so no live model service is required in CI. Live end-to-end
agent runs against a real GGUF model are out of scope for CI.

## Suites

| Suite | Covers | Run it | Runs in CI |
|---|---|---|---|
| Unit tests (main crate, `#[cfg(test)]` modules) | Vault encrypt/decrypt round-trip; prompt/thought construction | `cargo test` (repo root) | Yes — CI workflow |
| Integration tests (`FullIntegrationTest/tests/`: crypto, tools, llm, sandbox, mcp, integration) | Vault initialisation / encryption / state persistence; tool registry and blocklist; sandbox initialisation and limits; path guard; model detection; LLM client creation and request serialisation (headless-tolerant) | `cargo test` in `FullIntegrationTest/` | Yes — CI workflow |

`FullIntegrationTest/TEST_RESULTS.md` records the suite's last local
run (2026-04-26: 16 passed, 1 ignored — a state-persistence test
ignored for a known issue being investigated).

**Known state (2026-11-06, recorded when CI was wired):** the
`FullIntegrationTest` suite no longer compiles against the current
library API — it references items that have since been removed or
made private (`mcp::SecurityLevel`, the formerly public
`LlmClient.endpoint`, `ArgentSandbox::get_directory` /
`get_max_memory`). The main crate's own suite passes. Separately,
`cargo clippy -- -D warnings` currently fails on a small set of
mechanical lints in the library. Both are product-side drift,
recorded here rather than smoothed; CI is red until the maintainer
resolves them, and no release is tagged over that state.

## CI gates

- CI workflow (`.github/workflows/ci.yml`) on every push and pull
  request to `master`: `cargo clippy -- -D warnings` (the lint bar the
  project's own Makefile declares), then both test suites must pass.
- CodeQL workflow (`.github/workflows/codeql.yml`, Rust, build-mode:
  none) on push, pull request and a weekly schedule; findings are
  dispositioned per the house alert SLA (fix, or a dismissal with the
  reason recorded).

## Cadence

- Every pull request: clippy + both suites, and CodeQL.
- Weekly: scheduled CodeQL run.
- Every release candidate: the release-readiness run (checker +
  evidence reads), per the house Release-Readiness Standard.

## Coverage bar

No numeric floor; new functionality lands with tests per CONTRIBUTING.
