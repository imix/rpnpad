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
| `i` | Enter Alpha mode (for register commands) |
| `Q` | Quit |

### Insert Mode

Entered by typing a digit or starting a number. All printable characters build the number buffer.

| Key | Action |
|-----|--------|
| `Enter` | Push value onto stack |
| `Esc` | Cancel, discard input |
| `Backspace` | Delete last character |
| `+` `-` `*` `/` `^` `%` `!` | Push value then apply operation |
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
