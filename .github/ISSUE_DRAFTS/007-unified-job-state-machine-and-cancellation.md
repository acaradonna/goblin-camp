Title: Introduce unified Job state machine and cancellation semantics

Summary
- Current job handling spreads state across `JobBoard` (Vec), `ActiveJobs` (HashMap), and `AssignedJob` on workers. There is no explicit job state (Pending/InProgress/Completed/Cancelled), leading to edge cases (e.g., missing items, removed jobs) being handled ad-hoc.

Details
- Locations: `crates/gc_core/src/jobs.rs` and `crates/gc_core/src/systems.rs`.
- Missing concepts: job retry/backoff, cancellation when prerequisites disappear, reassignment if worker becomes unavailable.

Impact
- Harder to reason about lifecycle; increases risk of orphaned jobs and inconsistent assignments.

Proposal
- Define `JobState { Pending, InProgress, Completed, Cancelled }`.
- Move jobs through states explicitly; ensure `AssignedJob` references include state checks.
- Centralize transitions in a job manager system; emit events for completion/cancellation.

Acceptance Criteria
- New state machine implemented; existing tests updated.
- Add tests for cancellation scenarios (item missing, tile already mined, worker despawned).
