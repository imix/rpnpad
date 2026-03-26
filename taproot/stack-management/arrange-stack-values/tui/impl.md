# Implementation: Arrange Stack Values

## Behaviour
../usecase.md

## Design Decisions
- Ops map to single-key normal-mode bindings: `s`=swap, `p`=dup,
  `Backspace`=drop, `R`=rotate, Enter-with-empty-buffer=dup (HP48 convention)
- `Backspace` is the sole drop key in Normal mode — muscle memory for users
  coming from text editors; `d` is now Noop (freed for future use)
- `Delete` is the key for clear — removes all stack items; no error when
  stack is already empty (`CalcState::clear()` is a no-op on empty stack)
- `Op::Clear` already existed in the engine; keybindings are purely in handler.rs
  with no engine or app-layer changes required
- All ops return `Result<>` — underflow surfaces as `CalcError::StackUnderflow`
  which the app layer renders to the ErrorLine

## Source Files
- `src/engine/stack.rs` — CalcState: swap(), dup(), drop(),
  rotate(), clear() — all transactional, return Result
- `src/input/handler.rs` — handle_key(): maps s/p/Backspace/R/Enter/Delete
  to Action::Execute(Op::*); d → Noop
- `src/tui/widgets/hints_pane.rs` — STACK_OPS and Insert mode hints updated
  to show Backspace for drop instead of d
- `src/engine/ops.rs` — Op enum variants and dispatch

## Commits
- 1695d6a feat: complete Epic 1 — Core Engine Foundation
- 7066c63 feat: complete Epics 2–4 + layout width cap
- `3df48c0ab966ef1c51ad3f3d1eb13d20c700cd10` — (auto-linked by taproot link-commits)
- `498dac8c901af73c77f9ff163226569e44bf17c7` — (auto-linked by taproot link-commits)

## Tests
- `src/engine/stack.rs` (inline `#[cfg(test)]`) — covers swap, dup, drop,
  rotate, clear including underflow cases and deep-stack invariants
- `src/input/handler.rs` — AC-6 (Backspace→Drop), AC-7 (Delete→Clear),
  AC-8 (d→Noop); all Normal-mode op key assertions

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-26

## Notes
None
