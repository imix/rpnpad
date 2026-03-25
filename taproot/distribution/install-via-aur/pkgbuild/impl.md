# Implementation: PKGBUILD

## Behaviour
../usecase.md

## Design Decisions
- Package named `rpncalc-bin` following AUR convention for pre-built binary packages — avoids conflict with a potential future `rpncalc` source package; users install with `yay -S rpncalc-bin`
- PKGBUILD pulls the pre-built `x86_64-unknown-linux-gnu` binary archive from the GitHub Release — no Rust toolchain required on the user's machine; fast install
- `arch=('x86_64')` only — cargo-dist currently builds only x86_64 for Linux; if `aarch64-unknown-linux-gnu` is added to `Cargo.toml` targets, a second `source_aarch64` stanza can be added
- `sha256sums` must be updated on every release; an `update-aur.sh` helper script automates fetching the new checksum and bumping `pkgver`/`pkgrel`
- The PKGBUILD lives in `aur/` at the repo root — the maintainer pushes to the AUR remote (`ssh://aur@aur.archlinux.org/rpncalc-bin.git`) from that directory
- LICENSE file installed alongside the binary per AUR packaging guidelines

## Source Files
- `aur/PKGBUILD` — AUR package definition: version, source URL, checksum, install steps
- `aur/.SRCINFO` — machine-readable package metadata (generated from PKGBUILD via `makepkg --printsrcinfo`); required by AUR
- `aur/update-aur.sh` — helper script to update pkgver, sha256sum, and regenerate .SRCINFO for a new release

## Commits
- placeholder

## Tests
This is a packaging configuration. Integration tests require an Arch Linux environment:

- **AC-1** (fresh install): `yay -S rpncalc-bin` in a clean Arch VM; verify `rpncalc` on PATH
- **AC-2** (upgrade): install old version, update PKGBUILD to new version, run `yay -S rpncalc-bin`; verify new version active
- **AC-3** (manual makepkg): `cd aur && makepkg -si`; verify install identical to AUR helper flow
- **AC-4** (checksum mismatch): corrupt sha256sum in PKGBUILD; verify makepkg aborts with integrity error
- **AC-5** (wrong arch): add `aarch64` to arch array without a source for it; verify failure

## DoR Resolutions

## Status
- **State:** complete
- **Created:** 2026-03-25
- **Last verified:** 2026-03-25

## Notes
- First submission to AUR requires creating an account at aur.archlinux.org and adding an SSH key. The AUR remote for a new package is `ssh://aur@aur.archlinux.org/rpncalc-bin.git` (the repo is created automatically on first push).
- After each GitHub Release, run `aur/update-aur.sh <version>` then `git push` to the AUR remote.
