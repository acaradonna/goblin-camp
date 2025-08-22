# Pathfinding

We use the `pathfinding` crate for A* on a 4-connected grid. Walkable tiles are `Floor`.

## PathService

A caching layer wraps pathfinding requests to reduce repeated searches:

- LRU cache keyed by `(sx,sy,gx,gy)` using `lru` crate.
- API:
  - `get(&mut self, map, start, goal) -> Option<(Vec<(i32,i32)>, i32)>`
  - `batch(&mut self, map, &[PathRequest]) -> Vec<Option<...>>`
  - `stats() -> (hits, misses)` and `reset_stats()`
- Capacity is configurable; default demo uses 256.

This is meant as a building block for future pathfinding queues and agent planners. Determinism is preserved as cache lookups do not introduce nondeterministic behavior.

Grid topology:

- 4-way (N,E,S,W) for MVP; 8-way later with cost tweaks

Walkability:

- TileKind controls cost/blocked; dynamic blockers as components

Algorithms:

- A* (MVP)
- Flow fields for shared goal regions (stockpiles)
- Jump Point Search optional for speed on uniform grids
