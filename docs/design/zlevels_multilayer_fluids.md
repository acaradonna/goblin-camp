# Z-levels and Multi-layer Fluids (Epic #39)

Status: Draft → Ready for implementation
Owner: @acaradonna
Related: fluids, save-load, pathfinding, jobs, worldgen

## Goals

- Introduce stacked Z-level maps with consistent coordinates and deterministic stepping
- Add multi-layer fluids (e.g., water over magma) with per-layer rules and interactions
- Maintain performance and determinism with bounded memory and stable iteration

## Non-goals (MVP)

- Infinite z; chunk streaming; 3D FOV; complex gas simulation

## Architecture Overview

- Coordinate system: (x, y, z) with z in [0..Z-1]; row-major per level
- World storage: layered grids per component; fluids per layer, per cell with small fixed slots
- Scheduling: extend sim stages to process fluids per level, top-down or bottom-up as required
- Determinism: level-order iteration fixed; RNG streams per subsystem and z-scope

## Data Structures

- ZGrid<T>: Vec<T> sized width*height*z with (z-major | level-major) layout; benchmarks decide
- FluidCell: up to N layers with { kind, amount, temperature, flags }
- Collision/solids grid shared across levels with occupancy rules

## Interactions

- Heat exchange between layers; phase changes per material
- Vertical flow between z levels via permeability; lateral flow as in 2D with capacity
- Settling rules to ensure stable, bounded iterations

## CLI & Tools

- CLI demos to render (x, y, z) slices and vertical columns
- Debug inspectors for fluid stacks per cell

## Tests

- Golden snapshots for stacked worlds; determinism across seeds and steps
- Property: conservation within tolerance; vertical/lateral flow rules

## Risks & Mitigations

- Memory growth: use compact per-cell representations; cap layers N
- Performance: frontier-based updates; only active cells processed
- Complexity: isolate layer logic; keep interactions table-driven

## Tracking

TBD after story issue creation.
