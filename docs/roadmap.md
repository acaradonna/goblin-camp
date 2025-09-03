# Roadmap

Phases:

1. Foundations (M0–M3)
   - ECS core, schedules, resources
   - Grid map, FOV, A* pathfinding
   - Goblin agents, jobs: mine, haul, build
   - Save/load JSON prototype
   - CLI/TUI shell for sim control

2. Colony core (M4–M8)
   - Workshops, stockpiles, zones, designations
   - Fluids (2D cellular), temperature basics
   - Needs, moods, traits, professions
   - Combat MVP, injuries, death
   - Procedural worldgen (biomes, civs), caravans

3. Depth and polish (M9+)
   - Z-levels, multi-layer fluids
   - Advanced AI (squads, sieges), diplomacy
   - Economy, artifacts, events, justice
   - Modding/API, content packs, scenario editor

Release trains:

- Nightly: bleeding edge
- Alpha: monthly tagged builds
- Beta: feature complete, stabilization

M0 Progress (running checklist):

- [x] Core scaffolding (workspace, crates, docs)
- [x] Map generation MVP
- [x] Pathfinding A*
- [x] FOV/LOS check
- [x] Job board + assignment MVP
- [x] Save/Load JSON snapshot
- [x] CLI sim harness
- [x] Unit tests for map, LOS, path, save/load
- [x] Designations -> job creation (MVP)
- [x] Pathfinding batching/cache service + CLI demo
- [x] FOV per-entity visibility resource
- [x] ASCII map print in CLI
- [x] Documentation auto-deployment (GitHub Pages)
- [x] Designation lifecycle (prevent duplicates, consume on job creation)

## Current Status (M3 Active)

### ✅ Completed (M0-M2)
- [x] Core scaffolding (workspace, crates, docs)
- [x] Map generation MVP
- [x] Pathfinding A* with batching and caching
- [x] FOV/LOS check with per-entity visibility
- [x] Job board + assignment MVP
- [x] Save/Load JSON snapshot
- [x] CLI sim harness with ASCII rendering
- [x] Unit tests for map, LOS, path, save/load
- [x] Designations -> job creation (MVP)
- [x] Pathfinding batching/cache service + CLI demo
- [x] FOV per-entity visibility resource
- [x] ASCII map print in CLI
- [x] Documentation auto-deployment (GitHub Pages)
- [x] Designation lifecycle (prevent duplicates, consume on job creation)
- [x] Agent development environment and VS Code optimization
- [x] Save/Load v2 documentation and troubleshooting guide

### 🚧 In Progress (M3)
- Combat MVP core components (PR #175)
- Save/Load v2 implementation (Epic #38)
- Agent development guides and workspace optimization

### 📋 Next Priorities (M3)

#### P0 (Top Priority - Critical Path)
1. **Combat MVP Foundation** (#128 → #137)
   - Core combat components (Health, Faction, CombatStats, etc.)
   - Hostility policy and target selection
   - Move toward target using PathService
   - Attack resolution and cooldown system
   - Death handling and corpse markers
   - CLI combat demo (ASCII arena)
   - Wire combat systems into sim schedule

#### P1 (High Priority - M3 Scope)
2. **Fluids M3** (#115 → #124)
   - Core types and grid double-buffer
   - Sources registry + downward flow
   - Lateral spread + capacity rules + active frontier
   - Temperature grid + diffusion
   - Phase changes (freeze/boil; lava cools to rock)
   - Pathfinding integration (cost/blocked thresholds)
   - CLI demo (fluids) with ASCII renderer
   - Save/Load snapshot support
   - Tests (units + golden snapshots)
   - Docs polish and tuning

3. **Workshops M3** (#82 → #91)
   - Recipe registry core types
   - Station components and spawn helpers
   - WorkOrder entity and queueing
   - Haul to station job and input slots
   - Operate station job and outputs
   - MVP stations and recipes wired
   - CLI demo: workshops ASCII summary
   - Integration tests: deterministic outputs
   - Save/Load support for stations/orders
   - UntilHave(N) and output hauling to stockpiles

4. **Zones** (#93 → #113)
   - Core zone types, policies, and events
   - Zone indexer and occupancy counters
   - Acceptance API and deterministic selection
   - Reservation system with timeouts
   - Haul job generation (ground→stockpile)
   - Give/Take links and A->B feeder logic
   - CLI demo (zones overlay)
   - Save/Load schemas + migration
   - Rebalancing (bounded work)
   - Docs polish and guidance

5. **Worldgen Epic** (#138 → #146)
   - Core scaffolding (params, noise helpers, fixed-point, grids)
   - Heightmap generation + sea level classification
   - Temperature and rainfall layers
   - Flow direction, accumulation, and river classification
   - Depression filling and lake detection
   - Thermal erosion pass with frontier limiting
   - Biome assignment via table; palette and ASCII renderer
   - Civ placement (capitals + towns) and simple road routing
   - Embark extraction: sample layers into local map meta

#### P2 (Medium Priority - M4)
6. **Needs/Moods/Traits** (#160)
   - Deterministic motivation model
   - Components: Needs, NeedRates, Thoughts, Mood, Traits
   - Systems: ComputeEffectiveNeedRates, ApplyNeedRates, etc.
   - Integration: job scoring bias and hard blocks

7. **Pathfinding V2** (#161)
   - Flow fields, JPS, 8-way movement
   - Dynamic costs from fluids and terrain
   - Many-to-one goals optimization

## Agent-Friendly Priority Queue

### How to Work
1. **Pick topmost P1 story** in your area
2. **Ensure determinism and tests** per acceptance criteria
3. **Link your PR** to the issue (Fixes #N) and the parent epic where relevant
4. **One PR per issue** - include tests and CLI demo updates if applicable
5. **Keep changes deterministic** (no wall-clock time dependencies)
6. **Update docs** if behavior changes

### Current Work Areas
- **Combat MVP**: Focus on P0 issues first (#128, #129, #130, #131, #133, #137)
- **Fluids M3**: P1 issues (#115-#124) - foundational for temperature and phase changes
- **Workshops M3**: P1 issues (#82-#91) - production chains and automation
- **Zones**: P1 issues (#93-#113) - logistics and stockpile management
- **Worldgen**: P1 issues (#138-#146) - procedural world generation

### Milestones and Epics
- **M3 scope**: labeled "milestone:m3" - current active milestone
- **Epics**: labeled "epic" (see #33, #34, #36, #37, #38, #39, #40, #41, #42)
- **Z-levels Epic**: #39 - future M4+ feature for multi-layer worlds
