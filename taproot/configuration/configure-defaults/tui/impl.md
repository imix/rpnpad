# Implementation: Configure Defaults

## Behaviour
../usecase.md

## Design Decisions
- Intermediate `ConfigToml` struct with all-`Option` fields allows partial
  configs — only present keys override defaults; absent keys keep `Config::default()`
- Invalid field values are silently ignored (no error on launch) — each
  field is matched individually; a bad value leaves the default in place
- Config is loaded once at startup, before session restore; no hot-reload
- Config path: `~/.rpnpad/config.toml` via `dirs::home_dir()`

## Source Files
- `src/config/config.rs` — Config, ConfigToml, load_from_path(),
  Config::load(), Config::default()
- `src/main.rs` — calls Config::load() at startup before session restore

## Commits
- 7066c63 feat: complete Epics 2–4 + layout width cap

## Tests
- `src/config/config.rs` (inline `#[cfg(test)]`) — defaults, each field
  individually, case-insensitive values, partial configs, missing file,
  malformed TOML, invalid values, zero precision guard

## Status
- **State:** complete
- **Created:** 2026-03-21
- **Last verified:** 2026-03-27

## Notes
None
