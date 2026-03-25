# Implementation: snapcraft

## Behaviour
../usecase.md

## Design Decisions
- `confinement: classic` required for full terminal access (stdin/stdout, terminal control sequences) — strict confinement blocks these; classic requires Canonical manual review before publish to Snap Store
- Built from source using the `rust` snapcraft plugin rather than dumping a pre-built binary — more maintainable, avoids architecture-specific URL management, and snapcraft handles cross-arch builds naturally
- `base: core22` (Ubuntu 22.04 LTS) — current stable base with long support horizon; avoids `core18` (deprecated) and `core24` (newer, less tested)
- `grade: stable` — published to the stable channel; users get it by default without specifying a channel
- Version sourced from `Cargo.toml` via `adopt-info` — keeps snap version in sync with crate version automatically
- GitHub Actions integration: `snapcraft` can be invoked in the release workflow to build and publish the snap after the cargo-dist release succeeds

## Source Files
- `snap/snapcraft.yaml` — snap build definition: name, version, confinement, build steps

## Commits
- placeholder

## Tests
Integration tests require snapd and Snapcraft:

- **AC-1** (fresh install): publish snap to Snap Store edge channel; `snap install rpncalc --channel=edge`; verify `rpncalc` on PATH and `snap list` shows version
- **AC-2** (auto-refresh): install from edge, publish new revision; wait for auto-refresh or `snap refresh rpncalc`; verify new version
- **AC-3** (manual refresh): `snap refresh rpncalc`; verify latest revision installed
- **AC-4** (not found): `snap install rpncalc` before publishing; verify "snap not found" error
- **AC-5** (classic confinement): run rpncalc in terminal after install; verify stdin/stdout work without confinement errors

## DoR Resolutions

## Status
- **State:** complete
- **Created:** 2026-03-25
- **Last verified:** 2026-03-25

## Notes
- Classic confinement approval from Canonical may take days to weeks. During the review period, use `snap install --devmode rpncalc` for testing.
- A Snapcraft account (snapcraft.io) is required to publish. Register the snap name `rpncalc` via `snapcraft register rpncalc` before first publish.
- The snap name `rpncalc` may already be taken on the Snap Store — check before submitting.
