# UseCase: User pushes a numeric value onto the stack

## Actor
User (CLI power user)

## Preconditions
- rpncalc is running with the TUI open

## Main Flow
1. User begins typing a numeric literal — any digit keypress from normal mode
   triggers Insert mode automatically; no explicit mode-switch key required
2. TUI displays the growing input in the InputLine with a blinking cursor
3. User presses Enter to commit
4. The input is parsed into a CalcValue (Integer or Float)
5. The value is pushed onto the stack; stack display updates immediately

## Alternate Flows
- **Hex literal (0x…)**: parsed as integer in hexadecimal base
- **Octal literal (0o…)**: parsed as integer in octal base
- **Binary literal (0b…)**: parsed as integer in binary base
- **Float literal (digits with `.`)**: parsed as arbitrary-precision Float
- **User presses Esc mid-entry**: Insert mode exits, buffer discarded, no push
- **Operation shortcut**: in Insert mode, keys `s` `d` `r` `n` `p` `+` `-` `*`
  `/` `^` `%` `!` act as immediate shortcuts — submit the current buffer then
  execute the operation (e.g. type `3`, press `s` → push 3, swap)

## Error Conditions
- **Malformed input (e.g. `1.2.3`)**: error displayed on ErrorLine, stack
  unchanged, mode returns to normal

## Postconditions
- Stack depth increases by 1
- New value is at position 1 (top)
- Display updates to show the value in the current base/representation style
- Stack position labels (`1:` at top, `2:`, `3:`…) are always visible regardless
  of stack depth; empty rows show the label with no value beside it

## Flow

```mermaid
stateDiagram-v2
    [*] --> Normal
    Normal --> Insert : digit key pressed
    Insert --> Insert : digit / letter key
    Insert --> Normal : Esc — buffer discarded
    Insert --> Normal : Enter — parse ok → value pushed
    Insert --> Normal : Enter — parse fail → ErrorLine
    Insert --> Normal : op shortcut key — submit buffer → execute op
```

## Acceptance Criteria
**AC-1:** Given rpncalc is in normal mode, when the user presses a digit key, then Insert mode activates and the digit appears in the InputLine.

**AC-2:** Given Insert mode is active with a valid numeric literal, when Enter is pressed, then the value is pushed to the top of the stack and the stack display updates.

**AC-3:** Given Insert mode is active, when Esc is pressed, then the buffer is discarded, mode returns to normal, and the stack is unchanged.

**AC-4:** Given Insert mode is active with malformed input, when Enter is pressed, then an error is shown on the ErrorLine and the stack is unchanged.

**AC-5:** Given Insert mode is active, when an operation shortcut key (e.g. `s`, `+`) is pressed, then the current buffer is submitted and the operation executes immediately.

**AC-6:** Given the stack is empty, when the TUI renders the stack pane, then position labels (`1:`, `2:`, `3:`…) are shown for all visible rows with no value beside them.

## Related
- **Sibling**: [User arranges stack values](../arrange-stack-values/usecase.md)
- **Alpha mode** (free-text entry via `i` key, no shortcuts): see [named registers](../../state-and-memory/named-registers/usecase.md)
- **Parent intent**: [Stack Management](../../intent.md)

## Implementations <!-- taproot-managed -->
- [Push Value](./tui/impl.md)


## Status
- **State:** specified
- **Created:** 2026-03-21
- **Last reviewed:** 2026-03-24

