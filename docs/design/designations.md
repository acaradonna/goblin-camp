# Designations: Lifecycle, Deduplication, and Consumption (M1)

This document defines how designations (e.g., mining) behave so gameplay matches Dwarf Fortress expectations: placing a designation marks work exactly once, avoids duplicates on the same tile, and is consumed when a job is created or completed.

## Goals

- Deterministic, data-driven designation lifecycle
- Prevent duplicate designations on the same cell
- Consume designations when jobs are created (or completed), avoiding re-adding jobs
- Keep ECS systems stateless; state lives in components/resources

## ECS Data Model

- Component: `MineDesignation` — marker indicating a mine designation on an entity with `Position`
- Component: `DesignationState` — state of a designation (see below)
- Component: `MineDesignation` -- marker indicating a mine designation on an entity with `Position`
- Component: `DesignationState` -- state of a designation (see below)
- Resource: `DesignationConfig { auto_jobs: bool }` -- when true, create jobs from designations
- Resource: `JobBoard` -- destination for job creation

## States

DesignationState:

- New — freshly placed by the player
- Queued -- accepted and visible to job planning
- Assigned — a job has been created/reserved for this
- Consumed — designation should no longer create more jobs
- Component: `MineDesignation` -- marker indicating a mine designation on an entity with `Position`
- Component: `DesignationState` -- state of a designation (see below)
- Resource: `DesignationConfig { auto_jobs: bool }` -- when true, create jobs from designations
- Resource: `JobBoard` -- destination for job creation

## States

DesignationState:

- New -- freshly placed by the player
- Queued -- accepted and visible to job planning
- Assigned -- a job has been created/reserved for this
- Consumed -- designation should no longer create more jobs
- Cancelled -- removed by the player or invalidated

Only one active designation per cell may be in {New,Queued,Assigned}. Consumed/Cancelled remain only for audit/debug until cleanup.

## State Machine

```text
New -> Queued -> Assigned -> Consumed
  \-> Cancelled

Rules:
- New transitions to Queued when dedup passes and the designation enters the planning set.
- Queued transitions to Assigned when a job is emitted for it.
- Assigned transitions to Consumed on job creation (M1) or completion (later variants).
- Any non-consumed state can be Cancelled by the player.
```

## Deduplication (single designation per tile)

- On tick, gather all `MineDesignation` with `Position`.
- If more than one designation targets the same `(x,y)`, mark all but one as `Cancelled`.
- If an existing designation at `(x,y)` is already in {Queued,Assigned,Consumed}, new arrivals are immediately `Cancelled`.

Implementation sketch:

- Maintain a reusable `HashSet<Position>` in the dedup system, clearing it each tick rather than recreating it.
  - This avoids unnecessary allocations and improves performance, especially for large maps.
- First pass: mark first-seen as ok (transition New->Queued), subsequent as Cancelled

## Consumption semantics

For M1 MVP we consume at job creation to guarantee “one job per designation”:

- When `designation_to_jobs_system` sees a `Queued` designation, it emits exactly one `JobKind::Mine { x, y }`
- Immediately transition the designation to `Consumed`
- Never re-emit jobs for `Consumed` or `Cancelled`

Later variants could consume on job completion to better support retries, but that requires tracking job-designation linkage and failure handling.

## Systems and Ordering

Tick order (excerpt), aligning with `docs/design/sim_loop.md`:

1. Deduplicate designations (assign states New->Queued or Cancelled)
2. Map designations to jobs (Queued -> Assigned -> Consumed if `auto_jobs`)
3. Job assignment and execution systems

Notes:

- Systems are pure; ordering ensures determinism
- Use explicit queries with `With<MineDesignation>` and filters on `DesignationState`

## Edge Cases

- Re-placing on a consumed tile: immediately `Cancelled`
- Removing designation before assignment: `Cancelled`, no job emitted
- Multiple designations same tick: dedup ensures only one survives
- Map changes invalidating the tile: transition to `Cancelled`

## Acceptance Criteria (for issue M1: Designation lifecycle)

- Documented states and transitions
- Dedup strategy defined (single designation per cell)
- Consumption semantics defined for M1 (consume on job creation)
- Deterministic system ordering described
- Test cases listed in “Edge Cases” to guide `crates/gc_core/tests/`

## Follow-ups

- Implement `DesignationState` component and dedup system
- Update `designation_to_jobs_system` to consume on job creation
- Add tests for lifecycle and invariants
- Consider moving consumption to job completion in a later milestone
