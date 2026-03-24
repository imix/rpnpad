# Implementation: Push Value

## Behaviour
../usecase.md

## Design Decisions
- Auto-insert-mode: any digit keypress from normal mode triggers Insert mode
  directly (`Action::InsertChar(c)`) — no explicit key required
- `i` key in Normal mode enters true Alpha mode (`AppMode::Alpha`) — all
  printable chars are literal, no op shortcuts; used for register commands
- Insert mode (`AppMode::Insert`) has `InsertSubmitThen(Op)` shortcuts: keys
  `s d r n p + - * / ^ % !` submit the numeric buffer then execute the op
- Alpha mode (`AppMode::Alpha`) has no shortcuts — all chars including the
  shortcut keys are literal (`AlphaChar`)
- The rename `AppMode::Alpha → AppMode::Insert` / new `AppMode::Alpha`
  separates the two input contexts that were previously conflated
- Digit separators (`_`) are stripped before parsing, allowing `1_000_000`
- Float detection is heuristic: presence of `.` or `e`/`E` in input;
  everything else is attempted as arbitrary-precision integer (IBig)
- f64 intermediate used for float parsing before conversion to FBig

## Source Files
- `src/input/mode.rs` — AppMode enum: Normal, Insert(String), Alpha(String),
  AlphaStore(String), Chord(ChordCategory)
- `src/input/action.rs` — Action enum: InsertChar/InsertSubmit/InsertSubmitThen/
  InsertBackspace/InsertCancel for Insert mode; AlphaChar/AlphaSubmit/
  AlphaBackspace/AlphaCancel for Alpha and AlphaStore modes
- `src/input/handler.rs` — handle_key(): digits → InsertChar, i → EnterAlphaMode;
  Insert mode arm with InsertSubmitThen shortcuts; Alpha mode arm (all chars literal)
- `src/tui/app.rs` — apply(): InsertSubmit parses number; AlphaSubmit dispatches
  register commands (STORE/RCL/DEL); InsertSubmitThen submits buffer then executes op
- `src/tui/widgets/mode_bar.rs` — displays [INSERT] for Insert/AlphaStore, [ALPHA]
  for Alpha, [NORMAL] for Normal/Chord
- `src/tui/widgets/input_line.rs` — shows buffer for Insert, Alpha, and AlphaStore
- `src/tui/widgets/hints_pane.rs` — Insert mode shows op shortcuts; Alpha mode shows
  "all chars literal" hint with no shortcuts
- `src/input/parser.rs` — parse_value(): parses string into CalcValue
- `src/engine/stack.rs` — CalcState::push(): appends parsed value to stack

## Commits
- 1695d6a feat: complete Epic 1 — Core Engine Foundation
- 7066c63 feat: complete Epics 2–4 + layout width cap
- `d6e76b6316b2d36e54c992dc1cf434d9070b2bc3` — (auto-linked by taproot link-commits)
- `08fe974b28285b99ef851a0186131def26cc2cf2` — (auto-linked by taproot link-commits)

## Tests
- `src/input/handler.rs` — handler tests: Insert mode op shortcuts
  (InsertSubmitThen), Alpha mode all-chars-literal (no shortcuts for r/s/d/etc.)
- `src/tui/app.rs` — app tests: InsertChar creates AppMode::Insert, InsertSubmit
  parses numbers, AlphaSubmit dispatches commands (STORE/RCL), AlphaCancel
- `src/tui/widgets/mode_bar.rs` — Insert → [INSERT], Alpha → [ALPHA]
- `src/tui/widgets/input_line.rs` — Insert and Alpha buffers render with cursor
- `src/tui/widgets/hints_pane.rs` — Insert shows shortcuts, Alpha shows literal hint
- `src/input/parser.rs` (inline) — all numeric formats
- `src/engine/stack.rs` (inline) — push/pop behaviour

## DoD Resolutions
- **check-if-affected (src/tui/widgets/hints_pane.rs)**: updated — Insert mode
  shows op shortcut hints; new Alpha mode shows "all chars literal" hint panel
- **check-if-affected (src/tui/widgets/mode_bar.rs)**: updated — Alpha shows
  [ALPHA], Insert shows [INSERT]
- **check-if-affected (src/input/handler.rs)**: updated — new AppMode::Alpha arm
  with all-literal char handling; Insert arm has InsertSubmitThen shortcuts
- **check-if-affected (src/tui/app.rs)**: updated — InsertSubmit parses numbers,
  AlphaSubmit dispatches commands only; two separate action sets

## DoD Resolutions (AC-6)
- **check-if-affected (src/tui/widgets/stack_pane.rs)**: updated — empty rows now show
  HP48-style position labels (`4:`, `3:`, `2:`, `1:`) with DIM style; `label_col_width`
  unified to use `height` so label column is consistent across empty and value rows

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-24
