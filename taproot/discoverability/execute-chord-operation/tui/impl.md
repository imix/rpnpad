# Implementation: Execute Chord Operation

## Behaviour
../usecase.md

## Design Decisions
- Chord state is modelled as `AppMode::Chord(ChordCategory)` — the app
  holds mode in a single `AppMode` enum, not a separate flag
- `handle_key()` dispatches to `dispatch_chord_key(category, c)` which
  maps (category, second_key) → `Action`; unrecognised second keys return
  `Action::ChordInvalid`
- Esc from chord-wait returns `Action::ChordCancel` → mode resets to Normal
  with no state mutation

## Source Files
- `src/input/handler.rs` — handle_key() chord arm; dispatch_chord_key()
- `src/input/commands.rs` — (category, key) → Action mapping tables
- `src/input/mode.rs` — AppMode::Chord(ChordCategory) and ChordCategory
  enum
- `src/tui/widgets/hints_pane.rs` — chord-active render state (submenu view)

## Commits
- 7066c63 feat: complete Epics 2–4 + layout width cap

## Tests
- `src/input/handler.rs` (inline `#[cfg(test)]`) — chord dispatch for each
  category and Esc/invalid-key handling

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-26

## Notes
None
