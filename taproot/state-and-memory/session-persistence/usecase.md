# UseCase: Session state persists across process restarts

## Actor
User (CLI power user)

## Preconditions
- `persist_session = true` in config (default)
- `~/.rpncalc/` directory is writable

## Main Flow
1. User quits rpncalc (`q`) or the process receives SIGTERM
2. Current CalcState (stack + registers) is written atomically to
   `~/.rpncalc/session.json` via write-to-temp → rename
3. On next launch: config is loaded, then session.json is read and
   CalcState is restored before the first frame renders

## Alternate Flows
- **`persist_session = false`**: session is not saved on exit; rpncalc
  always starts with an empty stack and no registers
- **SIGTERM**: signal-hook handler triggers the same save path as clean exit

## Error Conditions
- **Write failure** (disk full, permissions): prior session.json left intact
  via atomic write — no corrupt partial state
- **Corrupt session.json on load**: file is ignored; rpncalc starts with
  empty state and an informative error is shown on the ErrorLine
- **SIGKILL**: no save possible — explicitly out of scope

## Postconditions
- On save: session.json reflects the final CalcState at exit time
- On restore: stack and registers are identical to the last saved state

## Flow

```mermaid
stateDiagram-v2
    [*] --> Running : launch — no session file → empty state
    [*] --> Running : launch — valid session.json → state restored
    [*] --> Running : launch — corrupt session.json → empty state + ErrorLine
    Running --> [*] : quit (q) or SIGTERM → session.json written atomically
```

## Acceptance Criteria
**AC-1:** Given `persist_session = true` and the user quits with `q`, then the current CalcState is written atomically to `~/.rpncalc/session.json`.

**AC-2:** Given a valid `session.json` exists on launch, then the prior stack and registers are restored before the first frame renders.

**AC-3:** Given `session.json` is corrupt on launch, then rpncalc starts with empty state and an informative error is shown on the ErrorLine.

**AC-4:** Given `persist_session = false`, then session.json is not read on launch and not written on exit.

## Related
- **Sibling**: [User undoes or redoes a state-mutating operation](../undo-redo/usecase.md)
- **Sibling**: [User stores and recalls values in named registers](../named-registers/usecase.md)
- **Configured by**: [User configures rpncalc defaults via config.toml](../../configuration/configure-defaults/usecase.md)
- **Parent intent**: [State and Memory](../../intent.md)

## Implementations <!-- taproot-managed -->
- [Session Persistence](./tui/impl.md)


## Status
- **State:** specified
- **Created:** 2026-03-21
- **Last reviewed:** 2026-03-24
