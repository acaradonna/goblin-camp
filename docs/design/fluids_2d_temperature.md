# Fluids 2D (cellular) and Basic Temperature — Draft Design (Epic #34)

Status: Draft
Owner: @acaradonna
Related: zones/stockpiles, pathfinding, jobs, save/load v2, worldgen

## Goals

- 2D cellular fluids for water/lava with simple, stable update rules
- Deterministic simulation (fixed tick, seeded RNG only for worldgen)
- Performance: O(cells touched) per tick, bounded work, cache-friendly
- Basic temperature field that diffuses/conducts and affects fluids (evap/freeze)
- CLI demos for visualization and validation

Non-goals (MVP): pressure waves, incompressible Navier-Stokes, z-levels

## Data model

- Grid: same width/height as map; per-cell struct:
  - fluid_kind: None | Water | Lava
  - fluid: u8 (0..=255) representing volume (0 empty, 255 full)
  - temp: i16 (scaled Celsius*10). Default from biome/worldgen, e.g., 150 = 15.0°C
  - flags: bitflags (Solid, Source, Frozen)
- Constants:
  - FULL: 255; MIN_FLOW=2; EPS=1; EVAP_THRESH=10; FREEZE_POINT=0°C, BOIL_POINT=100°C
- Separate Mask/Index:
  - solid_mask: Bitset for walls/impassable
  - sources: Vec<(pos, kind, rate)> for springs/magma vents

## Update loop (per tick)

1) Inject sources: add volume (saturating) at rate per tick, mark Source
2) Gravity/Spread pass (one sweep):
   - Down: move up to capacity below (FULL - below) from cell; min unit MIN_FLOW
   - Lateral: distribute remaining equally to left/right if below not possible
   - Up: if overpressure (volume > FULL and neighbors lower), bleed small amount
   - Use two-buffers (read old, write next) to keep determinism
   - Process in fixed order y from bottom to top, x left to right
3) Viscosity/Bleed: small decay to smooth stair-steps (volume -= volume/32)
4) Temperature pass:
   - Diffusion: temp_new = temp + k * sum(neigh - temp)
   - Phase change hooks:
     - Water: if temp <= 0°C and volume > EVAP_THRESH -> flags.Frozen, restrict flow (acts like solid with seepage)
     - Water: if temp >= 100°C -> reduce volume by boil_rate, maybe spawn Steam events (MVP: just evaporate)
     - Lava: high temp source; cools over time; when temp < solidus -> becomes Solid rock, set solid_mask
5) Cleanup: clamp, swap buffers, maintain an active-cells frontier for next tick

## Determinism & bounded work

- Active frontier: track cells that changed last tick; only reevaluate their 4-neigh next tick
- Deterministic tie-breaking: fixed neighbor order [Down, Left, Right, Up]
- Saturating arithmetic with u16 temp accumulators; no float during flow; temperature can use i32 fixed-point
- All randomization (e.g., source jitter) keyed by map seed and cell coords

## Interactions

- Pathfinding: cells with fluid > threshold considered high-cost/blocked depending on kind
- Jobs: hauling may be blocked by fluids; future work: buckets to drain
- Zones: designations like “no liquid” zone could be honored by path placement later

## Save/Load

- Versioned struct stored sparsely: run-length encode rows of empty to compress
- World snapshot includes fluid grid, temp grid, solid mask diffs, and sources
- Migration plan to v2 schema once save/load epic lands

## CLI Demos

- mapgen + fluids: seed map, place few water/lava sources; print ASCII frames for N steps
- fov overlay with fluids ignored vs. blocking to ensure logic parity
- knobs: --width/--height/--steps/--seed

### ASCII legend

- '.' floor dry, '~' shallow water (1..63), '=' medium (64..191), '≈' deep (192..255)
- '^' lava (scaled similar), '*' steam event marker (optional), '#' solid/rock, '█' frozen ice
- Optional: colorize later in TUI; CLI stays ASCII-only

## Implementation slices (stories)

1) Core types: FluidKind, FluidCell, FluidGrid with double-buffer, bitflags
2) Sources registry + injection; simple updater that only flows downward
3) Lateral flow and capacity rules; active frontier
4) Temperature grid + diffusion; per-kind base temps
5) Phase changes: water freeze/boil; lava cools -> rock (solid_mask update)
6) Pathfinding integration: cost/blocked based on thresholds per kind
7) CLI demo: new gc_cli subcommand `fluids` with seed/steps; ASCII renderer
8) Save/load snapshot for fluids (behind feature until v2 lands)
9) Tests: unit tests for flow rules and diffusion; golden ASCII snapshots for demo
10) Docs polish, tuning constants, and performance notes

## Risks & mitigations

- Performance spikes: use frontier and cap per-tick processed cell count; carryover backlog
- Numerical drift: integers and clamping; fixed update order
- ECS contention: isolate resources, schedule into its own stage; avoid conflicting queries
- Visual clarity: keep ASCII buckets and deterministic renderer

## Appendix: constants and pseudo-code

- move_amount = min(src - keep, capacity)
- capacity(cell) = FULL - cell.volume
- keep floor so tiny droplets disappear: keep = min(src, 1)
- Pseudocode outlines provided in comments in implementation PR
