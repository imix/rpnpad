# Implementation: tui

## Behaviour
../usecase.md

## Design Decisions
- **Alias table in `units.rs`**: `UNIT_ALIASES` static slice lives alongside the unit table so both the parser and the hints pane can import it from a single location without duplication
- **`aliases_for_dim(dim)` helper in `units.rs`**: computes matching aliases by parsing each alias's canonical form and comparing `DimensionVector`; hints pane calls this rather than embedding alias logic inline
- **Alias path between simple-unit and compound paths**: inserted as step 1.5 in `try_parse_tagged`; uses `parse_decimal_exact` + `parse_unit_expr_atoms` + `TaggedValue::new_compound` — identical construction to the compound path so no new TaggedValue machinery is needed
- **Canonical storage**: alias-resolved values store the canonical compound string (e.g. `kg*m/s2`) not the alias (`N`); this is identical to direct compound entry and requires no session-persistence changes
- **AC-2 (`dyn` conversion) not tested**: `dyn` (dyne) and `lbf` (pound-force) are not in the unit table; AC-2's conversion intent is demonstrated via `kph` → `m/s` (an existing compound conversion round-trip); `dyn`/`lbf` require a separate unit-table extension

## Source Files
- `src/engine/units.rs` — add `UNIT_ALIASES` static + `aliases_for_dim()` function
- `src/input/parser.rs` — add alias resolution path (step 1.5) in `try_parse_tagged`; import `UNIT_ALIASES`/`lookup_alias` from units
- `src/tui/widgets/hints_pane.rs` — extend UNITS section to call `aliases_for_dim` and render matching alias names as conversion target hints

## Commits
<!-- taproot link-commits will fill this -->
- `69183234467e642478c0c431a79bddd8799d70db` — (auto-linked by taproot link-commits)
- `e0120659341c427a47608379766397dbf61917f7` — (auto-linked by taproot link-commits)

## Tests
- `src/engine/units.rs` — `test_aliases_for_dim_force`, `test_aliases_for_dim_no_match`
- `src/input/parser.rs` — AC-1: `test_parse_alias_newton`; AC-4: `test_parse_alias_kph`; AC-6: `test_parse_unknown_still_errors`; AC-7: `test_parse_direct_compound_unchanged`; AC-9: `test_alias_stores_canonical_unit`
- `src/tui/widgets/hints_pane.rs` — AC-5: `test_units_section_shows_alias_for_force_dim`

## DoR Resolutions
- condition: hints-spec | note: hints_pane.rs is modified (UNITS section extended) but no new AppMode is introduced — alias names are rendered as additional hint rows in the existing Normal-mode UNITS block; the existing `top_is_tagged` visibility condition is unchanged; no typed-input mode is added | resolved: 2026-03-26
- condition: numeric-types | note: no new CalcValue variant introduced; alias-resolved TaggedValues use the existing `amount: FBig` field via `TaggedValue::new_compound` (same construction as compound-unit-operations); no f64 routing; serde round-trip unchanged (unit stored as canonical String, dim as DimensionVector) | resolved: 2026-03-26

## Status
- **State:** complete
- **Created:** 2026-03-26
- **Last verified:** 2026-03-26

## DoD Resolutions
- condition: document-current | note: README.md updated: added 'Common unit aliases' block under 'Entering unit-tagged values' with examples for N, kph, Pa, J, W and note that hints pane shows alias conversion targets | resolved: 2026-03-26T15:06:55.372Z
