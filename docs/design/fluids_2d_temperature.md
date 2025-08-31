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
  - temp: i16 (Celsius scaled by 10 — "S10C"). Default from biome/worldgen, e.g., 150 = 15.0°C
  - flags: bitflags (Solid, Source, Frozen)
- Constants:
  - FULL: 255; MIN_FLOW=2; EPS=1
  - Temperature scale: S10C (Celsius*10). FREEZE_POINT=0 (0.0°C), BOIL_POINT=1000 (100.0°C)
  - Volume thresholds: EVAP_THRESH=10 (min volume for evaporation), FREEZE_VOLUME_MIN=4 (min volume to form ice)
- Separate Mask/Index:
  - solid_mask: Bitset for walls/impassable
  - sources: Vec<(pos, kind, rate)> for springs/magma vents

## Update loop (per tick)

1. Inject sources: add volume (saturating) at rate per tick, mark Source
1. Gravity/Spread pass (one sweep):

- Down: move up to capacity below (FULL - below) from cell; min unit MIN_FLOW
- Lateral: distribute remaining equally to left/right if below not possible
- Up: if overpressure (volume > FULL and neighbors lower), bleed small amount
- Use two-buffers (read old, write next) to keep determinism
- Process in fixed order y from bottom to top, x left to right

1. Viscosity/Bleed: small decay to smooth stair-steps (volume -= volume/32)
1. Temperature pass:

- Diffusion: temp_new = temp + k * sum(neigh - temp)
- Phase change hooks:

  - Water: if temp <= FREEZE_POINT and volume >= FREEZE_VOLUME_MIN -> flags.Frozen, restrict flow (acts like solid with seepage)
  - Water: if temp >= BOIL_POINT and volume >= EVAP_THRESH -> reduce volume by boil_rate (MVP: evaporate; optional steam events)
  - Lava: high temp source; cools over time; when temp < solidus -> becomes Solid rock, set solid_mask

1. Cleanup: clamp, swap buffers, maintain an active-cells frontier for next tick

## Determinism & bounded work

- Active frontier: track cells that changed last tick; only reevaluate their 4-neigh next tick
- Deterministic tie-breaking: fixed neighbor order [Down, Left, Right, Up]
- Volume math: saturating arithmetic with u16 intermediates for flow; no floats
- Temperature math: store i16 (S10C); use i32 accumulators and fixed-point k (e.g., Q8.8) for diffusion
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

## Z-levels extension (multi-layer fluids)

Goals

- Support multiple stacked layers (z-levels) with vertical flow and pressure
- Keep per-tick work bounded and deterministic across layers
- Preserve integer/fixed-point math and active-frontier update model

Non-goals (MVP)

- Full Navier–Stokes simulation; diagonal vertical flow; arbitrary 3D meshes

Data model

- Layers: Vec<Layer>, each Layer is a 2D grid of Cells matching map width/height
- Cell additions:
  - ceiling/floor permeability flags (allows vertical exchange)
  - vertical capacity modifiers (e.g., grates, shafts)
  - heat coupling coefficient for vertical conduction
- Global:
  - z_count: u16, max Z levels loaded
  - VerticalExchange rules: min head/pressure difference to trigger up/down moves

Fixed-point volume and temperature

- Fluids use fixed-point units (e.g., milli-liters per tile) shared across layers
- Temperature conducts vertically using the same discrete step used horizontally with a coupling factor

Update loop (per tick)

1. Horizontal pass (each layer)
   - Use existing 2D frontier and flow rules per layer with bounded iterations
2. Vertical exchange pass (between layers)
   - For each active cell, compute head/pressure deltas with z-1 and z+1
   - Move min(delta, available, capacity) upward or downward respecting permeability
   - Enqueue affected neighbors to the frontier of both layers for next tick
3. Temperature conduction
   - Horizontal then vertical conduction using fixed-point, clamped to bounds

Determinism and ordering

- Process layers in ascending z order consistently
- Within a layer, process cells in row-major order for tie-breaking
- When moving vertically and both up/down are possible, prefer down then up (or define a stable enum ordering)

Performance guardrails

- Active frontier per layer; skip empty layers
- Global per-tick budget split among layers proportional to their active counts
- Chunking: optional 3D chunks (x,y,z) to improve cache locality; chunk queues feed layer frontier

Save/Load considerations

- Persist z_count and per-cell permeability/capacity flags
- Serialize layers sequentially (z=0..z=n-1) to keep row-major determinism
- Version gate: adding z-levels increments schema; migration fills z=0 from 2D saves and zeros other layers

CLI demo

- Extend existing `fov`/`fluids` demo or add `fluids-3d` with `--layers N` and `--rain` options
- ASCII slices: render one z at a time with keys [ and ] to change layer; optional vertical column probe

Tests

- Invariants: total mass conserved across layers except for sources/sinks; no negative volumes
- Determinism: same seed → identical cross-layer distributions after N ticks
- Edge: sealed floors block downward flow; vents only allow up; top layer evaporation only

Implementation plan (incremental)

1. Data scaffolding: layers vector, cell flags, schema bump, migrations from 2D
2. Vertical exchange rules and bounded iteration loop
3. Integrate with active frontier and per-tick budget
4. Temperature vertical conduction
5. CLI demo and golden scenarios
6. Persistence tests with z>1
