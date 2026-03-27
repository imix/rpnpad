# Conventions — Intent-Level

Global conventions that apply across all intents, behaviours, and implementations.

---

## Function Discoverability

Every user-visible function, alias, or operation must be surfaced in the hints pane.

This applies to:
- Mathematical operations and chord sequences
- Unit aliases (e.g. `N`, `kph`, `Pa`)
- Direct-access shortcuts (e.g. `q`→Square, `w`→Sqrt)

If a capability exists but is absent from the hints pane, it is considered undiscoverable and the implementation is incomplete.
