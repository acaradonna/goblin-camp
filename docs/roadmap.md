# Roadmap
Updated: 2025-09-05

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

Next (M4–M6 priorities):
- UI/UX core: camera, selection, overlays (see #201)
- Autosave & crash recovery (see #202, #214, #217)
- Construction: designations → jobs, materials, placement (see #212–#216)
- Performance & observability: benches and metrics (see #207, #218)
- Tutorial & onboarding basics (see #208)

Monthly refresh checklist:
- [x] Update this roadmap “Updated” date and priorities
- [x] Review `docs/plan/MASTER_PLAN.md` epics and statuses
- [x] Cross-link active milestones and issues
- [x] Validate docs build on CI
