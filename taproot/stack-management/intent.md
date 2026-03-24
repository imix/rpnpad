# Intent: Stack Management

## Goal
Enable users to build and organise a stack of numeric values as the input
medium for RPN computation.

## Stakeholders
- **CLI power user**: needs a frictionless way to push values and arrange
  the stack before applying operations — wrong stack order means wrong results

## Success Criteria
- Any numeric literal (integer, float, hex, octal, binary) can be pushed
  onto the stack in under 2 keypresses
- Stack manipulation operations (swap, dup, drop, rotate, clear) are
  reachable from a single keypress in normal mode
- The full stack is always visible with the most-recent value at the top

## Constraints
- Stack is purely in-memory during a session; persistence is a separate concern
- Stack display is part of this intent; hints pane is a separate intent

## Behaviours <!-- taproot-managed -->
- [User arranges stack values](./arrange-stack-values/usecase.md)
- [User pushes a numeric value onto the stack](./push-value/usecase.md)
- [User rolls a deep stack value to the top](./roll-to-top/usecase.md)


## Status
- **State:** active
- **Created:** 2026-03-20
