# Intent: State and Memory

## Goal
Enable users to preserve calculation state across sessions, recover from
mistakes at any point, and store intermediate results in named registers.

## Stakeholders
- **CLI power user**: needs to pick up where they left off after closing the
  terminal, undo errors without restarting a multi-step calculation, and
  share values between calculations via named registers and clipboard

## Success Criteria
- Session state (stack + registers) is restored automatically on next launch
- Undo covers every state-mutating operation without exception, including
  register stores and mode changes
- Named registers survive process exit and are recalled by name in subsequent
  sessions
- Top-of-stack value is copyable to system clipboard in the current
  representation style

## Constraints
- Undo history is in-memory only — not persisted across sessions
- Session writes must be atomic (write-to-temp → rename); no corrupt state
  on interrupted write
- SIGTERM must trigger a session save before exit; SIGKILL is explicitly
  out of scope

## Behaviours <!-- taproot-managed -->
- [User copies the top stack value to the system clipboard](./clipboard-copy/usecase.md)
- [User stores and recalls values in named registers](./named-registers/usecase.md)
- [Session state persists across process restarts](./session-persistence/usecase.md)
- [User undoes or redoes a state-mutating operation](./undo-redo/usecase.md)


## Status
- **State:** active
- **Created:** 2026-03-20
