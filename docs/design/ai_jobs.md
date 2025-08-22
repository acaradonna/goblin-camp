# AI Jobs and Designations

We maintain a `JobBoard` resource with simple FIFO/LIFO scheduling in M0. Agents with the `Carrier` role can be assigned jobs by `job_assignment_system`.

## Designations -> Jobs

`MineDesignation` entities with a `Position` are converted into `JobKind::Mine` when `DesignationConfig.auto_jobs` is enabled. The system `designation_to_jobs_system` performs this mapping each tick.

### Designation Lifecycle System

The designation system implements lifecycle management to prevent duplicate jobs:

- **`DesignationState`** enum tracks the state of each designation:
  - `Active` - Ready to be processed into jobs (default)
  - `Ignored` - Duplicate designation that should be skipped
  - `Consumed` - Processed designation (for future use)

- **`DesignationLifecycle`** component wraps the state as a Bevy ECS component

- **`designation_dedup_system`** runs before job creation to mark duplicate designations at the same position as `Ignored`

- **System ordering** ensures deterministic execution using `.chain()` so deduplication always runs before job creation

This prevents resource conflicts and ensures only one job is created per position, regardless of how many designations exist at that location.

## Next steps

- Job executors for mining/hauling.
- Priority queues and ownership.
- Path pre-check before assignment (using `PathService`).
- Support for designation areas/selections.

## AI and Jobs

Principles:

- Central job board; agents pull suitable jobs
- Utility-based action selection per agent
- Jobs are declarative with preconditions/effects

MVP jobs:

- Mine tile
- Haul item from A to B
- Build structure

Pathing:

- Use A* on grid; prefer cached flow fields for many-to-one tasks
