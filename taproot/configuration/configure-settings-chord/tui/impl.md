# Implementation: tui

## Behaviour
../usecase.md

## Design Decisions
- **`precision` moved from `App` to `CalcState`**: previously `App.precision` was not serialized to session.json; moving it to `CalcState` makes it persist across restarts alongside angle_mode, base, and hex_style.
- **New `Notation` enum in `src/engine/notation.rs`**: Fixed/Sci/Auto with serde. Added to `CalcState` so it persists in session.json and is readable by display code without threading extra parameters.
- **`PrecisionInput(String)` as a new `AppMode` variant**: the precision sub-mode is structurally equivalent to Insert mode (buffer accumulates, Enter confirms, Esc cancels, Backspace deletes). Reusing the AppMode enum pattern avoids ad-hoc state.
- **Hex style validation in `App::apply`**: keys `1`–`4` from the Config chord dispatch `SetHexStyle`, but `App::apply` checks `state.base == Base::Hex` before forwarding to `dispatch()`; if not HEX, sets error and exits chord. Handler cannot check app state, so validation belongs in apply.
- **`m`, `x`, `X` → Noop in Normal mode**: bindings removed; keys now fall through to the `_ => Action::Noop` arm. No error shown (AC-12).
- **Notation display in `value.rs`**: `display_with_notation(base, precision, notation)` wraps the existing `format_fbig_prec`; for `Sci` always uses `{:e}` formatting; for `Auto` applies threshold (`|v| ≥ 1e10` or `|v| < 1e-4 and v ≠ 0`); integers are unaffected by notation mode.
- **`SCI`/`AUTO` in mode bar right string**: appended after `DEG  DEC` when notation is non-default; subject to same width-truncation logic as other mode bar content.
- **Config chord submenu in hints pane**: replaces `m›`/`x›`/`X›` entries in `CHORD_LEADERS` with `C›  cfg`. The submenu shows ANGLE, BASE, NOTATION, PRECISION sections, plus HEX STYLE only when base=HEX (context-sensitive like existing chord submenus).

## Source Files
- `src/engine/notation.rs` — `Notation` enum with serde and Display
- `src/engine/mod.rs` — expose notation module
- `src/engine/stack.rs` — add `notation: Notation` and `precision: usize` fields
- `src/engine/value.rs` — `display_with_notation()` for sci/auto formatting
- `src/input/mode.rs` — add `Config` to `ChordCategory`; add `PrecisionInput(String)` to `AppMode`
- `src/input/action.rs` — add `SetNotation`, `EnterPrecisionInput`, `PrecisionInput*` actions
- `src/input/handler.rs` — Config chord dispatch; PrecisionInput mode; Noop for m/x/X
- `src/tui/app.rs` — remove `precision` field; handle new actions; hex style validation
- `src/tui/widgets/mode_bar.rs` — `[PREC]` mode label; `SCI`/`AUTO` indicator
- `src/tui/widgets/stack_pane.rs` — use `state.precision` + `state.notation`; drop param
- `src/tui/widgets/hints_pane.rs` — Config chord submenu; update leaders
- `src/tui/layout.rs` — drop `precision` from `stack_pane::render` call
- `src/config/config.rs` — add `notation` to `Config` and `ConfigToml`

## Commits
- placeholder

## Tests
- `src/engine/value.rs` — AC-3 (sci notation format), AC-4 (fixed), AC-5 (auto below threshold), AC-15 (auto above threshold); integer unaffected by notation
- `src/input/handler.rs` — AC-1/2/3/4 (Config chord dispatches correct actions); AC-12 (m/x are Noop); precision input mode keys
- `src/tui/app.rs` — AC-6 (precision updated), AC-7 (out-of-range rejected), AC-8 (hex style rejected when not HEX), AC-9 (hex style when HEX); AC-10 (Esc cancels)
- `src/tui/widgets/mode_bar.rs` — AC-14 (SCI/AUTO/blank indicator); [PREC] mode label
- `src/tui/widgets/hints_pane.rs` — AC-13 (Config chord shows all categories)
- `src/config/config.rs` — notation config.toml key parsed correctly

## Status
- **State:** complete
- **Created:** 2026-03-25
- **Last verified:** 2026-03-25

## Notes
None
