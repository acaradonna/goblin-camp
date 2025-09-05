Title: FOV compute is O(E * r^2 * log path) with naive LOS checks; optimize

Summary
- `compute_visibility_system` performs a naive per-entity radius square scan and calls `los_visible` per tile, which itself raycasts via Bresenham. This can be heavy for larger radii or entity counts.

Details
- Location: `crates/gc_core/src/fov.rs`.
- For each entity, loops `dy`/`dx` in a square radius and calls `los_visible` to perform a line trace.

Impact
- Performance costs scale poorly with number of entities and radius, affecting TUI framerate and headless sims.

Proposed Improvements
- Implement shadowcasting (e.g., recursive shadowcasting, Permissive FOV) to compute FOV in roughly O(r^2) without per-tile raycasts.
- Cache opacity map; avoid repeated `map.idx`/`get_tile` calls.
- Consider tile-visibility bitmap to speed overlay unions.

Acceptance Criteria
- Replace `los_visible` loop with a shadowcasting algorithm.
- Bench shows improved performance at higher radii (e.g., r=16..32) and entity counts.
