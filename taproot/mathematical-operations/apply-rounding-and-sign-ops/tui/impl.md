# Implementation: TUI

## Behaviour
../usecase.md

## Design Decisions
- **`r` rebind — Rotate moves to `R` (shift-r)**: The spec notes incorrectly stated `r` was free; in practice `r` = Rotate. Moving Rotate to `R` frees `r` for the new `r›` chord leader. `R` (shift-r) was unused in Normal mode.
- **SIGN in chord, not direct key**: The spec proposed `S` (shift-s) as a direct key for SIGN. In practice `S` = `EnterStoreMode` (named registers). Moving SIGN into the `r›` chord as second key `s` (`rs`) keeps all rounding/sign ops grouped and avoids breaking the store workflow.
- **`Q  quit` added to STACK_OPS hints**: Quit was bound to `Q` (from a prior behaviour) but never surfaced in the hints pane. Fixed in this pass.
- **FLOOR/CEIL/TRUNC return Float for Float inputs**: Consistent with all other float ops; avoids a Float→Integer type promotion that could surprise users. Integer inputs return Integer unchanged.
- **ROUND uses `binary_op` helper**: The existing `binary_op` infrastructure handles atomicity (stack unchanged on error). Custom precision validation (NotAnInteger) is applied before the round computation.
- **`NotAnInteger` error reused for non-integer precision**: Already defined in `CalcError`; applies cleanly to the ROUND precision validation case.

## Source Files
- `src/engine/ops.rs` — add Floor, Ceil, Trunc, Round, Sign variants + do_floor/do_ceil/do_trunc/do_sign/do_round implementations
- `src/input/mode.rs` — add Rounding to ChordCategory
- `src/input/handler.rs` — Normal: `r`→chord Rounding, `R`→Rotate; chord dispatch for Rounding; Insert: add `S` shortcut for SIGN via chord
- `src/tui/widgets/hints_pane.rs` — ROUNDING_OPS const, r› in CHORD_LEADERS, Q in STACK_OPS, R for rot, Rounding chord render branch

## Commits
<!-- taproot link-commits will fill this -->
- `04dda927646fbd5192e18398bdac21005265af62` — (auto-linked by taproot link-commits)
- `0f069cdf06451f0f8a917143942e8280747f2d62` — (auto-linked by taproot link-commits)

## Tests
- `src/engine/ops.rs` — AC-1 through AC-9, AC-11 through AC-13: FLOOR pos/neg, CEIL, TRUNC toward zero, ROUND decimal/negative precision, SIGN −1/0/+1, underflow, non-integer precision
- `src/input/handler.rs` — AC-14: Esc cancels chord; `r` enters chord mode; `R` rotates; `rs` dispatches Sign
- `src/tui/widgets/hints_pane.rs` — AC-10: r› and Q appear in Normal hints; Rounding chord submenu renders

## DoR Resolutions

## Status
- **State:** complete
- **Created:** 2026-03-25
- **Last verified:** 2026-03-26
