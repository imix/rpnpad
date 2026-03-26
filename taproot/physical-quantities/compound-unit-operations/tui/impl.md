# Implementation: tui

## Behaviour
../usecase.md

## Design Decisions
- **Atom-based compound unit representation**: compound unit strings (e.g. `km/h`, `kg*m/s2`) are stored in `TaggedValue.unit` as display strings and parsed on demand via `parse_unit_expr_atoms()`. This avoids adding a new field to `TaggedValue` and preserves serde compatibility.
- **`parse_unit_expr_atoms(expr: &str)` in `units.rs`**: splits on `/` for numerator/denominator, splits on `*` and whitespace for atoms, parses each atom as `<abbrev>[<exponent>]`. Temperature units rejected as compound atoms. Returns `Vec<(String, i8)>` — abbrev plus signed exponent.
- **`combine_atoms_mul` for unit arithmetic**: when multiplying two tagged values, the result atom list is the merge of both atom lists with exponents added. Zero-exponent atoms are dropped. This correctly handles unit cancellation (e.g. `km/h * h = km`).
- **`compound_to_si_scale` for conversion**: converts compound unit to SI scale factor by multiplying each constituent unit's `to_base` factor raised to its exponent. Used for Add/Sub conversion and for the ConvertUnit action.
- **`convert_tagged_to_unit` for Add/Sub conversion**: replaces the old category-based `convert()` in tagged Add/Sub. Uses simple `convert()` for same-category simple units; falls back to compound scale division for compound or cross-category same-dim units.
- **Dim equality check for Add/Sub**: the old category check is replaced by `tx.dim == ty.dim`. This correctly handles compound units (e.g. `m/s + km/h` is valid; `m/s + m/s2` is not).
- **Sqrt with dim halving**: `Op::Sqrt` on a tagged value calls `dim.halve()` (returns `None` if any exponent is odd) and propagates error `non-integer unit exponent after sqrt`. The unit display is derived by halving atom exponents; falls back to `derive_display_from_dim()` if atom exponents are not all even.
- **Reciprocal with dim negation**: `Op::Reciprocal` on a tagged value negates all dimension exponents via `dim.negate()` and derives the display by negating atom exponents.
- **Dimensionless result from Div**: when `result_dim.is_dimensionless()`, the result is a plain `CalcValue::Float`, not a `TaggedValue`. For same-dim division, the amount is converted to a common scale before dividing.
- **`ConvertUnit` extended for compound targets**: `TaggedValue::convert_to()` is extended — if either the source or target is not a simple unit, it parses both as compound unit expressions, checks dim equality, and scales via `compound_to_si_scale`.
- **Parser extension in `parser.rs`**: after simple unit suffix matching fails, a new path splits the input into a number prefix and a unit expression suffix, then calls `parse_unit_expr_atoms`. Any input containing `/` after the number is tried as a compound unit.
- **`TaggedValue::new_compound` constructor**: creates a TaggedValue from an amount (as FBig parsed without f64 routing), a unit display string, and a pre-computed DimensionVector. Used by the parser for compound unit input.

## Source Files
- `src/engine/units.rs` — add `parse_unit_expr_atoms`, `atoms_to_dim`, `atoms_to_display`, `combine_atoms_mul`, `compound_to_si_scale`, `convert_tagged_to_unit`, `derive_display_from_dim`, extend `TaggedValue::convert_to`
- `src/input/parser.rs` — add `split_number_unit` helper; extend `try_parse_tagged` to parse compound unit expressions
- `src/engine/ops.rs` — update `tagged_binary_op` (Add/Sub dim check, Mul compound, Div compound/dimensionless), replace `tagged_compound_error_unary` for Sqrt/Reciprocal with new compound-aware handlers

## Commits
- (auto-linked by taproot link-commits)
- `9605617f8a00a75824d5a4f79ba4256271c16f4f` — (auto-linked by taproot link-commits)
- `aa091b03685ea489bec66a061288f0222942811c` — (auto-linked by taproot link-commits)
- `363334656107237b400be5f222b048d44c208253` — (auto-linked by taproot link-commits)

## Tests
- `src/engine/units.rs` — `test_parse_unit_expr_atoms_*` (AC-1/AC-2 atom parsing), `test_combine_atoms_mul_*` (AC-4/AC-5/AC-6 unit combination), `test_compound_to_si_scale` (conversion factor), `test_convert_tagged_compound` (AC-14 compound convert)
- `src/input/parser.rs` — `test_parse_compound_speed` (AC-1), `test_parse_compound_acceleration` (AC-2), `test_parse_unknown_unit_error` (AC-12); AC-18 (malformed expression) has no dedicated test — malformed input currently falls through to number-only parse rather than raising an explicit error
- `src/input/handler.rs` — AC-17: `test_insert_unit_slash_is_insert_char`, `test_insert_unit_all_chars_literal`, `test_insert_unit_enter_submits` (InsertUnit mode captures `/` literally)
- `src/engine/ops.rs` — `test_compound_div_speed` (AC-3), `test_compound_mul_cancellation` (AC-4), `test_compound_mul_area` (AC-5), `test_compound_mul_force` (AC-6), `test_compound_div_dimensionless` (AC-7), `test_compound_mul_scalar` (AC-8), `test_compound_sqrt_area` (AC-9), `test_compound_sqrt_odd_error` (AC-10), `test_compound_add_incompatible_error` (AC-11), `test_compound_session_restore` (AC-13), `test_compound_add_same_unit` (AC-15), `test_compound_reciprocal` (AC-16)

## DoR Resolutions
- condition: hints-spec | note: no changes to hints_pane.rs and no new AppMode introduced — compound unit operations reuse existing Normal mode key bindings (`*`, `/`, `√`, `1/x`); not affected | resolved: 2026-03-26
- condition: numeric-types | note: no new CalcValue variant introduced; compound-unit TaggedValues use the existing `amount: FBig` field unchanged; `TaggedValue::new_compound` takes a pre-parsed FBig (no f64 routing); display via existing `format_fbig`; serde round-trip unchanged (unit is a String, dim is DimensionVector from compound-unit-model) — not affected | resolved: 2026-03-26

## Status
- **State:** complete
- **Created:** 2026-03-26
- **Last verified:** 2026-03-26

## DoD Resolutions
- condition: document-current | note: README.md updated: added compound unit input grammar, compound arithmetic examples (multiplication, division, sqrt, reciprocal, same-compound-unit addition), compound unit conversion example (m/s → km/h). No new key bindings or modes — compound units reuse existing arithmetic operators. | resolved: 2026-03-26T09:09:36.672Z
- condition: document-current | note: Bug fix only: malformed compound unit expression (e.g. 9.8 m//s) now raises 'invalid unit expression: m//s' instead of a generic number-parse error. No new user-visible behaviour, key binding, or grammar rule introduced — the existing README compound unit input section (which documents space-before-slash requirement and example expressions) already accurately describes the valid input forms. Error message specificity improvements are internal quality improvements, not documentation-worthy behaviour changes. | resolved: 2026-03-26T13:07:15.141Z

