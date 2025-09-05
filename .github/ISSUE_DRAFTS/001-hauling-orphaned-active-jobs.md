Title: Hauling system can orphan ActiveJobs when immediate-delivery path fails to find item

Summary
- In `hauling_execution_system`, the immediate-delivery path can clear a carrier's `AssignedJob` without removing the corresponding entry from `ActiveJobs` if the item is not found at the pickup location, leaving an orphaned job.

Details
- Location: `crates/gc_core/src/systems.rs` in `hauling_execution_system`.
- In the first pass, when a carrier has no item and is not at `from`, an update is planned with `dropping: true` and `target: to` (immediate delivery).
- In the second pass, the system attempts to find the item at `from` to add an `ItemUpdate` and push the job to `completed_jobs`.
- In the third pass, regardless of whether the item was found, if `dropping` is true the carrier's `inventory` is cleared and `assigned_job` is set to `None`.
- Finally, only jobs in `completed_jobs` are removed from `ActiveJobs`.

Impact
- Results in jobs that persist in `ActiveJobs` with no carrier assigned to them, leading to inconsistent state and potential logic leaks in longer simulations.

Steps to Reproduce
1) Create a haul job where the `from` location no longer contains the item (e.g., item removed or moved between planning and execution).
2) Ensure a carrier is assigned this job and is not at the `from` location.
3) Run a tick. The carrier will move to `to`, clear its assignment, but the job remains in `ActiveJobs`.

Expected Behavior
- Either:
  - The carrier retains the job and attempts again on subsequent ticks, or
  - The job is explicitly cancelled/removed from `ActiveJobs` when the item is not found, and the carrier's `AssignedJob` is cleared accordingly.

Actual Behavior
- The carrier's `AssignedJob` is cleared, but `ActiveJobs` still contains the job since it is only removed when added to `completed_jobs`.

Proposed Fix
- Only clear the carrier's `AssignedJob` when the job is marked complete (i.e., when added to `completed_jobs`).
- Alternatively, introduce explicit job states (e.g., Pending, InProgress, Failed) and handle the "item missing at from" case by cancelling the job or re-queuing it on `JobBoard`.
- Consider splitting immediate-delivery into a two-step state machine to avoid side effects when prerequisites are missing.

Acceptance Criteria
- Add tests covering the case where the item is missing at `from` during the immediate-delivery path.
- After a tick in that scenario, there is no orphan job in `ActiveJobs` and carrier/job state remains consistent.
- Existing passing tests for normal hauling and immediate-delivery continue to pass.

Notes
- See logic clusters around planning updates and third-pass application in `hauling_execution_system`.
