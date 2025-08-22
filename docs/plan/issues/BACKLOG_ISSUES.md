# Issue Backlog (to be created via gh)

Note: Run `./dev.sh check` before PRs. Use labels: area/sim, area/cli, kind/docs, kind/bench, epic/M1, epic/M2, epic/M3. Milestones: M1, M2, M3. All issues reference `docs/plan/MASTER_PLAN.md`.

## M1 Determinism + Lifecycle

- [ ] [M1] Design doc: Designation lifecycle (states, dedup, consumption)
  - Desc: Define DesignationState, state machine, dedup rules, and consumption semantics.
  - Deliverables: `docs/design/designations.md` with diagrams; acceptance criteria.

- [ ] [M1] Impl: DesignationState component + dedup system
  - Desc: Add component and a system that marks duplicates as Ignored.
  - Deliverables: code + tests; no behavior change to jobs yet.

- [ ] [M1] Impl: Consume designation on job creation
  - Desc: On job creation, mark designation Consumed and prevent re-jobbing.
  - Deliverables: code + tests.

- [ ] [M1] Tests: Designation lifecycle
  - Desc: Integration tests in `crates/gc_core/tests/` covering duplicate prevention and consumption.

- [ ] [M1] ADR: Time and determinism (fixed-step schedule, seed strategy)
  - Desc: Add `docs/architecture/adr/0001-time-determinism.md` describing tick length, ordering, RNG streams.

- [ ] [M1] Impl: Time resource + fixed-tick schedule + system ordering docstrings
  - Desc: Introduce `Time` resource, unify schedule run per tick, and annotate systems with ordering notes.

- [ ] [M1] Impl: Seed/Determinism resource; thread through mapgen/fov/path/jobs
  - Desc: Centralize RNG seed/state, remove ad-hoc seeding.

- [ ] [M1] Bench: Pathfinding A* on random maps (criterion)
  - Desc: Add `benches/path_aStar.rs` using `criterion`.

- [ ] [M1] Bench: FOV vis calc (criterion)
  - Desc: Add `benches/fov.rs` using `criterion`.

- [ ] [M1] Demo: Jobs output shows action log and assignment summary
  - Desc: Extend CLI jobs demo to print lifecycle events without changing behavior logic.

## M2 Job Execution MVP

- [ ] [M2] Design doc: Mining/Items/Stockpiles/Hauling minimal pipeline
  - Desc: Define events, components (Item, Inventory, Stockpile), and systems interactions.

- [ ] [M2] Impl: Mining system converts Wall->Floor and queues item spawn event
  - Desc: Executing mined jobs mutates tiles and emits ItemSpawn event.

- [ ] [M2] Impl: Item entity MVP (Stone), Position, Carriable marker
  - Desc: Basic item representation for hauling.

- [ ] [M2] Impl: Inventory component on agents; pick-up/put-down helpers
  - Desc: Carry a single item MVP.

- [ ] [M2] Impl: Stockpile MVP (zone entity, accepts any); nearest-stockpile query
  - Desc: Zone entity with bounds, simple membership.

- [ ] [M2] Impl: Hauling job execution to stockpile
  - Desc: Move item along path to a valid stockpile cell.

- [ ] [M2] Tests: Mining->Item->Haul end-to-end
  - Desc: Integration test with assertions on tiles and item counts.

- [ ] [M2] Demo: Jobs demo prints mined tiles + hauled counts
  - Desc: CLI messages for visibility.

## M3 UX Prototype

- [ ] [M3] Design doc: TUI shell (ratatui + crossterm), update loop, overlays
  - Desc: Propose crate layout and rendering plan.

- [ ] [M3] Impl: New crate gc_tui with basic map render and agent marker
  - Desc: Create crate and wire to core read-only world snapshot.

- [ ] [M3] Impl: Input: pause/step; toggle visibility overlay; resize-safe
  - Desc: Minimal input loop.

- [ ] [M3] Wiring: Shared core world state build for TUI; keep CLI as CI default
  - Desc: Extract world builder util to core so both CLI/TUI use it.

- [ ] [M3] Tests: Golden-frame snapshot of TUI rendering (ASCII capture)
  - Desc: Deterministic golden tests.

## Epics tracking (M4+)

- [ ] [Epic] Workshops/production chains — draft design doc
- [ ] [Epic] Zones/stockpile rules — draft design doc
- [ ] [Epic] Fluids 2D (cellular), basic temperature — draft design doc
- [ ] [Epic] Needs/moods/traits — draft design doc
- [ ] [Epic] Combat MVP (simple injuries/death) — draft design doc
- [ ] [Epic] Worldgen: overworld + embark — draft design doc
- [ ] [Epic] Save/Load v2 (RON/CBOR + migrations) — draft design doc
- [ ] [Epic] Z-levels, multi-layer fluids — draft design doc
- [ ] [Epic] Advanced AI/squads/sieges — draft design doc
- [ ] [Epic] Diplomacy/economy/events/justice — draft design doc
- [ ] [Epic] Modding/API/content packs — draft design doc
