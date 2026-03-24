# Implementation: Visual Polish TUI

## Behaviour
../usecase.md

## Design Decisions
- Rounded borders (`BorderType::Rounded`) on outer border and all inner panels —
  consistent with btop's aesthetic; Plain was too utilitarian
- Cyan accent on all panel titles and outer border title — single accent color,
  applied only to structural chrome, not content
- Separator row (`Block::default().borders(Borders::TOP)`) between error line and
  mode bar — produces `│────│` (outer `│` + inner `─────`); true `├───┤` junction
  chars would require overwriting the outer border cells, which violates ratatui's
  widget layering model
- Outer border title ` rpncalc ` (spaces for padding) in bold cyan — gives the app
  an identity anchor without adding a dedicated header row
- Minimum height guard raised from 6 → 7 (4 fixed rows: input+error+separator+mode,
  plus 2 border rows, plus 1 minimum content row)
- Hints pane block wraps all render paths (AlphaStore, Insert, Alpha, chord, Normal)
  — content renders into `block.inner(area)` for consistent inset

## Source Files
- `src/tui/layout.rs` — outer border style + title, separator row, updated guard
- `src/tui/widgets/stack_pane.rs` — rounded border + cyan title style
- `src/tui/widgets/hints_pane.rs` — bordered block with "Hints" title, content inset

## Commits
<!-- taproot link-commits will fill this -->
- `3e6badc8feed7dfd5b3db35e37fbd27e5d7bb0a6` — (auto-linked by taproot link-commits)
- `074982a52170d2fd53fa836c6ea66921a28727b4` — (auto-linked by taproot link-commits)

## Tests
- `src/tui/layout.rs` — updated corner char test (╭), new title/separator tests
- `src/tui/widgets/hints_pane.rs` — new bordered block test

## DoD Resolutions (AC-7)
- **check-if-affected (src/tui/widgets/stack_pane.rs)**: updated — added
  `Padding::horizontal(1)` to stack block; `block.inner()` accounts for padding automatically
- **check-if-affected (src/tui/widgets/hints_pane.rs)**: updated — added
  `Padding::horizontal(1)` to hints block; same pattern

## Status
- **State:** complete
- **Created:** 2026-03-24
- **Last verified:** 2026-03-24

## DoD Resolutions
- condition: baseline-usecase-exists | note: usecase.md exists at taproot/presentation/polish-visual-style/usecase.md — tool path resolution quirk | resolved: 2026-03-24T19:21:45.924Z
