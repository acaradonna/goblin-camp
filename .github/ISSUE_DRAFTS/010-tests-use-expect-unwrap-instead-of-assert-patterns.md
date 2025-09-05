Title: Tests rely on unwrap/expect; prefer assertion patterns for clearer failures

Summary
- Several tests use `unwrap()`/`expect()` to access resources/components, which can obscure failure contexts compared to explicit assertions.

Details
- Locations include: `crates/gc_core/tests/*.rs`, `crates/gc_tui/tests/snapshot_render.rs`.

Proposal
- Replace `unwrap()`/`expect()` in tests with `assert!(opt.is_some(), "...")` or `assert_eq!`/`assert_matches!` where clearer.
- This yields better failure diffs and reduces panic-only signal.

Acceptance Criteria
- Tests updated to favor explicit assertions; no behavior change.
