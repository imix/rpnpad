# Intent: Configuration

## Goal
Enable users to persist personal workflow defaults so rpncalc behaves
correctly from the first keypress of every session.

## Stakeholders
- **CLI power user**: wants their preferred angle mode, numeric base,
  precision, and undo depth remembered across restarts without re-entering
  them each session

## Success Criteria
- All display and behaviour defaults (angle mode, base, precision,
  representation style, undo depth, session persistence) are configurable
  via config.toml
- Missing or malformed config file is handled gracefully — rpncalc launches
  with sensible defaults rather than erroring

## Constraints
- Config file lives at `~/.rpncalc/config.toml` (XDG base dirs respected)
- Invalid individual field values silently fall back to defaults; the
  whole file need not be valid for valid fields to apply
- Phase 2 unit conversion rules (`units.toml`) are out of scope

## Behaviours <!-- taproot-managed -->
- [User configures rpncalc defaults via config.toml](./configure-defaults/usecase.md)


## Status
- **State:** active
- **Created:** 2026-03-20
