# rpnpad

An HP48-style RPN calculator for the terminal. Stack-based, keyboard-driven, no mouse required.

```
╭─ rpnpad ──────────────────────────────────────────────╮
│╭─ Stack ────────────────╮╭─ Hints ───────────────────╮│
││  4:                    ││ ARITHMETIC                 ││
││  3:                    ││ +  add    -  sub           ││
││  2:          3.14159…  ││ *  mul    /  div           ││
││  1:               42   ││ ^  pow    %  mod           ││
││                        ││ !  fact   n  neg           ││
││                        ││                            ││
││                        ││ STACK                      ││
││                        ││ s  swap   d  drop          ││
││                        ││ p  dup    r  rot           ││
││                        ││ u  undo   y  yank          ││
││                        ││ S  store                   ││
││                        ││                            ││
││                        ││ t› trig   l› log           ││
││                        ││ f› fn     c› const         ││
││                        ││ m› mode   x› base          ││
│╰────────────────────────╯╰───────────────────────────╯│
│>                                                       │
│                                                        │
│────────────────────────────────────────────────────────│
│[NORMAL]                                      RAD  DEC  │
╰────────────────────────────────────────────────────────╯
```

## Install

### macOS — Homebrew

```sh
brew install imix/tap/rpnpad
```

Or if you have already tapped the repository:

```sh
brew tap imix/tap
brew install rpnpad
```

### Linux — Snap

```sh
snap install rpnpad
```

Requires [snapd](https://snapcraft.io/docs/installing-snapd) (pre-installed on Ubuntu; available on most distros).

### Linux and macOS — curl installer

```sh
curl -fsSL https://github.com/imix/rpnpad/releases/latest/download/rpnpad-installer.sh | sh
```

The installer places `rpnpad` in `~/.cargo/bin/`. If that directory is not on your `PATH`, add this line to your shell profile (`.bashrc`, `.zshrc`, etc.):

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

### From source — cargo

```sh
cargo install rpnpad
```

Requires a Rust toolchain ([rustup.rs](https://rustup.rs)).

---

## Quick Start

Launch the calculator:

```sh
rpnpad
```

Push two numbers and add them:

1. Type `3` — starts number entry (INSERT mode)
2. Press `Enter` — pushes `3` onto the stack
3. Type `4`, press `Enter` — pushes `4`
4. Press `+` — consumes positions 1 and 2, pushes `7`

Press `q` to quit.

---

## Stack Model

rpnpad uses an RPN (Reverse Polish Notation) stack, in the style of the HP48 calculator.

- Values are **pushed** onto the stack; operations **consume** values from the top
- **Position 1** is always the top of the stack (most recently pushed)
- **Position 2** is the next value down, and so on
- Operations like `+` consume positions 1 and 2, and push one result back

Example — compute `(3 + 4) × 2`:

```
Push 3    →  1: 3
Push 4    →  2: 3   1: 4
Press +   →  1: 7
Push 2    →  2: 7   1: 2
Press *   →  1: 14
```

---

## Key Reference

### Normal Mode

| Key | Action |
|-----|--------|
| `0`–`9`, `.`, `-` | Start number entry (enters Insert mode) |
| `Enter` | Duplicate position 1 |
| `+` `-` `*` `/` | Add, subtract, multiply, divide |
| `^` | Power (x^y: position 2 ^ position 1) |
| `%` | Modulo |
| `!` | Factorial |
| `n` | Negate position 1 |
| `q` | Square position 1 (x²) |
| `w` | Square root of position 1 (√x) |
| `s` | Swap positions 1 and 2 |
| `d` | Drop position 1 |
| `p` | Duplicate position 1 |
| `r` | Rotate top 3 (1→3, 2→1, 3→2) |
| `↑` | Enter Browse mode (roll any depth to top) |
| `u` | Undo |
| `Ctrl-r` | Redo |
| `y` | Yank (copy position 1 to clipboard) |
| `S` | Store position 1 to a named register |
| `U` | Convert position 1 to a different unit (enters Unit mode) |
| `i` | Enter Alpha mode (for register commands) |
| `Q` | Quit |

### Insert Mode

Entered by typing a digit or starting a number. All printable characters build the number buffer.

| Key | Action |
|-----|--------|
| `Enter` | Push value onto stack |
| `Esc` | Cancel, discard input |
| `Backspace` | Delete last character |
| `Space` | Enter unit expression context — all subsequent keys are literal (no shortcuts). Use this to type compound units like `1 m/s`. |
| `+` `-` `*` `/` `^` `%` `!` | Push value then apply operation (only fires before a space is typed) |
| `q` | Push value then square it (x²) |
| `w` | Push value then take square root (√x) |
| `s` `d` `p` `r` `n` | Push value then apply stack op |

### Browse Mode

Entered by pressing `↑` in Normal mode when the stack has ≥ 2 items. A cursor highlights a stack position.

| Key | Action |
|-----|--------|
| `↑` | Move cursor deeper (toward older values) |
| `↓` | Move cursor toward position 1 |
| `Enter` | Roll the highlighted item to position 1 |
| `Esc` | Cancel, stack unchanged |

### Chord Sequences

Press the leader key to enter a chord sub-mode, then press the second key.
Press `Esc` at any point to cancel.

**`t` — Trig**

| Key | Operation |
|-----|-----------|
| `s` | sin |
| `c` | cos |
| `a` | tan |
| `S` | asin |
| `C` | acos |
| `A` | atan |

**`l` — Log**

| Key | Operation |
|-----|-----------|
| `l` | ln |
| `L` | log₁₀ |
| `e` | eˣ |
| `E` | 10ˣ |

**`f` — √ (functions)**

| Key | Operation |
|-----|-----------|
| `s` | √x |
| `q` | x² |
| `r` | 1/x |
| `a` | |x| |

**`c` — Constants**

| Key | Value |
|-----|-------|
| `p` | π |
| `e` | e |
| `g` | φ (golden ratio) |

**`m` — Angle Mode**

| Key | Mode |
|-----|------|
| `d` | Degrees |
| `r` | Radians |
| `g` | Gradians |

**`x` — Base**

| Key | Base |
|-----|------|
| `c` | Decimal |
| `h` | Hexadecimal |
| `o` | Octal |
| `b` | Binary |

**`X` — Hex Style** (active when base is Hex)

| Key | Format |
|-----|--------|
| `c` | `0xFF` |
| `a` | `$FF` |
| `s` | `#FF` |
| `i` | `FFh` |

### Unit Mode

Entered by pressing `U` in Normal mode when the stack has ≥ 1 item.

| Key | Action |
|-----|--------|
| (any text) | Build the target unit abbreviation |
| `Enter` | Convert position 1 to the entered unit |
| `Esc` | Cancel, stack unchanged |
| `Backspace` | Delete last character |

---

## Physical Units

Tag values with physical units to perform unit-aware calculations.

### Entering unit-tagged values

Type a number followed by a unit abbreviation (space optional), then press `Enter`:

```
1.9 oz      → pushes 1.9 oz onto the stack
98.6F       → pushes 98.6 °F (F is an alias for °F)
6 ft        → pushes 6 ft
```

Compound units (built from two or more base units) require a **space** between the number and the unit expression, because `/` is otherwise treated as a division shortcut in Insert mode:

```
9.8 m/s2    → pushes 9.8 m/s² (acceleration)
100 km/h    → pushes 100 km/h (speed)
80 kg*m/s2  → pushes 80 kg·m/s² (force)
25 m2       → pushes 25 m² (area)
```

The space triggers unit expression context — all subsequent keys (including `/`, `*`, `+`, etc.) are treated as literal characters until `Enter`. Simple units without `/` remain space-optional (`1.9oz` and `1.9 oz` both work).

**Input grammar:** `<numerator>/<denominator>`, where each part contains unit atoms joined by `*` or space. Exponents are plain ASCII digits (`m2`, `s2`, not `m²`). Both `*` and space are accepted as numerator separators.

**Common unit aliases** — short names for derived SI units are accepted directly:

```
9.8 N       → pushes 9.8 kg·m/s² (newton — force)
100 kph     → pushes 100 km/h (speed)
101325 Pa   → pushes 101325 kg/m·s² (pascal — pressure)
4.2 J       → pushes 4.2 kg·m²/s² (joule — energy)
60 W        → pushes 60 kg·m²/s³ (watt — power)
```

The alias is resolved to its canonical compound form on entry; the stack and all downstream operations see the canonical unit. When a stack value's dimension matches a known alias, the hints pane UNITS section shows the alias name as a named conversion target (e.g. `→ N`).

### Supported units

| Category | Units |
|----------|-------|
| Weight | `oz`, `lb`, `g`, `kg` |
| Length | `mm`, `cm`, `m`, `km`, `in`, `ft`, `yd`, `mi` |
| Time | `s`, `min`, `h` |
| Temperature | `°C`, `°F` (aliases: `C`, `F`, `degC`, `degF`) |

### Converting units

Press `U` in Normal mode, type the target unit, then `Enter`:

```
1.9 oz   → stack: [1.9 oz]
U        → enter Unit mode
g Enter  → converts to 53.864 g

27.78 m/s → stack: [27.78 m/s]
U         → enter Unit mode
km/h      → converts to 100 km/h
```

Or use Alpha mode with `in <unit>`:

```
i           → enter Alpha mode
in g Enter  → converts position 1 to grams
```

### Arithmetic with tagged values

Operations between two values of the **same unit** (or same dimension) auto-convert position 2 into position 1's unit before operating:

```
1 kg        → stack: [1 kg]
500 g       → stack: [1 kg, 500 g]
+           → result: 1.5 kg  (500 g converted to 0.5 kg first)
```

Multiplying or dividing a tagged value by a plain number preserves the unit:

```
6 ft        → stack: [6 ft]
2           → stack: [6 ft, 2]
*           → result: 12 ft
```

Dividing two values of the same dimension yields a dimensionless ratio:

```
6 ft        → stack: [6 ft]
2 ft        → stack: [6 ft, 2 ft]
/           → result: 3  (dimensionless)
```

**Compound unit arithmetic** — multiplying or dividing values with different dimensions produces a compound result:

```
100 km      → stack: [100 km]
2 h         → stack: [100 km, 2 h]
/           → result: 50 km/h

5 m         → stack: [5 m]
3 m         → stack: [5 m, 3 m]
*           → result: 15 m2

80 kg       → stack: [80 kg]
9.8 m/s2    → stack: [80 kg, 9.8 m/s2]
*           → result: 784 kg*m/s2

50 km/h     → stack: [50 km/h]
2 h         → stack: [50 km/h, 2 h]
*           → result: 100 km  (h cancels)
```

`√` (square root) halves all dimension exponents — error if any exponent is odd:

```
25 m2       → stack: [25 m2]
w           → result: 5 m

4 m/s       → stack: [4 m/s]
w           → error: non-integer unit exponent after sqrt
```

`1/x` (reciprocal) negates all dimension exponents:

```
4 m/s2      → stack: [4 m/s2]
fr          → result: 0.25 s2/m
```

Adding or subtracting compound-unit values requires identical dimensions:

```
1 m/s       → stack: [1 m/s]
2 m/s       → stack: [1 m/s, 2 m/s]
+           → result: 3 m/s

1 m/s       → stack: [1 m/s]
1 m/s2      → stack: [1 m/s, 1 m/s2]
+           → error: incompatible units
```

Temperature arithmetic (°C and °F are not additive — only conversion is meaningful):

```
98.6 F      → stack: [98.6 °F]
U           → Unit mode
C Enter     → result: 37 °C
```

---

## Named Registers

Store intermediate results in named registers for later recall.

### Store with `S` shortcut

1. Ensure the value is at position 1
2. Press `S` — mode bar shows `[INSERT]` and prompts for a name
3. Type a register name (e.g. `r1`, `total`, `x`)
4. Press `Enter` — value is stored; position 1 is unchanged

### Store, recall, and delete via Alpha mode

Press `i` to enter Alpha mode, then type a command and press `Enter`:

| Command | Action |
|---------|--------|
| `name STORE` | Pop position 1 and store it under `name` |
| `name RCL` | Push the value stored in `name` onto the stack |
| `name DEL` | Delete the register `name` |
| `in <unit>` | Convert position 1 to `<unit>` (e.g. `in kg`, `in °F`) |
| `RESET` | Clear stack and registers |

Example — store π, do some work, recall it:

```
cp          → pushes π to position 1
S Enter pi  → stores π as "pi", stack unchanged
…           → do other calculations
i           → enter Alpha mode
pi RCL      → pushes π back onto the stack
```

Active registers appear in the Hints pane for quick reference.

---

## Configuration

Settings persist automatically between sessions (saved to `~/.config/rpnpad/state.json`).

### Angle mode

Default: `RAD`. Change with the `m` chord:

- `md` — degrees
- `mr` — radians
- `mg` — gradians

### Numeric base

Default: `DEC`. Change with the `x` chord:

- `xc` — decimal
- `xh` — hexadecimal
- `xo` — octal
- `xb` — binary

When in hex mode, use `X` to switch the display style (`0xFF`, `$FF`, `#FF`, `FFh`).

### Session persistence

The stack, registers, angle mode, and base are saved when you quit and restored when you next launch rpnpad. No manual save step is required.

---

## Releasing

1. Bump the version in `Cargo.toml` (patch / minor / major as appropriate)
2. Run `cargo check` to update `Cargo.lock`
3. Commit: `git commit -am "chore: bump version to X.Y.Z"`
4. Tag: `git tag vX.Y.Z`
5. Push: `git push origin main vX.Y.Z`

GitHub Actions picks up the tag and runs the full release pipeline automatically — builds binaries for Linux and macOS, creates the GitHub Release with archives and installer script, updates the Homebrew formula, and publishes the snap.

If the build fails: delete the tag (`git push origin :vX.Y.Z`), fix the issue, and re-tag.

---

## Building from Source

```sh
git clone https://github.com/imix/rpnpad
cd rpnpad
cargo build --release
./target/release/rpnpad
```

Requires Rust stable (1.70+).

---

## Built with Taproot

rpnpad is developed using [Taproot](https://github.com/felixwatts/taproot) — a lightweight requirements traceability system that keeps behaviour specs, implementation records, and source code in sync. Every feature in rpnpad traces from a user-observable behaviour spec down to the commit that delivered it.
