# Implementation: snapcraft

## Behaviour
../usecase.md

## Design Decisions
- `confinement: classic` required for full terminal access (stdin/stdout, terminal control sequences) — strict confinement blocks these; classic requires Canonical manual review before publish to Snap Store
- Built from source using the `rust` snapcraft plugin rather than dumping a pre-built binary — more maintainable, avoids architecture-specific URL management, and snapcraft handles cross-arch builds naturally
- `base: core22` (Ubuntu 22.04 LTS) — current stable base with long support horizon; avoids `core18` (deprecated) and `core24` (newer, less tested)
- `grade: stable` — published to the stable channel; users get it by default without specifying a channel
- Version sourced from `Cargo.toml` via `adopt-info` — keeps snap version in sync with crate version automatically
- GitHub Actions integration: uses `snapcore/action-build@v1` to build the snap — the official action handles LXD, base snap provisioning, and build environment setup without requiring live Snap Store access during the build. Replaces the previous manual `snapcraft pack --destructive-mode` approach which failed because GitHub Actions runners cannot install snaps from the store at build time.

## Source Files
- `snap/snapcraft.yaml` — snap build definition: name, version, confinement, build steps
- `.github/workflows/release.yml` — `publish-snap` job: extracts the linux tarball, builds the snap, and publishes to the Snap Store

## Commits
- placeholder
- `c429a0bb4fe2c6b048de6fd837bc32b9691d2b3e` — (auto-linked by taproot link-commits)
- `ad427b8f0be853f51a35a27d9aaf76d9df4a7eac` — (auto-linked by taproot link-commits)
- `c6aa5d4ee5a41b1dad749ac0743bef38596acf9a` — (auto-linked by taproot link-commits)
- `cbce3842efa7d271c696c4d1394df2b87c521db0` — (auto-linked by taproot link-commits)

## Tests
Integration tests require snapd and Snapcraft:

- **AC-1** (fresh install): publish snap to Snap Store edge channel; `snap install rpnpad --channel=edge`; verify `rpnpad` on PATH and `snap list` shows version
- **AC-2** (auto-refresh): install from edge, publish new revision; wait for auto-refresh or `snap refresh rpnpad`; verify new version
- **AC-3** (manual refresh): `snap refresh rpnpad`; verify latest revision installed
- **AC-4** (not found): `snap install rpnpad` before publishing; verify "snap not found" error
- **AC-5** (classic confinement): run rpnpad in terminal after install; verify stdin/stdout work without confinement errors

## DoR Resolutions

## DoD Resolutions
- condition: document-current | note: README.md snap section already documents 'snap install rpnpad' with the snapd prerequisite — no update needed | resolved: 2026-03-25T19:42:45.328Z
- condition: document-current | note: README snap section unchanged — external cause fix adds no new user-visible behaviour | resolved: 2026-03-25T19:53:54.998Z



## Status
- **State:** needs-rework
- **Created:** 2026-03-25
- **Last verified:** 2026-03-25

## Notes
- Classic confinement approval from Canonical may take days to weeks. During the review period, use `snap install --devmode rpnpad` for testing.
- A Snapcraft account (snapcraft.io) is required to publish. Register the snap name `rpnpad` via `snapcraft register rpnpad` before first publish.
- The snap name `rpnpad` may already be taken on the Snap Store — check before submitting.
- **External cause (2026-03-25):** GitHub Actions ubuntu-22.04 runners start with snapd in a degraded state. `snapcraft --destructive-mode` fails to install the `core22` base snap unless snapd is explicitly initialised first. Fix: add `sudo systemctl start snapd && sudo snap wait system seed.loaded` before the `Install snapcraft` step.
