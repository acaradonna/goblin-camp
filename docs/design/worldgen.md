# World Generation

Targets:

- Overworld: heightmap + rainfall + temperature -> biomes
- Sites: embark map carved from world tiles

Approach:

- Noise-based terrain, rivers, lakes; erosion pass
- Place civs/roads; simulate histories for flavor (lightweight)
- Data-driven params for seeds and biome defs

## Epic breakdown and acceptance criteria

This epic (#37) is executed via the following sequenced issues. Each story is small, testable, and deterministic.

### Story 1: [Worldgen] 1/9 — Core scaffolding (params, fixed-point, grids)

Scope:

- Define WorldGenParams and deterministic RNG substreams (height, temp, rain, rivers, civs)
- Implement fixed-point helpers (S16.16) and row-major layer grids (height, temp, rain, biome, rivers, lakes)
- Dev-only ASCII layer render helpers

Deliverables:

- Types + constructors with bounds checks; docs

Acceptance:

- Unit tests for fixed-point ops and RNG substreams
- Deterministic empty grid initialization

### Story 2: [Worldgen] 2/9 — Heightmap generation + sea level

Scope:

- Domain-warped noise (fbm + optional ridged) → normalized height [0..1]
- Apply sea_level; classify ocean/shore/inland

Deliverables:

- Height grid, ocean mask

Acceptance:

- Golden ASCII snapshot for known seeds; determinism test

### Story 3: [Worldgen] 3/9 — Temperature and rainfall layers

Scope:

- Temperature: latitude gradient + altitude lapse + noise; i16 C*10
- Rainfall: winds + orographic + noise; i16 mm*10

Deliverables:

- Temp and rain grids

Acceptance:

- Golden ASCII snapshots and determinism tests

### Story 4: [Worldgen] 4/9 — Flow, accumulation, river class

Scope:

- 8-way steepest descent flow_dir; upstream accumulation
- Threshold to river mask; u8 class buckets

Deliverables:

- flow_dir grid, river class grid

Acceptance:

- ASCII rivers rendered with thickness; determinism test

### Story 5: [Worldgen] 5/9 — Depression filling and lakes

Scope:

- Simple pit filling or outlet carving; basin detect with spill

Deliverables:

- Lake id mask; updated river continuity

Acceptance:

- No dead-end rivers in basins; determinism test

### Story 6: [Worldgen] 6/9 — Lightweight erosion pass

Scope:

- Thermal erosion on steep slopes; optional hydraulic smoothing on rivers
- Frontier-bounded iterations

Deliverables:

- Smoother height with bounded CPU

Acceptance:

- Perf within budget for 256x128; determinism preserved

### Story 7: [Worldgen] 7/9 — Biome assignment + palette

Scope:

- Köppen-like table mapping (temp, rain, height) → BiomeId with overrides

Deliverables:

- Biome grid; ASCII legend/palette

Acceptance:

- Biome histogram sanity checks; determinism test

### Story 8: [Worldgen] 8/9 — Civ sites and roads (flavor)

Scope:

- Poisson-disc site placement; A* roads over cost grid

Deliverables:

- civ_sites, roads lists; ASCII overlay

Acceptance:

- Spacing constraints enforced; determinism test

### Story 9: [Worldgen] 9/9 — Embark extraction

Scope:

- EmbarkRect → local seed; sample layers into local meta; river/shore features

Deliverables:

- EmbarkMapMeta with summaries; ASCII cutout demo

Acceptance:

- Deterministic extraction for same rect; doc examples
