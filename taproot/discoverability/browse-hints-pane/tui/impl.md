# Implementation: Browse Hints Pane

## Behaviour
../usecase.md

## Design Decisions
- HintsPane is purely functional — it takes `&CalcState` and `&AppMode`
  and renders directly; no owned state
- Three render states driven by `AppMode`: Normal (categorised grid),
  Chord-active (category header + submenu), Alpha (alpha navigation hints)
- Context-sensitivity is implemented via stack depth check: depth 0 shows
  push/constants; depth ≥1 adds unary ops; depth ≥2 adds binary op hints
- Register section rendered only when `CalcState.registers` is non-empty
- Responsive: layout module controls pane visibility — collapses at <60 cols

## Source Files
- `src/tui/widgets/hints_pane.rs` — full hints pane render logic, all
  category tables (ARITHMETIC, STACK_OPS, TRIG_OPS, LOG_OPS, FN_OPS,
  CONST_OPS, ANGLE_OPS, BASE_OPS, HEX_STYLE_OPS, CHORD_LEADERS)
- `src/tui/layout.rs` — responsive layout constraints controlling pane
  width and visibility
- `src/input/mode.rs` — AppMode state machine (Normal/Chord/Alpha)

## Commits
- 7066c63 feat: complete Epics 2–4 + layout width cap

## Tests
- `src/tui/widgets/hints_pane.rs` — AC-1: `test_depth0_shows_constants_leader`, `test_depth0_shows_stack_ops`; AC-2: `test_depth2_shows_full_arithmetic`, `test_normal_mode_shows_add_op`; AC-3: `test_registers_shows_section_header`, `test_registers_shows_register_name`, `test_registers_shows_recall_command`
- `src/tui/layout.rs` — AC-4: `test_narrow_terminal_hides_hints` (width<60 collapses pane), `test_wide_terminal_shows_hints`, `test_medium_terminal_shows_hints`

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-21

## Notes
None
