# Implementation: cargo-dist installer + README

## Behaviour
../usecase.md

## Design Decisions
- This behaviour is fully satisfied by two upstream implementations — no new source files are required:
  1. `../../cargo-dist-release-pipeline/github-actions/impl.md` — produces the installer script (`rpncalc-installer.sh`) and platform binary archives uploaded to each GitHub Release; the installer handles OS/arch detection, archive download, SHA-256 verification, extraction to `~/.cargo/bin/`, and PATH reminder (satisfies AC-2, AC-3, AC-4, AC-5)
  2. `../../project-readme/readme/impl.md` — documents the curl install command that users copy; AC-1 explicitly requires the command to come from the README
- The installer script is generated and managed by cargo-dist; it is not hand-maintained in this repository
- AC-6 (no OWNER placeholders) is a release-gate concern tracked in the README impl; it is not re-tested here

## Source Files
- *(none — this behaviour is implemented by the release pipeline and README)*

## Commits
<!-- taproot link-commits will fill this -->

## Tests
All ACs are satisfied by upstream implementations:
- **AC-1** (install command in README): covered by `project-readme/readme` impl
- **AC-2** (correct platform binary): covered by `cargo-dist-release-pipeline/github-actions` impl — the installer detects OS/arch and selects the matching archive
- **AC-3** (unsupported platform error): covered by `cargo-dist-release-pipeline/github-actions` impl — the cargo-dist installer script exits with an error on unsupported platforms
- **AC-4** (checksum mismatch aborts): covered by `cargo-dist-release-pipeline/github-actions` impl — the installer verifies SHA-256 before extracting
- **AC-5** (PATH reminder shown): covered by `cargo-dist-release-pipeline/github-actions` impl — the cargo-dist installer prints PATH guidance when `~/.cargo/bin/` is not on PATH

## DoR Resolutions

## Status
- **State:** complete
- **Created:** 2026-03-24
- **Last verified:** 2026-03-24
