Title: CLI defaults to interactive menu causing blocking in non-interactive environments

Summary
- Running `gc_cli` without a subcommand defaults to an interactive menu (`Demo::Menu` -> `interactive_pick()`), which blocks on stdin in CI/automation. This makes non-interactive usage brittle.

Details
- Location: `crates/gc_cli/src/main.rs`:
  - `Args { demo: Option<Demo> }` and `match args.demo.clone().unwrap_or(Demo::Menu)`.
  - `interactive_pick()` reads from stdin and falls back to `Demo::Mapgen` only on invalid input.

Impact
- Headless environments (e.g., CI, docker) may hang waiting for input if subcommand omitted.

Proposed Fix
- Default to a non-interactive demo (e.g., `Mapgen`) when no subcommand is provided.
- Keep `--demo menu` as an explicit opt-in for interactive mode.
- Alternatively, detect `stdin` not being a TTY and auto-select a default non-interactive demo.

Acceptance Criteria
- `gc_cli` runs to completion without input when invoked with no subcommand.
- Interactive menu still works when explicitly requested.
- Add a small doc snippet in `README.md` explaining the behavior.
