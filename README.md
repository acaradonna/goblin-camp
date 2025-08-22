# Goblin Camp

This repository will host a long-term project to build a complex, deep simulation/colony management game inspired by Dwarf Fortress, reimagined around goblins. Scope is intentionally massive and will be approached iteratively.

Near-term goals:

- Establish architecture, tech stack, and project scaffolding.
- Implement core ECS loop, map generation, pathfinding, jobs, and UI shell.
- Build robust data-driven content system for entities, items, and worldgen.

This project will be developed in public with modular milestones that can be built and tested independently.

Workspace layout:

- `crates/gc_core`: Core simulation engine (ECS, world, jobs, pathfinding).
- `crates/gc_cli`: Headless CLI to run and smoke-test the simulation.
- `docs/`: Architecture, roadmap, and design notes.

## Quickstart

- Build: `cargo build`
- Run interactive demo menu: `cargo run -p gc_cli -- menu`
- Run specific demos:
  - Mapgen: `cargo run -p gc_cli -- mapgen`
  - FOV with visibility overlay: `cargo run -p gc_cli -- fov --show-vis`
  - Pathfinding: `cargo run -p gc_cli -- path`
  - Batched pathfinding with cache: `cargo run -p gc_cli -- path-batch`
  - Jobs & Designations: `cargo run -p gc_cli -- jobs`
  - Save/Load: `cargo run -p gc_cli -- save-load`

Tip: global flags like `--width/--height` must come before the subcommand, e.g. `cargo run -p gc_cli -- --width 40 --height 20 path-batch`.

See `docs/roadmap.md` for milestone progress.
