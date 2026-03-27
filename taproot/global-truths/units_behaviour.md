# Unit Rules — Behaviour-Level

Rules governing how unit strings are parsed, resolved, and normalised in arithmetic. Apply to every behaviour and implementation that touches unit input, arithmetic, or the unit registry.

---

## Unit Abbreviations Are Case-Sensitive

Unit abbreviations are matched exactly as registered. `oz` and `OZ` are distinct strings; only the registered casing is valid. This applies to both simple units and alias lookups.

Examples: `oz`, `kg`, `°F`, `N`, `kph` are valid; `OZ`, `KG`, `n`, `KPH` are not.

---

## Alias Table Takes Priority Over Unit Registry

During unit string resolution, the alias table is checked before the unit registry. If a string matches an alias, it resolves to the alias's canonical compound expression — even if a same-named entry exists in the unit registry.

**Implication for future unit additions:** adding a new unit to the registry whose abbreviation collides with an existing alias will be silently shadowed by the alias. Check the alias table before adding new units.

The alias table (`UNIT_ALIASES` in `src/engine/units.rs`) is the single authoritative source for aliases. The hints pane derives alias display from it; no separate list is maintained.

---

## Binary Unit Arithmetic Normalises to Position 1's Units

In all binary unit arithmetic (`+`, `−`, `×`, `÷`), position 2's value is normalised before the operation: for each dimension where position 2 has a unit atom and position 1 also has a unit atom for the same dimension, position 2's atom is converted to match position 1's. Dimensions present only in position 2 are left unchanged.

Examples:
- `1.9 oz` (p2) `+ 2 g` (p1) → convert `oz` to `g` → result in `g` ✓
- `5 ft` (p2) `× 3 km` (p1) → convert `ft` to `km` → `15 km2` ✓
- `9.8 m/s2` (p2) `× 80 kg` (p1) → `kg` has no length/time match in p1 → `kg*m/s2` unchanged ✓
- `100 km` (p2) `÷ 2 h` (p1) → no shared dimension → `km/h` ✓

**Implication:** stack order determines the unit of the result. This is consistent across all operators.
