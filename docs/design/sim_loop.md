# Simulation Loop

Goals:

- Deterministic fixed timestep (e.g., 10 Hz) for repeatable saves/replays.
- Stateless systems; data lives in components/resources.

Stages:

- Input -> Designations -> Job Planning -> AI -> Movement/Pathing -> World Effects -> Cleanup

Tick order is designed to be deterministic. Current default schedule:

1. Movement
2. Confine to map bounds
3. Designation -> Job mapping (if enabled)
4. Job assignment
5. Visibility recomputation

Notes:

- Visibility uses per-entity computation with Bresenham LOS within a radius.
- Pathfinding requests should be funneled through `PathService` for caching.
- Future: system ordering will move to explicit sets and stages.
