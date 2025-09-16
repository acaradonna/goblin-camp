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


## Repository Structure and Navigation

### Workspace Layout
- **Root**: `/` - Workspace Cargo.toml, dev.sh script
- **CLI interface**: `crates/gc_cli/` - Command-line demos and testing interface
- **Core engine**: `crates/gc_core/` - ECS simulation, pathfinding, jobs, save/load
- **Documentation**: `docs/` - Architecture, design notes, roadmap

<!-- Generated automatically on 2025-09-16 00:48:55 UTC -->
