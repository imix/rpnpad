# Discussion: tui

## Session
- **Date:** 2026-03-26
- **Skill:** tr-implement

## Pivotal Questions

**Where should the alias table live?**
`units.rs` — it's unit domain logic, and both the parser (for resolution) and the hints pane (for dimension matching) need access. Putting it in `parser.rs` would force hints_pane.rs to depend on parser internals.

**Should alias resolution use a new TaggedValue constructor or the existing compound path?**
Reuse the existing compound path (`parse_decimal_exact` + `parse_unit_expr_atoms` + `TaggedValue::new_compound`). The alias just provides the canonical compound string; everything downstream is identical to direct compound entry.

## Alternatives Considered

- **Alias table in `parser.rs`** — rejected: hints_pane.rs would need to import parser internals to show matching aliases
- **Special-case `TaggedValue::new_alias(amount, alias, canonical)` constructor** — rejected: unnecessary complexity; `new_compound` with the canonical string is sufficient and keeps storage identical to direct entry

## Decision

Alias table and dim-matching helper in `units.rs`; parser adds a step-1.5 lookup; hints pane calls `aliases_for_dim`. Three focused changes, no new types, no storage changes.

## Open Questions

- `dyn` and `lbf` are not in the unit table — AC-2's conversion to `dyn` cannot be tested until a force-unit extension is added.
