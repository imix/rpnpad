# Implementation: snapcraft

## Behaviour
../usecase.md

## Design Decisions
- `confinement: strict` with no extra plugs — the snap sets `HOME=$SNAP_USER_COMMON` in the app environment, redirecting `dirs::home_dir()` to `~/snap/rpnpad/common/`; config and session files land at `~/snap/rpnpad/common/.rpnpad/`; no store review required
- Uses `dump` plugin with pre-built binary from cargo-dist — simpler than building from source in the snap pipeline; the binary is already verified by the cargo-dist release
- `base: core22` (Ubuntu 22.04 LTS) — current stable base with long support horizon; avoids `core18` (deprecated) and `core24` (newer, less tested)
- `grade: stable` — published to the stable channel; users get it by default without specifying a channel
- Version sourced from `version.txt` written by the release workflow — keeps snap version in sync with the release tag
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
- `ed3f756cfe5bfd316a4ab3a90f88e7012cd92561` — (auto-linked by taproot link-commits)
- `afdc4b5f2cf79c13a5fc66130ae8be98da38f95e` — (auto-linked by taproot link-commits)
- `f3dc9d2c82e8b4d9439e1c793b025770137ac775` — (auto-linked by taproot link-commits)
- `8b9d87ac96e75a315cc46008578df1c2597856a2` — (auto-linked by taproot link-commits)

## Tests
Integration tests require snapd and Snapcraft:

- **AC-1** (fresh install): publish snap to Snap Store; `snap install rpnpad`; verify `rpnpad` on PATH and `snap list` shows version
- **AC-2** (auto-refresh): install snap, publish new revision; wait for auto-refresh or `snap refresh rpnpad`; verify new version
- **AC-3** (manual refresh): `snap refresh rpnpad`; verify latest revision installed
- **AC-4** (not found): `snap install rpnpad` before publishing; verify "snap not found" error
- **AC-5** (terminal access): run rpnpad in terminal after install; verify stdin/stdout and TUI rendering work correctly under strict confinement

## DoR Resolutions

## DoD Resolutions
- condition: document-current | note: README.md snap section already documents 'snap install rpnpad' with the snapd prerequisite — no update needed | resolved: 2026-03-25T19:42:45.328Z
- condition: document-current | note: README snap section unchanged — HOME override is an internal detail; user-facing install command is identical | resolved: 2026-03-25T21:03:45.092Z

- condition: document-current | note: README snap section unchanged — confinement change is not user-visible; install command is the same | resolved: 2026-03-25T20:19:38.445Z

- condition: document-current | note: README snap section unchanged — CI mechanism change is not user-visible | resolved: 2026-03-25T20:07:22.698Z
- condition: document-current | note: README snap section unchanged — external cause fix adds no new user-visible behaviour | resolved: 2026-03-25T19:53:54.998Z

## Status
- **State:** complete
- **Created:** 2026-03-25
- **Last verified:** 2026-03-25

## Notes
- A Snapcraft account (snapcraft.io) is required to publish. Register the snap name `rpnpad` via `snapcraft register rpnpad` before first publish.
- The snap name `rpnpad` may already be taken on the Snap Store — check before submitting.
- **External cause (2026-03-25):** GitHub Actions ubuntu-22.04 runners start with snapd in a degraded state. `snapcraft --destructive-mode` fails to install the `core22` base snap unless snapd is explicitly initialised first. Resolved by switching to `snapcore/action-build@v1`.
