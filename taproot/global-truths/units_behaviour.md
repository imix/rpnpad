# Unit Parsing Rules — Behaviour-Level

Rules governing how unit strings are parsed and resolved. Apply to every behaviour and implementation that touches unit input or the unit registry.

---

## Unit Abbreviations Are Case-Sensitive

Unit abbreviations are matched exactly as registered. `oz` and `OZ` are distinct strings; only the registered casing is valid. This applies to both simple units and alias lookups.

Examples: `oz`, `kg`, `°F`, `N`, `kph` are valid; `OZ`, `KG`, `n`, `KPH` are not.

---

## Alias Table Takes Priority Over Unit Registry

During unit string resolution, the alias table is checked before the unit registry. If a string matches an alias, it resolves to the alias's canonical compound expression — even if a same-named entry exists in the unit registry.

**Implication for future unit additions:** adding a new unit to the registry whose abbreviation collides with an existing alias will be silently shadowed by the alias. Check the alias table before adding new units.

The alias table (`UNIT_ALIASES` in `src/engine/units.rs`) is the single authoritative source for aliases. The hints pane derives alias display from it; no separate list is maintained.
