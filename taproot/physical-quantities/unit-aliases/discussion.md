# Discussion: Unit Aliases

Skill: tr-behaviour

## Pivotal Questions

**Should output alias display (collapsing `kg*m/s2` → `N`) be in scope?**
No. Output aliases require separating the canonical storage form from the display form throughout `TaggedValue`, and introduce ambiguity when a dimension matches multiple aliases (e.g. `J` and `N*m` for energy). This is a larger architectural decision deferred to the backlog.

**Should alias-aware conversion hints be in scope?**
Yes. Surfacing `N` in the hints pane UNITS section when stack top is force-dimensioned is purely additive to the hints renderer — no storage or display entanglement. It delivers practical value (user sees `→ N` and can type it) at low cost.

**How large should the initial alias table be?**
A curated handful of high-frequency SI-derived units: `N`, `kph`, `Pa`, `J`, `W`, `Hz`. No user-defined aliases. The table is the single authoritative source.

## Alternatives Considered

**User-defined aliases via config.toml** — rejected for this behaviour. Adds config surface area and edge cases (collision with unit names, persistence). Deferred if demand arises.

**Alias lookup as a fallback in the compound-unit parser** — rejected in favour of a first-pass lookup table. The alias table is checked before compound parsing, which is cleaner and avoids the compound parser needing to know about aliases.

## Decision

Input-only aliases with alias-aware conversion hints. Canonical form stored and displayed; alias only influences parsing (input) and hints (discovery).

## Open Questions

- Should `kPa` (kilo-pascal) be in the initial table, or is `Pa` sufficient? (The SI prefix mechanism may handle `kPa` automatically if SI prefixes are ever added to the compound parser.)
- `Hz` = `1/s` — is a dimensionless-numerator compound unit handled correctly by `parse_unit_expr_atoms`?
