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

Next (M1–M3 targets):

- Designation lifecycle (prevent duplicates, consume on job creation)
- Job execution systems (mine, haul)
- Deterministic tick scheduler and seed handling
- Benchmarks for pathfinding and FOV
- TUI shell prototype
