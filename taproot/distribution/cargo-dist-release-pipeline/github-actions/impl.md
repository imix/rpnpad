# Implementation: cargo-dist GitHub Actions Release Pipeline

## Behaviour
../usecase.md

## Design Decisions
- `cargo-dist` was not installed in the implementation environment; `Cargo.toml` config and `.github/workflows/release.yml` are written manually to match `cargo dist init` output for v0.22.x. The maintainer can run `cargo dist init` after installing cargo-dist to regenerate from scratch if the workflow drifts from the tool's latest output.
- Homebrew tap is configured as a placeholder (`OWNER/homebrew-tap`) — the maintainer must create the tap repository and update this value before the first release that targets Homebrew.
- crates.io publishing is disabled (`publish = false`) — can be opted in by setting `publish = ["crates-io"]` in `[workspace.metadata.dist]`.
- `fail-fast: true` on the build matrix — all target platforms must succeed for a release to be created, satisfying AC-6 fail-fast semantics.
- Workflow pins cargo-dist at `v0.22.1` via the shell installer — pinning avoids unexpected breakage from upstream changes; bump the version string to upgrade.
- Linux runners use `ubuntu-latest` — `ubuntu-20.04` reached end-of-life April 2025 and was retired from GitHub Actions hosted runners.
- The workflow uses `actions/checkout@v4`, `actions/upload-artifact@v4`, and `actions/download-artifact@v4` — current stable major versions.
- GitHub Release is created as a draft first, then published after all assets are uploaded — prevents users from seeing an incomplete release.

## Source Files
- `Cargo.toml` — `[workspace.metadata.dist]` section added: targets, CI backend, Homebrew tap, installer types
- `.github/workflows/release.yml` — GitHub Actions release workflow: plan → build (3-platform matrix) → publish release + Homebrew formula

## Commits
<!-- taproot link-commits will fill this -->
- `2267a1569f8896382799b36e30172af1065fcb7d` — (auto-linked by taproot link-commits)
- `cfa4c637f466ad8edd679d9c5b18a7627bea57a7` — (auto-linked by taproot link-commits)

## Tests
This is a CI/CD pipeline configuration. All ACs are integration tests verified against live GitHub Actions runs — no local unit tests are applicable.

- **AC-1** (tag triggers pipeline): push a version tag matching `Cargo.toml`; observe workflow triggers and completes without manual steps
- **AC-2** (release assets present): inspect GitHub Release page after successful run; verify `.tar.gz` archives for all 3 targets, SHA-256 checksums, and `install.sh` are present
- **AC-3** (Homebrew formula updated): inspect the tap repository after successful run; verify formula version, URL, and checksum are updated
- **AC-4** (build failure blocks release): introduce a compile error, push a tag; verify workflow fails and no release is published
- **AC-5** (no tap configured → graceful skip): comment out Homebrew config, push a tag; verify release is published and Homebrew step is skipped without error
- **AC-6** (version mismatch blocks release): push a tag whose version differs from `Cargo.toml`; verify workflow fails before creating any release

## DoR Resolutions

## Status
- **State:** complete
- **Created:** 2026-03-24
- **Last verified:** 2026-03-26
