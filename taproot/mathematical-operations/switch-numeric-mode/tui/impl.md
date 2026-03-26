# Implementation: Switch Numeric Mode

## Behaviour
../usecase.md

## Design Decisions
- Mode switches are implemented as ops in the same `apply_op` dispatch —
  `Op::SetBase(Base::Hex)` etc. — so they are undo-able via the same
  snapshot mechanism as all other state-mutating ops
- The original `x›`/`m›`/`X›` separate chord leaders were removed and
  consolidated into the `C›` config chord (see `configure-settings-chord`);
  `m`, `x`, `X` are now Noop in Normal mode
- All stack values are re-rendered on the next frame using the new mode
  from `CalcState` — no re-computation, purely display

## Source Files
- `src/engine/base.rs` — Base and HexStyle enums
- `src/engine/angle.rs` — AngleMode enum
- `src/engine/ops.rs` — mode-switch op variants and dispatch
- `src/input/handler.rs` — m/x/X → Noop; mode-switch keys are now in
  the `C›` Config chord dispatch (see configure-settings-chord impl)
- `src/input/commands.rs` — chord second-key dispatch for mode ops

## Commits
- 7066c63 feat: complete Epics 2–4 + layout width cap

## Tests
- `src/engine/base.rs` (inline `#[cfg(test)]`) — Base and HexStyle variants
- `src/engine/angle.rs` (inline `#[cfg(test)]`) — AngleMode variants and
  angle conversion

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-26

## Notes
None
