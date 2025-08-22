# AI Jobs and Designations

We maintain a `JobRegistry` (map of `JobId -> Job`) and a `JobQueue` (pending jobs by id). Agents pull jobs that match their role.

## Designations -> Jobs

`MineDesignation` entities with a `Position` are converted into `JobKind::Mine` when `DesignationConfig.auto_jobs` is enabled. After creating a job, the designation entity is despawned. A simple in-tick dedupe prevents duplicate jobs from multiple identical designations.

## Assignment and Execution

- `miner_assignment_system`: assigns the next matching `Mine` job id to idle `Miner` agents.
- `mining_execution_system`: when a miner has a `Mine{x,y}` job, it instantly converts a `Wall` at `(x,y)` into `Floor` (MVP behavior), then clears the assignment and removes the job from the registry.

Future work: path-based approach to reach targets, tool requirements, multi-tile designations, and ownership/priorities.

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
