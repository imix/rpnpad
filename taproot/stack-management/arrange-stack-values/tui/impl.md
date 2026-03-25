# Implementation: Arrange Stack Values

## Behaviour
../usecase.md

## Design Decisions
- All five ops map to single-key normal-mode bindings: `s`=swap, `p`=dup,
  `d`=drop, `r`=rotate, Enter-with-empty-buffer=dup (HP48 convention)
- `Backspace` added as alias for `d` (drop) in Normal mode — muscle memory
  for users coming from text editors; no conflict because Backspace in Insert
  mode already deletes the last typed char (different mode, different handler arm)
- `Delete` added as the key for clear — removes all stack items; no error
  when stack is already empty (`CalcState::clear()` is a no-op on empty stack)
- `Op::Clear` already existed in the engine; this change is purely a keybinding
  addition in handler.rs with no engine or app-layer changes required
- All ops return `Result<>` — underflow surfaces as `CalcError::StackUnderflow`
  which the app layer renders to the ErrorLine

## Source Files
- `src/engine/stack.rs` — CalcState: swap(), dup(), drop(),
  rotate(), clear() — all transactional, return Result
- `src/input/handler.rs` — handle_key(): maps s/p/d/r/Enter
  to Action::Execute(Op::*)
- `src/engine/ops.rs` — Op enum variants and dispatch

## Commits
- 1695d6a feat: complete Epic 1 — Core Engine Foundation
- 7066c63 feat: complete Epics 2–4 + layout width cap
- `3df48c0ab966ef1c51ad3f3d1eb13d20c700cd10` — (auto-linked by taproot link-commits)

## Tests
- `src/engine/stack.rs` (inline `#[cfg(test)]`) — covers swap, dup, drop,
  rotate, clear including underflow cases and deep-stack invariants

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-25

## Notes
None
