# Implementation: Session Persistence

## Behaviour
../usecase.md

## Design Decisions
- Atomic write via write-to-temp → rename: session written to a `.tmp`
  file first, then `fs::rename()` — prevents corrupt state on interrupted write
- SIGTERM handled via `signal-hook` in `main.rs` — sets an atomic flag;
  the event loop checks the flag each tick and triggers a save before exit
- Session path: `~/.rpncalc/session.json` (via `dirs::home_dir()`)
- Only `CalcState` (stack + registers) is persisted — undo history is
  intentionally not persisted
- `RESET` command (typed in alpha mode) saves an empty `CalcState` to
  `session.json` and clears the running state — overwrites, does not delete

## Source Files
- `src/config/session.rs` — save(), load(), atomic write logic
- `src/main.rs` — SIGTERM handler setup via signal-hook; startup sequence
  (load config → restore session → render first frame)
- `src/tui/app.rs` — Action::ResetSession handler

## Commits
- 7066c63 feat: complete Epics 2–4 + layout width cap

## Tests
- `src/tui/app.rs` (inline `#[cfg(test)]`) — ResetSession clears stack
  and registers
- `src/config/session.rs` (inline `#[cfg(test)]`) — save/load round-trip,
  corrupt file handled gracefully, persist_session=false skips save

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-21

## Notes
None
