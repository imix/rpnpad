# Implementation: tui

## Behaviour
../usecase.md

## Design Decisions
- **`CalcValue::Tagged(TaggedValue)` new variant**: adds `TaggedValue { amount: FBig, unit: String }` to the existing `CalcValue` enum. `amount` uses dashu `FBig` (not `f64`) to prevent precision noise in round-trip conversions (e.g. `ft → cm → ft` yielding `3.200000000000001`). `f64` is only used transiently during scale arithmetic and immediately lifted back to `FBig`. The unit is stored as its canonical display string; the static registry is looked up at use time. Serde uses dashu's built-in serde feature.
- **Static unit registry in `engine/units.rs`**: a `&[(abbrev, category, Option<f64>)]` slice with linear search. With ~20 entries, lookup is negligible. Temperature units have `None` for the scale factor and are handled by a separate affine conversion path.
- **Temperature stored in user's unit (not Kelvin)**: `98.6 °F` stores `98.6` with unit `"°F"`. Conversion uses direct affine formula. Normalising to Kelvin would work too, but this avoids a confusing intermediate and is simpler for affine arithmetic.
- **ASCII aliases for temperature** (`F`/`C` accepted as aliases for `°F`/`°C`): terminal keyboards cannot reliably input `°`. The canonical display is always `°F`/`°C`; aliases only apply to parser input and the `in` command.
- **Unit-aware ops at the `apply_op` dispatch level**: new `tagged_binary_op` and `tagged_unary_op` helpers intercept `Tagged` values before the existing plain-value closures. This keeps existing arithmetic code entirely unchanged.
- **Convert key `U` in Normal mode** → `AppMode::ConvertInput(String)` (form a). Form b: `in <unit>` typed in Alpha mode → `Action::ConvertUnit`. Both dispatch to the same action handler in `App::apply`.
- **`CalcError::IncompatibleUnits(String)` new error variant**: distinguishes unit errors from `InvalidInput` and `DomainError`. Used for: incompatible categories in arithmetic, compound-unit errors, convert-on-unitless, convert to incompatible unit.

## Source Files
- `src/engine/units.rs` — `UnitCategory`, `Unit`, static registry, `lookup_unit()`, `convert()`, `TaggedValue`
- `src/engine/mod.rs` — expose `units` module
- `src/engine/value.rs` — `CalcValue::Tagged(TaggedValue)`, updated display, serde, `to_f64`, `from_tagged`
- `src/engine/error.rs` — add `IncompatibleUnits(String)` variant
- `src/engine/ops.rs` — `tagged_binary_op`, `tagged_unary_op`; unit-aware dispatch for Add/Sub/Mul/Div/Negate/Abs/Floor/Ceil/Trunc/Round/Sqrt/Reciprocal/Square/Pow/trig/log
- `src/input/parser.rs` — extend `parse_value` to try `<number>[space]<unit>` suffix
- `src/input/commands.rs` — add `["in", unit]` → `Action::ConvertUnit`
- `src/input/action.rs` — add `ConvertUnit(String)`, `EnterConvertMode`, `ConvertChar(char)`, `ConvertBackspace`, `ConvertSubmit`, `ConvertCancel`
- `src/input/mode.rs` — add `AppMode::ConvertInput(String)`
- `src/input/handler.rs` — map `U` → `EnterConvertMode` in Normal; handle `ConvertInput` mode keys
- `src/tui/app.rs` — handle `ConvertUnit`, `EnterConvertMode`, `ConvertChar/Backspace/Submit/Cancel`
- `src/tui/widgets/hints_pane.rs` — add `U  unit` to stack ops; add ConvertInput mode hints; add compound unit branch showing COMPOUND UNIT section with source unit and prompt
- `README.md` — document unit tagging syntax, convert command, arithmetic behaviour

## Commits
- a56d5b4 taproot(physical-quantities/unit-aware-values): implement unit-tagged values
- de8609a taproot(physical-quantities/unit-aware-values): fix hints — grouped unit ref and UNITS section
- f87e884 taproot(physical-quantities/unit-aware-values): contextual hints — filter by category, show unit syntax
- `d63b4cd207f2abf20ac90071dc11bfde503dd4eb` — (auto-linked by taproot link-commits)

## Tests
- `src/engine/units.rs` — unit conversion math: weight (oz↔g, lb↔kg), length (ft↔m, in↔cm), temperature (°F↔°C); incompatible category error; unknown unit lookup
- `src/engine/value.rs` — `Tagged` display (`"1.9 oz"`), serde round-trip, `to_f64` passthrough
- `src/input/parser.rs` — parse `"1.9 oz"` (with space), `"1.9oz"` (no space), `"98.6F"` (alias), negative value, unknown unit error, ambiguous suffix
- `src/input/commands.rs` — `in g`, `in °F`, `in F` (alias), `in m` commands
- `src/engine/ops.rs` — AC-7 (same unit add), AC-8 (cross-unit add → p1's unit), AC-11 (incompatible categories error), AC-14 (scalar×tagged), AC-15 (plain+tagged error), AC-17 (same-unit div → dimensionless), AC-18 (tagged×tagged error), AC-20 (negate preserves unit)
- `src/tui/app.rs` — AC-3 (weight convert), AC-5 (°F→°C), AC-12 (incompatible convert error), AC-13 (convert on unitless error)
- `src/tui/widgets/hints_pane.rs` — AC-23 compound unit: ConvertInput shows COMPOUND UNIT section with source unit; does not show WEIGHT/LENGTH/TEMPERATURE groups

## DoD Resolutions
- condition: document-current | note: README updated with Physical Units section (unit input syntax, supported units table, conversion with U key and `in <unit>` Alpha command, arithmetic behaviour for same-category values, scalar multiplication, dimensionless division, temperature conversion). U key added to Normal Mode key reference table. Unit Mode key table added. `in <unit>` added to Alpha mode commands table. All user-visible behaviour is accurately reflected. | resolved: 2026-03-26
- condition: document-current | note: Hints pane UI fix only: CompoundInput now shows COMPOUND UNIT section instead of irrelevant Weight/Length/Temperature groups when a compound unit is at stack top. No new key bindings, modes, or conversion behaviors introduced. README already documents compound unit conversion (U key, 27.78 m/s → km/h example). Nothing to update. | resolved: 2026-03-26T12:12:16.059Z

- condition: document-current | note: FBig scale factor arithmetic is an internal precision fix — unit conversion produces clean results without noise (37.27704 not 37.27704000000001). No new user-visible behaviour, key bindings, or configuration options. README unit arithmetic documentation already accurately describes the behaviour. | resolved: 2026-03-26T07:47:43.161Z

- condition: document-current (rework) | note: Hint panel changes are internal UI only — no new user-visible behaviour. README already accurately describes U key and unit conversion. No README change required. | resolved: 2026-03-26
- condition: document-current (rework-2) | note: ConvertInput filtering and Insert mode unit hint are internal UI changes. README unit syntax documentation already covers "1.9 oz", "6 ft", "98.6F" as valid input forms. No README change required. | resolved: 2026-03-26
- condition: document-current (rework-3) | note: `TaggedValue.amount` changed from `f64` to `FBig` — internal precision fix. No user-visible behaviour change; README description of unit arithmetic is already accurate. No README change required. | resolved: 2026-03-26

## Status
- **State:** complete
- **Created:** 2026-03-26
- **Last verified:** 2026-03-26

## Notes
- AC-22 (session persistence) is covered by serde on `CalcValue::Tagged` — the existing `session.rs` serializes the full stack; no additional changes needed beyond the new variant having `#[derive(Serialize, Deserialize)]`.
- `unitless / tagged` (e.g. `3 ÷ 1.9oz`) is not covered by a spec AC. Implemented as error "compound unit not supported" to avoid silently producing a physically wrong `oz` result.
