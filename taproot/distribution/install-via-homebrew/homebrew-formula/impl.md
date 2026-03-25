# Implementation: cargo-dist Homebrew formula + README

## Behaviour
../usecase.md

## Design Decisions
- This behaviour is fully satisfied by two upstream implementations — no new source files are required:
  1. `../../cargo-dist-release-pipeline/github-actions/impl.md` — the release pipeline generates the Homebrew formula (`rpncalc-installer.sh` and the tap formula file) and uploads binary archives; Homebrew handles OS/arch selection, checksum verification, and PATH linking (satisfies AC-2, AC-3, AC-4, AC-5)
  2. `../../project-readme/readme/impl.md` — documents the `brew install imix/tap/rpncalc` command and the `brew tap` + `brew install` alternate path that users copy (satisfies AC-1)
- The formula is generated and managed by cargo-dist; it is not hand-maintained in this repository
- `OWNER` placeholder in the usecase notes is resolved: the real tap is `imix/tap` (Cargo.toml: `tap = "imix/homebrew-tap"`)

## Source Files
*(none — this behaviour is implemented by the release pipeline and README)*

## Commits
<!-- taproot link-commits will fill this -->

## Tests
All ACs are satisfied by upstream implementations:
- **AC-1** (brew install command in README): covered by `project-readme/readme` impl
- **AC-2** (correct platform binary): covered by `cargo-dist-release-pipeline/github-actions` impl — Homebrew selects the matching bottle from the formula's platform-specific URLs
- **AC-3** (upgrade installs new version): covered by `cargo-dist-release-pipeline/github-actions` impl — each new release updates the formula version; `brew upgrade` picks it up
- **AC-4** (tap not found gives actionable error): covered by `cargo-dist-release-pipeline/github-actions` impl — Homebrew's built-in error handling surfaces the unknown tap message
- **AC-5** (checksum mismatch aborts): covered by `cargo-dist-release-pipeline/github-actions` impl — the formula embeds the SHA-256 generated during the release; Homebrew verifies before installing

## DoR Resolutions

## Status
- **State:** complete
- **Created:** 2026-03-25
- **Last verified:** 2026-03-25
