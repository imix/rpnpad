# Implementation: Undo / Redo

## Behaviour
../usecase.md

## Design Decisions
- Snapshot-before-execute model: `UndoHistory::snapshot(&state)` is called
  before every state-mutating op, cloning the full `CalcState`
- `snapshot()` clears the redo stack — any new op after an undo discards
  forward history
- Depth is bounded: oldest snapshot removed from the front when
  `past.len() > max_depth` (from config `max_undo_history`, default 1000)
- Undo history is in-memory only — not persisted to session.json

## Source Files
- `src/engine/undo.rs` — UndoHistory: snapshot(), undo(), redo()
- `src/tui/app.rs` — app loop calls history.snapshot(&state) before
  each apply_op; handles Action::Undo and Action::Redo
- `src/input/handler.rs` — u → Action::Undo, Ctrl-R → Action::Redo

## Commits
- 7066c63 feat: complete Epics 2–4 + layout width cap

## Tests
- `src/engine/undo.rs` (inline `#[cfg(test)]`) — covers snapshot, undo,
  redo, depth bounding, and redo-cleared-on-new-op

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-26

## Notes
None
