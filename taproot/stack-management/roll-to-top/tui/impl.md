# Implementation: Roll to Top TUI

## Behaviour
../usecase.md

## Design Decisions
- `AppMode::Browse(usize)` stores cursor position as a 1-indexed stack
  position (2..=depth); position 1 is the top — starting at 2 means
  "the first item above the top"
- `EnterBrowseMode` depth check lives in `app.rs::apply()`, consistent
  with the `EnterStoreMode` pattern (handler produces the action,
  app decides whether preconditions are met)
- Cursor clamped at `depth` in app logic; visible-height limit is an
  accepted trade-off documented in the spec Notes
- `browse_cursor: Option<usize>` passed to `stack_pane::render` rather
  than full `AppMode` — keeps the widget's interface narrow and avoids
  coupling stack rendering to mode logic
- Cursor highlight uses `Modifier::REVERSED` — standard TUI cursor
  idiom, visually distinct without introducing a new accent color
- Browse mode in handler uses `_ => Action::Noop` for unrecognised
  keys — silently consumed, satisfying AC-11

## Source Files
- `src/engine/stack.rs` — `CalcState::roll(n)`: removes item at
  1-indexed position N from top, pushes it to top
- `src/input/mode.rs` — `AppMode::Browse(usize)` variant added
- `src/input/action.rs` — `EnterBrowseMode`, `BrowseCursorUp`,
  `BrowseCursorDown`, `BrowseConfirm`, `BrowseCancel` added
- `src/input/handler.rs` — Normal: `↑ → EnterBrowseMode`; new
  `AppMode::Browse` arm handling navigation and confirmation keys
- `src/tui/app.rs` — all 5 Browse actions handled in `apply()`
- `src/tui/widgets/stack_pane.rs` — `browse_cursor: Option<usize>`
  param; cursor row rendered with `Modifier::REVERSED`
- `src/tui/layout.rs` — derives `browse_cursor` from `app.mode`,
  passes to `stack_pane::render`
- `src/tui/widgets/mode_bar.rs` — `Browse(_) => "[BROWSE]"`
- `src/tui/widgets/hints_pane.rs` — Browse mode hint panel

## Commits
<!-- taproot link-commits will fill this -->
- `5d087e3047be51f79388daec1760877d9a6056f6` — (auto-linked by taproot link-commits)
- `706a54a252a42144f8fd68113382106fc21616ac` — (auto-linked by taproot link-commits)

## Tests
- `src/engine/stack.rs` (inline) — roll mechanics (AC-1 postconditions),
  roll underflow (AC-7 engine side)
- `src/input/handler.rs` (inline) — `↑` → `EnterBrowseMode` (entry),
  Browse nav keys (AC-3/4), other keys → Noop (AC-11), Esc →
  `BrowseCancel` (AC-2), Enter → `BrowseConfirm`
- `src/tui/app.rs` (inline) — error on ≤1 item (AC-7), cursor
  up/down clamping (AC-5/6), confirm rolls correct item (AC-1),
  cancel preserves stack (AC-2), roll records undo snapshot (AC-12)
- `src/tui/widgets/mode_bar.rs` (inline) — `[BROWSE]` displayed (AC-8)
- `src/tui/widgets/hints_pane.rs` (inline) — Browse hints visible (AC-9)
- `src/tui/widgets/stack_pane.rs` (inline) — cursor row has REVERSED
  modifier (AC-10)

## Status
- **State:** complete
- **Created:** 2026-03-24
- **Last verified:** 2026-03-26
