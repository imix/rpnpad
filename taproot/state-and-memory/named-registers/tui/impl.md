# Implementation: Named Registers

## Behaviour
../usecase.md

## Design Decisions
- Registers are stored directly in `CalcState.registers: HashMap<String, CalcValue>` —
  no separate register module; `src/engine/registers.rs` is an empty placeholder
- STORE/RCL/DEL are parsed from alpha mode input by `parse_command()` in
  `src/input/commands.rs`, returning typed `Action` variants
- Register operations are handled in `app.rs` `apply()`, not in `apply_op()` —
  they need direct access to `UndoHistory` for snapshot-before-mutate
- Two STORE paths with different stack semantics:
  - Alpha-mode `<name> STORE` → `Action::StoreRegister` → **pops** X (consuming it)
  - `S` key in normal mode → `Action::EnterStoreMode` → `AppMode::AlphaStore` →
    on submit: **peeks** X via `stack.last()` (X remains on stack)
- RCL pushes a clone of the register value (register preserved)

## Source Files
- `src/engine/stack.rs` — CalcState.registers: HashMap<String, CalcValue>
- `src/input/commands.rs` — parse_command(): parses <name> STORE/RCL/DEL
  into Action::StoreRegister/RecallRegister/DeleteRegister
- `src/input/handler.rs` — 'S' in Normal mode → Action::EnterStoreMode;
  AppMode::AlphaStore key handling
- `src/tui/app.rs` — apply(): EnterStoreMode → AppMode::AlphaStore (peek
  path); AlphaSubmit for AlphaStore (peek + insert); StoreRegister (pop
  path); all register actions with undo snapshots

## Commits
- 7066c63 feat: complete Epics 2–4 + layout width cap

## Tests
- `src/tui/app.rs` (inline `#[cfg(test)]`) — store, recall, delete, missing
  register errors, undo after store/delete, atomic undo of stack+registers
- `src/input/commands.rs` (inline `#[cfg(test)]`) — command parsing for
  STORE/RCL/DEL variants

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-26

## Notes
`src/engine/registers.rs` exists but is empty — register logic lives in
`CalcState` and `app.rs`. The file may be cleaned up in future.
