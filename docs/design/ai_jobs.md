# AI Jobs and Designations

We maintain a `JobBoard` resource with simple FIFO/LIFO scheduling in M0. Agents with the `Carrier` role can be assigned jobs by `job_assignment_system`.

## Designations -> Jobs

`MineDesignation` entities with a `Position` are converted into `JobKind::Mine` when `DesignationConfig.auto_jobs` is enabled. The system `designation_to_jobs_system` performs this mapping each tick. Future work: remove the designation after job creation, avoid duplicate job creation, and support areas/selections.

## Next steps

- Job executors for mining/hauling.
- Avoid re-adding identical jobs.
- Priority queues and ownership.
- Path pre-check before assignment (using `PathService`).

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
