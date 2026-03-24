# Implementation: README

## Behaviour
../usecase.md

## Design Decisions
- README is the single pre-install documentation surface — no separate wiki or docs site; it must be comprehensive but scannable
- Key reference derived directly from `src/input/handler.rs` — handler is the single source of truth; README must be updated whenever bindings change
- `OWNER` placeholder used in install commands — must be replaced with the real GitHub username before the first release (three locations: Homebrew tap name, curl URL, Cargo.toml repository field)
- Quick-start uses a concrete arithmetic example (push two numbers, add) — chosen to require no prior RPN knowledge
- Chord sequences presented as a table per leader key — mirrors the in-app hints pane structure users will see once installed
- Store shortcut (`S`) documented alongside Alpha mode (`i` + `name STORE`) — both paths do slightly different things (S peeks; Alpha STORE pops) and both deserve coverage

## Source Files
- `README.md` — project README: description, all install paths, quick start, complete key reference by mode, stack model, named registers, configuration

## Commits
<!-- taproot link-commits will fill this -->

## Tests
All ACs are verified by content inspection:
- **AC-1** (all three install paths): README contains brew, curl, and cargo commands
- **AC-2** (quick-start correct): example uses only documented keys and produces a deterministic result
- **AC-3** (key reference covers all modes): Normal, Insert, Browse, and all 7 chord sequences present
- **AC-4** (stack model explained): position numbering, RPN convention, and HP48 heritage described
- **AC-5** (named registers documented): S shortcut and Alpha mode STORE/RCL/DEL commands present
- **AC-6** (no OWNER placeholders): verified before first release by maintainer

## DoR Resolutions

## Status
- **State:** in-progress
- **Created:** 2026-03-24
- **Last verified:** 2026-03-24
