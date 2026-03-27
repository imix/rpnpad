# Glossary — Intent-Level

Definitions and factual constraints that apply across the entire project.

---

## HP48-Style Position Numbering

The stack uses HP48-style position numbering: position 1 is the most recently entered value (the top of the stack). Positions increase downward toward older values.

- Position 1 = top = most recent value
- Position 2 = second from top
- Position N = Nth from the top

All specs that reference stack positions use this convention. "Position 1" and "top of stack" are interchangeable terms.

---

## Config and Session Directory

rpnpad stores all user data under `~/.rpnpad/`:

- `~/.rpnpad/config.toml` — startup defaults (angle mode, base, precision, etc.)
- `~/.rpnpad/session.json` — persisted stack and register state

XDG base directories are respected where applicable. The directory is created on first launch if absent.

---

## Atomic Session Writes

All writes to `session.json` use an atomic write-to-temp → rename pattern. The prior file is never overwritten in place. No corrupt partial state can result from an interrupted write (power loss, kill signal, etc.).

This is a constraint on the `state-and-memory` intent: any implementation of session persistence must use this pattern.
