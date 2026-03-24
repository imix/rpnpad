# Intent: Discoverability

## Goal
Enable users to find and execute any operation without consulting external
documentation, through a context-sensitive hints pane that adapts to
calculator state.

## Stakeholders
- **CLI power user**: needs to find rare operations (e.g. factorial, trig
  inverses) in under 5 seconds without leaving the terminal or reading a
  manual — the hints pane is the only onboarding required

## Success Criteria
- Every MVP operation is reachable via the hints pane within 2 context states
  from any starting stack state
- A user with no prior rpncalc experience performs a two-operand arithmetic
  operation within 60 seconds of first launch
- Rare operations (e.g. `1/x`, `!`, trig inverses) are found in under 5
  seconds without external documentation

## Constraints
- Hints pane is read-only / purely functional — no owned state, no side effects
- Hints pane collapses entirely below 60 terminal columns (layout constraint)
- Chord submenu display is part of this intent; chord execution belongs to
  the input-handling concern

## Behaviours <!-- taproot-managed -->
- [User browses the hints pane to find an operation](./browse-hints-pane/usecase.md)
- [User executes an operation via chord sequence](./execute-chord-operation/usecase.md)


## Status
- **State:** active
- **Created:** 2026-03-20
