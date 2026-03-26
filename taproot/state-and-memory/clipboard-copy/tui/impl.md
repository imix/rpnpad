# Implementation: Clipboard Copy

## Behaviour
../usecase.md

## Design Decisions
- Uses `arboard` crate for cross-platform clipboard access
- Value is formatted using the same display logic as the stack pane —
  current base and `HexStyle` applied before writing to clipboard
- `y` (yank) in normal mode → `Action::Yank` — no stack mutation

## Source Files
- `src/tui/app.rs` — Action::Yank handler: formats X value and calls
  arboard::Clipboard::new()?.set_text(formatted)
- `src/input/handler.rs` — y → Action::Yank
- `src/engine/value.rs` — value formatting with base/style

## Commits
- 7066c63 feat: complete Epics 2–4 + layout width cap

## Tests
- `src/tui/app.rs` (inline `#[cfg(test)]`) — yank on empty stack produces
  error; yank on non-empty stack leaves stack unchanged

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-26

## Notes
Clipboard availability depends on the runtime environment — no display
server (e.g. headless CI) will cause `arboard` to return an error, which
is surfaced on the ErrorLine.
