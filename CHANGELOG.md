# Changelog

All notable changes to Argent are documented here. Format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions per
`Cargo.toml`. Tags: `v<version>` after merge.

## [Unreleased]

### Added
- **Portfolio certification rollout:** `README.md` (overview, quick start,
  build, test, usage, project layout), `CONTRIBUTING.md` (PR-flow discipline:
  draft PR → tests green → owner merges; no direct pushes to `master`;
  `CHANGELOG` entry under Unreleased per PR; semver bump; merge commits
  reference PR numbers; releases tagged `vX.Y.Z`), `CHANGELOG.md`, and
  `ATTRIBUTION.md`.

### Changed
- Version bump `0.1.0` → `0.1.1` (chore): `Cargo.toml`, `configs/config.toml`,
  `container/build.sh` (`VERSION`), `container/Dockerfile` (`LABEL version`).

### Notes
- No deploy target found: `container/build.sh` and the Makefile `container`
  targets build a local Docker image only — nothing pushes to a registry,
  and there is no wrangler/Pages/release-publish path. Skipped track 2; if a
  registry publish step is added later it should be wired to the signed-deploy
  wrapper before first use.
- `LICENSE` is missing — license choice flagged to the owner in the PR body;
  no license headers added to source files until the license is declared.
