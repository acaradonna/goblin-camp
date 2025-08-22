# Architecture Overview

Core tenets:

- ECS via `bevy_ecs` for data-oriented separation of concerns.
- Deterministic simulation loop with explicit system order (see [ADR-0001](adr/0001-time-determinism.md)).
- Data-driven world definitions.

Subsystems (M0):

- Map (grid, tiles)
- FOV/LOS (Bresenham, per-entity visibility resource)
- Pathfinding (A*, PathService with LRU cache and batching)
- Jobs (JobBoard, designation->job mapping)
- Save/Load (JSON snapshot)
- CLI demo harness

Future: TUI front-end, content packs, worldgen layers, fluids, combat.
