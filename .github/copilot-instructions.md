# Goblin Camp Development Instructions

Goblin Camp is a Rust-based colony management game simulation inspired by Dwarf Fortress. It uses Bevy ECS for the core simulation engine with a CLI interface for headless testing and development.

**ALWAYS reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.**

## Working Effectively

### Bootstrap and Build
- **CRITICAL**: Set timeouts to 60+ minutes for all build commands. NEVER CANCEL builds even if they appear to take time.
- Initial setup and build: `./dev.sh setup` -- takes ~40 seconds with dependencies, NEVER CANCEL. Set timeout to 60+ minutes.
- Build only: `./dev.sh build` or `cargo build` -- typically <5 seconds after initial setup, NEVER CANCEL.
- Release build: `cargo build --release` -- takes ~31 seconds, NEVER CANCEL. Set timeout to 60+ minutes.

### Testing and Validation
- Run tests: `./dev.sh test` or `cargo test` -- takes <1 second, set timeout to 10+ minutes for safety.
- Linting: `./dev.sh lint` or `cargo clippy` -- takes ~14 seconds, NEVER CANCEL. Set timeout to 30+ minutes.
- Code formatting: `./dev.sh format` or `cargo fmt` -- takes <1 second.
- **Full validation pipeline**: `./dev.sh check` -- runs format check, lint, and tests. Takes <1 second after initial setup, set timeout to 30+ minutes.

### Development Workflow
- **ALWAYS run `./dev.sh check` before making commits** - this ensures format, linting, and tests pass.
- Use the `./dev.sh` script for all common operations - it's well-designed and handles edge cases.
- The repository builds successfully and all demos work correctly.

## Running and Testing the Application

### Interactive Demo Menu
- **Primary interface**: `./dev.sh demo` or `cargo run -p gc_cli -- menu`
- Provides interactive selection of all available demos
- **VALIDATION**: Always run this after making changes to verify the application works

### Specific Demo Commands
**IMPORTANT**: Global flags must come BEFORE the subcommand, e.g., `cargo run -p gc_cli -- --width 40 --height 20 mapgen`

- **Map generation**: `cargo run -p gc_cli -- mapgen`
- **Field of view**: `cargo run -p gc_cli -- fov`
- **FOV with visibility overlay**: `cargo run -p gc_cli -- --show-vis fov`
- **Pathfinding**: `cargo run -p gc_cli -- path`
- **Job assignment**: `cargo run -p gc_cli -- jobs`
- **Save/Load**: `cargo run -p gc_cli -- save-load`
- **Path batching**: `cargo run -p gc_cli -- path-batch`

### Global Flags (place before subcommand)
- `--width <WIDTH>` - Map width (default: 80)
- `--height <HEIGHT>` - Map height (default: 50)
- `--steps <STEPS>` - Simulation steps (default: 10)
- `--seed <SEED>` - RNG seed for reproducible maps (default: 42)
- `--ascii-map` - Print ASCII map (default: true)
- `--show-vis` - Show visibility overlay in FOV demo

### Example Commands
- `cargo run -p gc_cli -- --width 40 --height 20 mapgen`
- `cargo run -p gc_cli -- --show-vis fov`
- `cargo run -p gc_cli -- --seed 123 --width 60 path`

## Manual Validation Scenarios

**CRITICAL**: After making changes, ALWAYS run through these validation scenarios to ensure functionality works correctly:

### 1. Core Simulation Validation
```bash
# Test map generation and visualization
cargo run -p gc_cli -- mapgen
# Verify: ASCII map displays with ~ (water), . (floor), # (spawn point)

# Test custom map sizes
cargo run -p gc_cli -- --width 40 --height 20 mapgen
# Verify: Map is 40x20 and displays correctly
```

### 2. Pathfinding Validation
```bash
# Test basic pathfinding with visualization
cargo run -p gc_cli -- path
# Verify: "Path found" message with length/cost, ASCII map shows path with 'o' characters

# Test pathfinding with custom map
cargo run -p gc_cli -- --width 60 --height 30 path
# Verify: Path is found and visualized on custom-sized map
```

### 3. ECS and Job System Validation
```bash
# Test job assignment system
cargo run -p gc_cli -- jobs
# Verify: "Grak assigned: [UUID]" message appears (shows job assignment working)
```

### 4. Persistence Validation
```bash
# Test save/load functionality
cargo run -p gc_cli -- save-load
# Verify: "Serialized save length: X bytes" and "Reloaded world with AxB map" messages
```

### 5. Performance and Caching Validation
```bash
# Test pathfinding cache performance
cargo run -p gc_cli -- path-batch
# Verify: "Batched X requests. Cache hits=Y, misses=Z" where hits > 0 indicates caching works
```

### 6. Field of View Validation
```bash
# Test FOV calculation
cargo run -p gc_cli -- fov
# Verify: "LOS from (1,1) to bottom-right-1: true" message

# Test FOV with visibility overlay
cargo run -p gc_cli -- --show-vis fov
# Verify: Map displays with '*' characters showing visible area from entity perspective
```

## Repository Structure and Navigation

### Workspace Layout
- **Root**: `/` - Workspace Cargo.toml, dev.sh script
- **Core engine**: `crates/gc_core/` - ECS simulation, pathfinding, jobs, save/load
- **CLI interface**: `crates/gc_cli/` - Command-line demos and testing interface
- **Documentation**: `docs/` - Architecture, design notes, roadmap
- **Tests**: `crates/gc_core/tests/` - Integration tests for core functionality

### Key Files
- `dev.sh` - Primary development script (setup, test, lint, format, check, demo)
- `Cargo.toml` - Workspace configuration
- `crates/gc_cli/src/main.rs` - CLI application entry point and demo implementations
- `crates/gc_core/src/lib.rs` - Core simulation engine
- `docs/roadmap.md` - Project milestones and progress tracking

### Dependencies and Tech Stack
- **Rust Edition**: 2021
- **ECS Framework**: Bevy ECS 0.14
- **Serialization**: serde + serde_json
- **CLI**: clap 4.5 with derive features
- **Pathfinding**: pathfinding crate with A* implementation
- **Map Generation**: noise crate for procedural generation
- **Caching**: LRU cache for pathfinding optimization

## Development Standards

### Code Quality
- **Format**: `cargo fmt` - always run before commits
- **Linting**: `cargo clippy` - fixes code quality issues
- **Testing**: All tests must pass - currently 5 integration tests covering core functionality
- **Full check**: `./dev.sh check` - runs format check + lint + tests

### Testing Guidelines
- Tests are located in `crates/gc_core/tests/`
- Integration tests cover: pathfinding, save/load, field of view, path caching
- Run `cargo test` to execute all tests (takes <1 second)
- Tests validate core ECS systems, serialization, and algorithm correctness

### Common Tasks and Expected Outputs

#### Repository Root Contents
```
ls -la
.git/
.github/
.gitignore
Cargo.lock
Cargo.toml
README.md
crates/
dev.sh*
docs/
```

#### Workspace Structure
```
find crates -name "*.toml"
crates/gc_cli/Cargo.toml
crates/gc_core/Cargo.toml
```

#### Demo Help Output
```
cargo run -p gc_cli -- --help
Commands:
  menu        Interactive menu
  mapgen      Show generated map and basic info
  fov         Line-of-sight/FOV demo
  path        A* pathfinding demo
  jobs        Job board + designation assignment demo
  save-load   Save/Load snapshot demo
  path-batch  Batched pathfinding with LRU cache
```

## Critical Timing and Timeout Information

- **Initial dependency download and build**: 40 seconds typical, set 60+ minute timeout
- **Release builds**: 31 seconds typical, set 60+ minute timeout  
- **Incremental builds**: <5 seconds typical, set 60+ minute timeout for safety
- **Linting**: 14 seconds typical, set 30+ minute timeout
- **Tests**: <1 second typical, set 10+ minute timeout for safety
- **Full validation**: <1 second after setup, set 30+ minute timeout

**NEVER CANCEL**: Always wait for build and test commands to complete. Build times may vary based on system performance and cache state.

## Troubleshooting

### Build Issues
- If build fails due to missing dependencies, ensure Rust is installed: `rustc --version`
- Clean build: `cargo clean && cargo build`
- Check workspace consistency: ensure all Cargo.toml files are valid

### Demo Issues
- If demos don't display correctly, verify terminal supports ASCII characters
- For path visualization issues, ensure maps have valid paths (not all water)
- Job assignment UUIDs are randomly generated and will vary between runs

### Development Environment
- Repository works out-of-the-box with standard Rust installation
- No external system dependencies required beyond Rust toolchain
- Builds and runs successfully on standard Linux development environments