# Implementation: TUI

## Behaviour
../usecase.md

## Design Decisions
- **Quit rebound `q` → `Q` (shift-q)**: frees `q` for x² while keeping single-key quit; `ctrl` modifiers are rare in this app (only `ctrl-r` for redo), so uppercase is the natural choice
- **√ assigned to `w`**: `s` is swap (reserved); `w` is the cleanest available letter with no conflict; `\` was considered but awkward to type on most keyboards
- **Both `q` and `w` added as `InsertSubmitThen` shortcuts**: consistent with how `!`, `n`, and all other op shortcuts work in Insert mode — pressing them pushes the buffer then applies the operation
- **`f›` chord leader renamed from `fn` → `√`**: companion discoverability fix; immediately signals the group contains square-root functions without entering the chord
- **`q  x²` and `w  √` added to ARITHMETIC (depth ≥ 2) and UNARY_OPS (depth == 1)**: both are unary operations, so they appear whenever at least one value is on the stack

## Source Files
- `src/input/handler.rs` — Normal mode: `q`→Square, `Q`→Quit, `w`→Sqrt; Insert mode: `q`/`w` shortcuts added
- `src/tui/widgets/hints_pane.rs` — ARITHMETIC and UNARY_OPS gain `q  x²` and `w  √`; chord leader `f›  fn` renamed to `f›  √`; Insert mode hints updated
- `README.md` — key reference updated: `q`→x², `w`→√, `Q`→Quit

## Commits
<!-- taproot link-commits will fill this -->
- `a998d3b` — declare implementation
- `ecbb346` — implement direct x² and √ keys

## Tests
- `src/input/handler.rs` — AC-1: `q`→Square; AC-2: `w`→Sqrt; `Q`→Quit; Insert `q`/`w` shortcuts; AC-4: chords `fq`/`fs` unchanged
- `src/tui/widgets/hints_pane.rs` — AC-3: `q  x²` and `w  √` in depth≥1 hints; chord leader shows `√`; AC-5: recip/abs absent from Normal hints

## DoR Resolutions

## Status
- **State:** complete
- **Created:** 2026-03-25
- **Last verified:** 2026-03-26
