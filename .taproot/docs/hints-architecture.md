# Hints Pane Architecture

Rules for the `hints_pane.rs` widget. Every spec that introduces a new `AppMode`
or modifies key bindings must have ACs that satisfy these rules before the impl
is declared ready.

---

## Purpose

The hints pane is a **live reference**, not a marketing panel. Its job is to
answer the question "what can I press right now?" — completely and accurately.
It must never show informal examples in place of an exhaustive list.

---

## Normal mode structure

Normal mode renders sections in this fixed order. Each section is conditional
on context (stack depth, active features) but the order never changes:

| Order | Section | Condition |
|-------|---------|-----------|
| 1 | **ARITHMETIC** | depth ≥ 1 (binary ops at depth ≥ 2; unary-only at depth = 1) |
| 2 | **STACK** | always |
| 3 | Chord leaders | depth ≥ 1 |
| 4 | **SESSION** | always |
| 5 | **REGISTERS** | registers non-empty |
| 6 | **UNITS** | stack top is a tagged value |

**Key grouping rule**: a key belongs to the section that describes its
*function*, not the section it was most convenient to append to. Examples:
- `U` (convert unit) → UNITS, not STACK
- `S` (store register) → STACK (because it acts on the stack value)
- `Q` (quit) → SESSION

Any key that doesn't fit an existing section gets its own section before
SESSION. Never add unrelated keys to an existing section as a shortcut.

---

## Modal hint panels

When any mode other than `Normal` or `Chord` is active, the hint panel
**replaces** the Normal mode view entirely. The modal panel must show:

1. **Mode header** — styled dim, e.g. `CONVERT TO UNIT`, `PRECISION`, `STORE NAME`
2. **Blank line**
3. **Key table** — every key that has an effect in this mode, one per line:
   `<key>  <action>`
4. **Blank line** (if a reference section follows)
5. **Reference content** — all valid inputs, grouped and labelled (see below)

A modal panel that shows "e.g. …" or a partial list fails rule 5. Every valid
input must appear.

### Chord submenus

Chord submenus follow the same modal pattern with a `[LABEL]` header and a
complete op table. No reference section is needed (all inputs are the key table
itself).

---

## Reference content rules

When a mode accepts free-form text (unit names, register names, precision
digits), the panel must show a **complete, grouped reference** of all valid
inputs.

### Grouping

- Group inputs by **semantic category** (e.g. Weight / Length / Temperature)
- Within each group: sort alphabetically by abbreviation
- Label each group with a dim header line

### Format

```
WEIGHT    oz  lb  g  kg
LENGTH    mm  cm  m  km  in  ft  yd  mi
TEMP      °C  °F  (also: C  F  degC  degF)
```

Aliases (inputs that resolve to the same canonical value) may be shown
parenthetically after the canonical form, or on a second line. They must be
shown — omitting aliases forces the user to guess.

---

## New mode checklist (DoR gate)

Before an impl that introduces a new `AppMode` or modifies `hints_pane.rs`
can be declared ready, the parent usecase.md must contain ACs covering:

- [ ] The mode header text
- [ ] Every key available in the mode and its label
- [ ] If the mode accepts typed input: the complete grouped reference of valid inputs
- [ ] The condition under which the Normal mode section for the triggering key is shown
      (e.g. "UNITS section shown only when stack top is a tagged value")

---

## Current sections reference

### STACK section keys

| Key | Label | Condition |
|-----|-------|-----------|
| `s` | swap | always |
| `Bksp` | drop | always |
| `p` | dup | always |
| `R` | rot | always |
| `u` | undo | always |
| `y` | yank | always |
| `S` | store | always |

### UNITS section keys

| Key | Label | Condition |
|-----|-------|-----------|
| `U` | convert | stack top is tagged |

### SESSION section keys

| Key | Label | Condition |
|-----|-------|-----------|
| `Q` | quit | always |
