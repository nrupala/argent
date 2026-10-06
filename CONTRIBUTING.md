# Contributing to Argent

Thank you for your interest in contributing to Argent!

## PR-flow discipline (required)

- **Draft PRs only.** Every change ships as a draft pull request against
  `master`; direct pushes to `master` are retired. CI must be green before
  review, and the owner merges when ready.
- **No direct pushes to `master`.**
- **CHANGELOG entry.** Every PR adds an entry under `## [Unreleased]` in
  `CHANGELOG.md` describing the change.
- **Version bump.** `Cargo.toml` version follows semver: `patch`=fix,
  `minor`=feature, `major`=breaking. Keep `configs/config.toml`,
  `container/build.sh` (`VERSION`), and `container/Dockerfile` (`LABEL
  version`) consistent with it.
- **Merge commits reference the PR number.** Releases are tagged `vX.Y.Z`
  after merge.

## Development setup

```bash
# Clone the repository
git clone https://github.com/nrupala/argent.git
cd argent

# Build
make build        # cargo build --release

# Run the tests
make test         # cargo test

# Quality gates (must all pass before a PR leaves draft)
make fmt          # cargo fmt --check
make clippy       # cargo clippy -- -D warnings
make audit        # cargo audit
```

## Code style

- Rust: `cargo fmt` formatting, clippy with warnings denied (`make clippy`).
- Keep changes minimal and focused; one concern per PR.

## Committing

Use conventional commits (`feat:`, `fix:`, `docs:`, `chore:`), e.g.
`fix: harden sandbox path blocklist`.

## Code of conduct

- Be respectful and inclusive.
- Provide constructive feedback.
- Welcome newcomers.

## License note

Argent currently has **no declared license** (flagged to the owner in the
certification PR). By contributing, you agree your contributions are made
under whatever license the owner selects; the `LICENSE` file, once added,
is the final word.
