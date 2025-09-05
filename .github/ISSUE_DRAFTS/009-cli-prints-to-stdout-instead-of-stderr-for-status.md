Title: CLI uses stdout for status/progress; consider stderr for non-data messages

Summary
- `gc_cli` prints mixed status and data messages to stdout (e.g., ASCII maps and progress). In pipelines, it's helpful to separate data from logs by sending status/progress to stderr.

Details
- Location: `crates/gc_cli/src/main.rs` various `println!` calls.

Proposal
- For demos that output consumable data (e.g., serialized save JSON length, map), keep data on stdout and route log/status to `eprintln!`.
- Optionally add a `--quiet` or `--log-level` flag.

Acceptance Criteria
- Consistent separation of data vs logs; optional flag to silence logs.
