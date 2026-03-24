# Behaviour: User reads the project README to discover and learn rpncalc

## Actor
Prospective user or new user (any platform, no rpncalc installed yet)

## Preconditions
- The GitHub repository is public
- `README.md` exists at the repository root
- The README install commands reference the real GitHub owner (not the `OWNER` placeholder)

## Main Flow
1. User discovers the project (via GitHub search, a link, word of mouth, or Homebrew search).
2. User opens the repository and reads the README.
3. User reads the project description and understands what rpncalc is and who it is for.
4. User selects an install path appropriate to their platform and preference (Homebrew, curl, or cargo).
5. User follows the install command for their chosen path.
6. User reads the quick-start example and runs their first calculation.
7. User refers to the key reference to learn additional operations.

## Alternate Flows
### User already has Rust toolchain
- **Trigger:** User sees the `cargo install` path and prefers it
- **Steps:**
  1. User runs `cargo install rpncalc`.
  2. Flow continues from step 6.
- **Outcome:** rpncalc installed via Cargo; user proceeds to quick start.

### User returns to README as a reference
- **Trigger:** Existing user forgets a key binding or wants to check a chord sequence
- **Steps:**
  1. User opens README directly to the key reference section.
  2. User finds the binding they need.
- **Outcome:** User resumes work without needing to consult the in-app hints pane.

## Postconditions
- User knows what rpncalc is and whether it suits their needs
- User has a working install command they can run immediately
- User understands the RPN stack model and HP48-style position numbering
- User can perform basic arithmetic, use chord sequences, and recall key bindings from the README

## Error Conditions
- **`OWNER` placeholder not replaced in install commands**: User sees `brew install OWNER/tap/rpncalc` or a curl URL with `OWNER` — command fails; maintainer must update the README with the real GitHub username before publishing.
- **README key reference out of sync with handler.rs**: User tries a key binding listed in the README that no longer works or has changed; maintainer must keep the key reference updated when adding or changing bindings.
- **Quick-start example uses removed or renamed operation**: User follows the example and gets an error; example must be validated against the current build before release.

## Flow

```mermaid
flowchart TD
    A[User finds project] --> B[Reads description]
    B --> C{Choose install path}
    C -->|macOS + Homebrew| D[brew install ...]
    C -->|Linux or macOS + curl| E[curl ... | sh]
    C -->|Has Rust| F[cargo install rpncalc]
    D --> G[Follow quick-start]
    E --> G
    F --> G
    G --> H[Refer to key reference as needed]
    H --> I[User is productive]
```

## Related
- `../install-via-homebrew/usecase.md` — downstream; README is the entry point that sends macOS users to the Homebrew path
- `../install-via-curl/usecase.md` — downstream; README is the entry point that sends Linux/macOS users to the curl path
- `../cargo-dist-release-pipeline/usecase.md` — upstream; the pipeline must have run before the install commands in the README are valid

## Acceptance Criteria

**AC-1: All three install paths documented**
- Given a user on any supported platform reads the README
- When they look for installation instructions
- Then they find distinct, copy-pasteable commands for Homebrew (macOS), curl (Linux/macOS), and cargo (any platform with Rust)

**AC-2: Quick-start example is complete and correct**
- Given a user has just installed rpncalc
- When they follow the quick-start section in the README
- Then they can complete at least one full calculation (push two values, apply an operation, see the result) using only the README as a guide

**AC-3: Key reference covers all modes**
- Given a user reads the key reference section
- When they look for any supported operation
- Then they find it listed under the correct mode (Normal, Insert, Browse, chord sequences) with the correct key and a brief description

**AC-4: Stack model is explained**
- Given a user unfamiliar with RPN reads the README
- When they read the stack model section
- Then they understand: values are pushed onto a stack; operations consume from the top; position 1 is the top; positions are numbered from the top down

**AC-5: Named registers are documented**
- Given a user wants to store an intermediate result
- When they read the README
- Then they find instructions for storing (`S` then register name) and recalling (`i` then `name RCL`) a named register value

**AC-6: README contains no unfilled placeholders before first release**
- Given the maintainer is preparing the first release
- When they review the README
- Then no `OWNER` placeholder appears in any install command or URL

## Implementations <!-- taproot-managed -->
- [README](./readme/impl.md)

## Status
- **State:** implemented
- **Created:** 2026-03-24
- **Last reviewed:** 2026-03-24

## Notes
- The README is the only pre-install documentation surface — no wiki, no docs site. Keep it comprehensive but scannable.
- Key reference must be kept in sync with `src/input/handler.rs` — the handler is the single source of truth for bindings.
- The `OWNER` placeholder appears in three places: the Homebrew tap name, the curl URL, and the `repository` field in `Cargo.toml`. All three must be updated together before first release.
- The in-app hints pane (discoverability intent) serves current users; the README serves prospective and returning users who don't have the tool open.
