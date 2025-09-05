Title: Clarify and harden ECS schedule ordering to avoid side effects

Summary
- The default schedule mixes systems whose semantics depend on ordering (designation->assignment->execution->auto-haul). Some systems have side effects based on previous passes (e.g., item spawn queue). Minor ordering changes could break invariants.

Details
- Location: `crates/gc_core/src/bootstrap.rs` `build_default_schedule()`.
- Systems chained: `designation_dedup_system`, `designation_to_jobs_system`, `job_assignment_system`.
- Then execution: `mine_job_execution_system`, `hauling_execution_system`, `auto_haul_system`.

Risks
- Tight coupling via insert/remove and `Added<Item>` filters can create fragile order dependencies.

Proposed Improvements
- Document ordering invariants in code comments and module docs.
- Consider labels and explicit `before/after` constraints instead of relying on ordering within tuples.
- Separate planning vs. apply phases consistently across systems.

Acceptance Criteria
- Add system labels and ordering constraints; update docs.
- Add regression tests to ensure schedule ordering invariants.
