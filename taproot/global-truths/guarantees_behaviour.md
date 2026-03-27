# Guarantees — Behaviour-Level

Cross-cutting guarantees that every behaviour and its implementation must uphold.

---

## Errors Never Modify State

When an error occurs, all application state (stack, registers, modes) is left unchanged. Errors are reported on the ErrorLine only; no partial mutations are written.

This applies to:
- Stack underflow (operation requires more items than are present)
- Domain errors (e.g. sqrt of a negative, division by zero)
- Incompatible unit arithmetic
- Malformed input that fails parsing
- Any other condition that causes an error to be shown on the ErrorLine

If an error is shown, the caller can assume the state is identical to before the operation was attempted.
