# Goblin Camp Master Plan

Purpose: deliver a playable, headless-first colony sim that can evolve toward a Dwarf Fortress–class game. Plan is incremental, deterministic, and test-first, aligned with existing CLI demos and dev.sh workflow.

Guiding principles

- Deterministic ECS: explicit system order, fixed-step tick, seeded RNG.
- Small, composable features: “one concern per change”.
- Tests before demos: unit/integration/bench per feature.
- Data-oriented design: simple components, pure systems.
- Performance awareness from day one: profiling and benches on hot paths.

Quality gates (must hold before merge)

- ./dev.sh check passes (fmt, clippy, tests)
- Demos run via ./dev.sh demo and validate scenarios in copilot-instructions
- Added/updated docs (design + ADR if behavior changes)

Phase overview (maps to epics)

- M0 Foundations (Done): ECS, map, FOV, A*, jobs MVP, save/load, CLI
- M1 Determinism + Lifecycle: designation lifecycle, fixed tick, seed/resource handling, benches
- M2 Job Execution MVP: mining + items + stockpiles + hauling pipeline, demo + tests
- M3 UX Prototype: TUI shell prototype, input loop, overlays; CLI remains canonical for CI
- M4–M8 Colony Core: workshops, zones, fluids 2D, needs/moods, basic combat, worldgen
- M9+ Depth/Polish: z-levels, advanced AI, diplomacy, economy, modding

Architecture pillars

- ECS layers: core (math/map/algorithms), sim (systems/resources), IO (CLI/TUI), content (data)
- Services: PathService (LRU), SaveService (versioned), TimeService (fixed-step), EventBus (map/item events)
- Determinism: no wall-clock; PRNG streams per subsystem

Acceptance criteria examples

- Deterministic tick: same seed + same inputs => identical save hashes after N steps
- Mining job: wall tile becomes floor; stone item spawned; action logged; tests assert outcomes
- Haul job: item moves to nearest stockpile; path used; cache shows hits > 0 in batch demo

Risks and mitigations

- Scope creep: lock sprints to issues below; changes go to next sprint
- Non-determinism: forbid std::time in sim; audit RNG use; add determinism test
- Performance: bench hot paths (A*, FOV) on each PR; cap allocations; reuse buffers

Backlog (ordered, smallest viable increments)

M1 Determinism + Lifecycle

1. Design doc: Designation lifecycle (states, dedup, consumption)
2. Impl: DesignationState component + dedup system
3. Impl: Consume designation on job creation
4. Tests: Designation lifecycle
5. ADR: Time and determinism (fixed-step schedule, seed strategy)
6. Impl: Time resource + fixed-tick schedule + system ordering docstrings
7. Impl: Seed/Determinism resource; thread through mapgen/fov/path/jobs
8. Bench: Pathfinding A* on random maps (criterion)
9. Bench: FOV vis calc (criterion)
10. Demo: Jobs output shows action log and assignment summary

M2 Job Execution MVP

11. Design doc: Mining/Items/Stockpiles/Hauling minimal pipeline
12. Impl: Mining system converts Wall->Floor and queues item spawn event
13. Impl: Item entity MVP (Stone), Position, Carriable marker
14. Impl: Inventory component on agents; pick-up/put-down helpers
15. Impl: Stockpile MVP (zone entity, accepts any); nearest-stockpile query
16. Impl: Hauling job execution to stockpile
17. Tests: Mining->Item->Haul end-to-end
18. Demo: Jobs demo prints mined tiles + hauled counts

M3 UX Prototype

19. Design doc: TUI shell (ratatui + crossterm), update loop, overlays
20. Impl: New crate gc_tui with basic map render and agent marker
21. Impl: Input: pause/step; toggle visibility overlay; resize-safe
22. Wiring: Shared core world state build for TUI; keep CLI as CI default
23. Tests: Golden-frame snapshot of TUI rendering (ASCII capture)

Tracking epics (M4+)

E1. Workshops/production chains
E2. Zones/stockpile rules
E3. Fluids 2D (cellular), basic temperature
E4. Needs/moods/traits
E5. Combat MVP (simple injuries/death)
E6. Worldgen: overworld + embark
E7. Save/Load v2 (RON/CBOR + migrations)
E8. Z-levels, multi-layer fluids
E9. Advanced AI/squads/sieges
E10. Diplomacy/economy/events/justice
E11. Modding/API/content packs

How to work this plan

- Create issues per item above; prefix titles with [M1]/[M2]/[M3] or [Epic]
- Keep PRs to a single issue; include demo and tests
- Always validate via ./dev.sh demo and manual scenarios
