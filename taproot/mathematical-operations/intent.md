# Intent: Mathematical Operations

## Goal
Enable users to apply a comprehensive set of mathematical operations to
stacked values and control the numeric modes that govern their computation.

## Stakeholders
- **CLI power user**: needs the full breadth of arithmetic, scientific, and
  bitwise operations available without leaving the terminal — covering daily
  arithmetic through occasional trig and bitwise work

## Success Criteria
- All 41 MVP operations are executable from the keyboard without external docs
- Numeric base (DEC/HEX/OCT/BIN) and angle mode (DEG/RAD/GRAD) can be
  switched mid-session with a single chord
- Arithmetic errors (division by zero, domain errors) are caught and reported
  without modifying stack state

## Constraints
- f64 intermediates acceptable for trig/log; arbitrary precision via dashu
  for integer and general float operations
- Bitwise operations apply to integer representations only
- Phase 2 unit conversions are out of scope for this intent

## Behaviours <!-- taproot-managed -->
- [User applies a mathematical operation to stacked values](./apply-operation/usecase.md)
- [User switches numeric mode mid-session](./switch-numeric-mode/usecase.md)


## Status
- **State:** active
- **Created:** 2026-03-20
