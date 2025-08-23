# Goblin Camp

This repository will host a long-term project to build a complex, deep simulation/colony management game inspired by Dwarf Fortress, reimagined around goblins. Scope is intentionally massive and will be approached iteratively.

Near-term goals:

- Establish architecture, tech stack, and project scaffolding.
- Implement core ECS loop, map generation, pathfinding, jobs, and UI shell.
- Build robust data-driven content system for entities, items, and worldgen.
- Implement designation lifecycle management to prevent duplicate job creation.

This project will be developed in public with modular milestones that can be built and tested independently.

Workspace layout:

- `crates/gc_core`: Core simulation engine (ECS, world, jobs, pathfinding).
- `crates/gc_cli`: Headless CLI to run and smoke-test the simulation.
- `docs/`: Architecture, roadmap, and design notes.

## Quickstart

### Using the dev script (recommended)
- Setup: `./dev.sh` (builds, tests, and verifies everything works)
- Run demos: `./dev.sh demo`
- Full check: `./dev.sh check` (format, lint, test)
- Help: `./dev.sh help`

### Manual commands
- Build: `cargo build`
- Run interactive demo menu: `cargo run -p gc_cli -- menu`
- Run specific demos:
  - Mapgen: `cargo run -p gc_cli -- mapgen`
  - FOV with visibility overlay: `cargo run -p gc_cli -- --show-vis fov`
  - Pathfinding: `cargo run -p gc_cli -- path`
  - Batched pathfinding with cache: `cargo run -p gc_cli -- path-batch`
  - Jobs & Designations: `cargo run -p gc_cli -- jobs`
  - Save/Load: `cargo run -p gc_cli -- save-load`

Tip: global flags like `--width/--height` must come before the subcommand, e.g. `cargo run -p gc_cli -- --width 40 --height 20 path-batch`.

See `docs/roadmap.md` for milestone progress.

## Development

The project follows standard Rust development practices with automated CI validation:

- **Code formatting**: `cargo fmt` or `./dev.sh format`
- **Linting**: `cargo clippy` or `./dev.sh lint`
- **Testing**: `cargo test` or `./dev.sh test`
- **Full validation**: `./dev.sh check` (recommended before commits)

### Continuous Integration

All pull requests are automatically validated through GitHub Actions CI:

- ✅ **Format Check**: Code must be properly formatted (`cargo fmt --check`)
- ✅ **Lint Check**: No clippy warnings allowed (`cargo clippy` with `-D warnings`)
- ✅ **Build Verification**: All code must compile successfully
- ✅ **Core Tests**: 35 tests across 7 test suites must pass
- ✅ **Demo Verification**: Key functionality demos must execute correctly

The CI process ensures consistent code quality and prevents regressions. All checks must pass before PRs can be merged.

The development script `./dev.sh` provides convenient shortcuts for common tasks. Run `./dev.sh help` for a full list of commands.

### GitHub Copilot Integration

The project includes MCP (Model Context Protocol) server configurations optimized for GitHub Copilot development:

- **Setup**: Run `./setup-mcp.sh` to configure MCP servers
- **Configurations**: Multiple options in `.github/mcp-*.json`
- **Benefits**: Enhanced file navigation, GitHub integration, research capabilities
- **Documentation**: See `.github/mcp-servers.md` for details
