# Intent: Physical Quantities and Unit Conversion

## Stakeholders
- **CLI power user**: engineer, scientist, or anyone doing real-world calculations — their interest is staying inside rpnpad when working with quantities that carry units (weight, length, temperature), rather than switching to a conversion website or external calculator

## Goal
Enable users to tag stack values with physical units and convert between compatible units with a single command. Arithmetic between unit-tagged values preserves the unit, so calculations involving real-world quantities flow naturally inside rpnpad without context-switching.

## Success Criteria
- [x] A value on the stack can be tagged with a unit (e.g. `1.9 oz`, `6 ft`, `98.6 °F`)
- [x] A tagged value converts to a compatible unit on demand (e.g. `1.9 oz` → `53.86 g`)
- [x] Arithmetic between same-unit values preserves the unit (e.g. `1.9 oz + 2 oz` → `3.9 oz`)
- [x] Supported categories on first delivery: weight, length, temperature (imperial ↔ metric)
- [ ] The unit model supports compound units (e.g. m/s, kg·m/s²) internally from the start, so derived units can be added without refactoring

## Constraints
- Compound units (speed, pressure, etc.) are not user-facing in the first delivery — the architecture must accommodate them, but they ship as a later behaviour
- Currency conversion is out of scope
- Cross-category arithmetic (e.g. `1 oz + 1 ft`) must produce a clear error, not a silent wrong result

## Behaviours <!-- taproot-managed -->
- [Unit-Aware Values](./unit-aware-values/usecase.md)
- [Compound Unit Data Model](./compound-unit-model/usecase.md)
- [Compound Unit Operations](./compound-unit-operations/usecase.md)

## Status
- **State:** active
- **Created:** 2026-03-25
- **Last reviewed:** 2026-03-26

## Notes
- The HP48 unit object model is a useful reference: values carry a unit tag, and the calculator handles conversion automatically when units are compatible.
- Phase 2 (compound units, speed, pressure, etc.) is a natural extension of this intent once the core model is in place.
