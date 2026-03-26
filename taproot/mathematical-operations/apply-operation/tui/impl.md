# Implementation: Apply Operation

## Behaviour
../usecase.md

## Design Decisions
- All ops go through a single `apply_op(state, op)` dispatch function —
  atomicity guaranteed: if an op returns `Err`, the stack is left unchanged
- Trig ops use f64 intermediates (via `to_f64()`) and the active `AngleMode`
  from `CalcState`; result converted back to `FBig`
- Bitwise ops require integer operands — `CalcError::TypeMismatch` returned
  for float inputs
- Constants (`PushPi`, `PushE`, `PushPhi`) are ops in the same dispatch
  table — they push without consuming any operand

## Source Files
- `src/engine/ops.rs` — Op enum (all 41 operations + constants) and
  apply_op() dispatch
- `src/engine/constants.rs` — π, e, φ values as CalcValue
- `src/engine/angle.rs` — AngleMode enum and angle conversion helpers
- `src/engine/value.rs` — CalcValue (Integer/Float) and arithmetic traits
- `src/input/handler.rs` — immediate-key bindings (+/-/*// etc.)
  and chord dispatch to Action::Execute(Op::*)

## Commits
- 1695da feat: complete Epic 1 — Core Engine Foundation
- 7066c63 feat: complete Epics 2–4 + layout width cap

## Tests
- `src/engine/ops.rs` (inline `#[cfg(test)]`) — covers arithmetic, trig,
  log/exp, bitwise, utility ops, constants, underflow, and domain errors

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-26

## Notes
None
