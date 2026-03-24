# UseCase: User arranges stack values

## Actor
User (CLI power user)

## Preconditions
- rpncalc is running in normal mode
- Stack has sufficient depth for the chosen operation (≥1 for dup/drop,
  ≥2 for swap, ≥3 for rotate; clear works on any depth including empty)

## Main Flow
1. User presses a single key in normal mode:
   - `s` — swap: exchanges X and Y positions
   - `p` — dup: duplicates X, pushing a copy onto the stack
   - `d` — drop: discards X
   - `r` — rotate: moves X to Z, Y to X, Z to Y (roll down top three)
   - clear — removes all values from the stack
2. Stack updates immediately; display reflects new arrangement

## Alternate Flows
- **Enter with empty input buffer (HP convention)**: behaves as dup

## Error Conditions
- **Stack underflow** (e.g. swap with <2 items, rotate with <3): error shown
  on ErrorLine, stack left unchanged

## Postconditions
- Stack reflects the new arrangement
- All values that were not affected remain unchanged and in their prior positions

## Flow

```mermaid
stateDiagram-v2
    [*] --> Normal
    Normal --> Normal : s/p/d/r — ok → stack updated
    Normal --> Normal : s/p/d/r — underflow → ErrorLine
```

## Acceptance Criteria
**AC-1:** Given the stack has ≥1 item, when the user presses `p`, then X is duplicated at the top of the stack.

**AC-2:** Given the stack has ≥2 items, when the user presses `s`, then X and Y are exchanged.

**AC-3:** Given the stack has ≥1 item, when the user presses `d`, then X is removed from the stack.

**AC-4:** Given the stack has ≥3 items, when the user presses `r`, then X moves to Z position, Y moves to X, and Z moves to Y.

**AC-5:** Given insufficient stack depth for the chosen operation, when the key is pressed, then an error is shown on the ErrorLine and the stack is unchanged.

## Related
- **Sibling**: [User pushes a numeric value onto the stack](../push-value/usecase.md)
- **Parent intent**: [Stack Management](../../intent.md)

## Implementations <!-- taproot-managed -->
- [Arrange Stack Values](./tui/impl.md)


## Status
- **State:** specified
- **Created:** 2026-03-21
- **Last reviewed:** 2026-03-24
