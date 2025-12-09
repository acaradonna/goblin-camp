# 🏰 Goblin Camp

> *A complex, deep simulation/colony management game inspired by Dwarf Fortress, reimagined around goblins.*

[![CI Status](https://github.com/acaradonna/goblin-camp/workflows/CI/badge.svg)](https://github.com/acaradonna/goblin-camp/actions)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://acaradonna.github.io/goblin-camp/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](#license)

**Goblin Camp** is an ambitious, long-term project to build a colony simulation game with the depth and complexity of Dwarf Fortress. The game centers around managing a goblin colony, with rich simulation systems for mining, crafting, combat, and survival.

## 🎯 Core Features

### ✅ **Currently Implemented**

- **🗺️ World Generation**: Procedural terrain with noise-based height maps
- **🔍 Pathfinding**: A* algorithm with LRU caching for optimal performance
- **👁️ Field of View**: Line-of-sight calculations for visibility systems
- **💼 Job System**: Hierarchical task assignment (mining, hauling, building)
- **⛏️ Mining Operations**: Convert wall tiles to floors, spawn stone items
- **📦 Item Management**: Full ECS entities with spatial simulation
- **🏪 Stockpiles**: Zone-based storage areas with automatic hauling
- **💾 Save/Load**: JSON serialization with versioning support
- **🎮 CLI Interface**: Interactive demo system with ASCII visualization

### 🚧 **In Development**

- **🏭 Workshops**: Production chains and crafting systems
- **🌡️ Temperature**: Heat simulation and environmental effects
- **💪 Needs & Moods**: Goblin psychology and satisfaction systems
- **⚔️ Combat**: Basic combat mechanics and injury systems

### 📋 **Planned Features**

- **🌍 World Simulation**: Overworld generation with biomes and civilizations
- **🔥 Fluids**: 2D cellular automata for water and lava flow
- **🏗️ Z-Levels**: Multi-layer underground complexes
- **🤖 Advanced AI**: Squad tactics, sieges, and diplomatic systems
- **🎨 Modding Support**: Data-driven content and scripting API

## 🎬 Demo Gallery

### 🗺️ Map Generation

```text
####################
##    ............##
##  .............##
## ...........   ##
## .........     ##
#           .....##
#   .......     .##
# .......   ...  ##
# .....  .... ....#
# ...   ....  ....#
####################
```

*Procedural terrain with noise-based generation*

### ⛏️ Mining & Hauling Pipeline

```text
Step 1: Wall Mining        Step 2: Item Creation       Step 3: Hauling to Stockpile
┌─────────────────┐       ┌─────────────────┐        ┌─────────────────┐
│ # # # # # # # # │       │ # # # # # # # # │        │ # # # # # # # # │
│ # M . . . . . # │  →    │ # G . . . . . # │   →    │ # . . . . . . # │
│ # . . . . . . # │       │ # . s . . . . # │        │ # . . . . . . # │
│ # . . . [S] . # │       │ # . . . [S] . # │        │ # . . . [s] . # │
└─────────────────┘       └─────────────────┘        └─────────────────┘

M = Miner    G = Goblin    s = Stone    S = Stockpile    # = Wall    . = Floor
```

*Complete mining-to-stockpile pipeline with automatic job assignment*

### 🎯 Pathfinding Demo

```text
Start: (2,2) → Goal: (8,6)
####################
##S...............##
## ***.............##
##    ****     ....##
##       ****......##
##          ****...##
##             **G.##
####################

*** = Optimal path found by A* algorithm
S = Start, G = Goal
```

*Cached pathfinding with performance optimization*

## 🚀 Quick Start

### 🔧 Prerequisites

- **Rust 1.81+** - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For cloning the repository

### 📥 Installation

```bash
git clone https://github.com/acaradonna/goblin-camp.git
cd goblin-camp
./dev.sh setup  # One-time setup: download deps and initial build

# Optional quick validation anytime
./dev.sh fast
```

### 🎮 Running Demos

```bash
# Interactive demo menu
./dev.sh demo

# Or run specific demos directly:
cargo run -p gc_cli -- menu          # Interactive menu
cargo run -p gc_cli -- mapgen        # Map generation demo
cargo run -p gc_cli -- fov           # Field of view demo
cargo run -p gc_cli -- path          # Pathfinding demo
cargo run -p gc_cli -- jobs          # Job system demo
cargo run -p gc_cli -- save-load     # Save/load demo
cargo run -p gc_cli -- tui           # TUI prototype (interactive)
```

### 🎛️ Command Options

```bash
# Custom map sizes
cargo run -p gc_cli -- --width 40 --height 20 mapgen

# Show visibility overlays
cargo run -p gc_cli -- --show-vis fov

# Multiple simulation steps
cargo run -p gc_cli -- --steps 50 jobs
```

> 💡 **Tip**: Global flags like `--width/--height` must come before the subcommand.

#### TUI Controls

- q or Esc: Quit
- Space: Pause/resume
- .: Single-step
- v: Toggle visibility overlay
- 1..9: Steps per frame

## 🏗️ Architecture

### 📊 **Entity-Component-System (ECS)**

Built on [Bevy ECS](https://github.com/bevyengine/bevy) for high-performance, data-oriented design:

- **🎯 Entities**: Goblins, items, designations, world features
- **📦 Components**: Pure data (Position, Inventory, Job assignments)
- **⚙️ Systems**: Logic operating on components (movement, mining, hauling)
- **🗃️ Resources**: Global state (GameMap, JobBoard, Time)

### 🔄 **Deterministic Simulation**

- **⏱️ Fixed-Step Timing**: Frame-rate independent simulation
- **🎲 Seeded RNG**: Reproducible random number generation
- **📋 System Ordering**: Explicit dependencies and execution order
- **🔍 Testing**: Comprehensive integration tests for all systems

### 📁 **Project Structure**

```text
goblin-camp/
├── crates/
│   ├── gc_core/          # 🧠 Core simulation engine
│   │   ├── src/
│   │   │   ├── components.rs    # ECS components
│   │   │   ├── systems.rs       # Core simulation systems
│   │   │   ├── jobs.rs          # Job assignment & execution
│   │   │   ├── world.rs         # Spatial representation
│   │   │   ├── path.rs          # A* pathfinding
│   │   │   ├── fov.rs           # Field of view
│   │   │   └── ...
│   │   └── tests/        # Integration tests
│   └── gc_cli/           # 🖥️ CLI interface & demos
│   └── gc_tui/           # 🧪 TUI prototype (interactive terminal UI)
├── docs/                 # 📚 Design documentation
│   ├── architecture/     # System architecture
│   ├── design/          # Feature design docs
│   └── plan/            # Development roadmap
└── target/              # Build artifacts
```

## 🧪 Development

### Using the dev script (recommended)

- **Setup**: `./dev.sh setup` (builds, tests, and verifies everything works)
- **Quick Check**: `./dev.sh fast` (<30s format, check, unit tests)
- **Agent Check**: `./dev.sh agent` (~1m clippy + integration tests)
- **Full Check**: `./dev.sh full` (~5m CI simulation with demos)
- **Sync**: `./dev.sh sync` (fetch, rebase, update deps)
- **Pre-Push**: `./dev.sh pre-push` (branch/commit validation + agent checks)
- **Git Hook (optional)**: `ln -sf ../../scripts/hooks/pre-push .git/hooks/pre-push` to mirror CI before every push.
- **Git Hooks**: `git config core.hooksPath scripts/hooks` to enable the pre-push hook (runs pre-push + markdownlint + shellcheck when tools are installed)

### Manual commands

- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run -p gc_cli`
- Docs: `cargo doc --open`

### Shared Bootstrap

Both CLI and TUI use `gc_core::bootstrap` to construct a canonical world and default schedule. This avoids drift between shells and preserves determinism (seeded RNG, fixed tick).

## 🔧 CI/CD Pipeline

Goblin Camp uses a tiered GitHub Actions pipeline for fast feedback and high quality.


### 1. Fast Feedback (On Pull Requests)
Runs on every PR commit. Optimized for speed (<2m).
- ✅ **PR Validation**: Commit message & branch naming
- 🎨 **Linting**: Rustfmt & Clippy
- 🧪 **Tests**: Unit & Integration tests (Debug build)
- 🎮 **Demos**: Essential demo validation (Headless)


### 2. Production Quality (On Merge to Main)
Runs on `main` branch. Comprehensive validation.
- 🔨 **Release Build**: Optimized binary verification
- 📊 **Coverage**: Code coverage reports (≥75% required)
- 📚 **Docs**: Documentation generation & check
- 🔒 **Security**: Audit & License checks


### 3. Local Validation
You can simulate these pipelines locally:

```bash
./dev.sh agent   # Run PR-level checks
./dev.sh full    # Run Main-level checks (CI simulation)
```


### 🔍 Security Scanning (`security-scan.yml`)

**Security Validations:**

- 🔍 **Cargo Audit**: Automated vulnerability scanning
- 🚫 **Cargo Deny**: Dependency license and security checks
- 🔬 **CodeQL**: Static code security analysis
- 📊 **Security Reports**: Automated issue creation for vulnerabilities

**Schedule:** Daily at 3 AM UTC + on-demand

### ⚡ Performance Benchmarking (`performance-benchmark.yml`)

**Performance Tracking:**

- 📊 **Pathfinding Benchmarks**: A* algorithm performance
- 🗺️ **Map Generation**: Procedural generation speed
- 🔄 **ECS Performance**: Entity-component-system efficiency
- 📈 **Regression Detection**: Automated performance alerts

**Features:**

- Historical performance comparison
- Automated issue creation for regressions
- Detailed benchmark artifacts and reports

**Schedule:** Weekly on Sundays at 2 AM UTC + on-demand

### 📦 Dependency Analysis (`dependency-analysis.yml`)

**Dependency Management:**

- 📅 **Outdated Check**: Identify outdated dependencies
- 📋 **License Analysis**: Review dependency licenses
- 📏 **Size Analysis**: Monitor dependency bloat
- 🔒 **Security Audit**: Check for vulnerable dependencies

**Schedule:** Weekly on Mondays at 4 AM UTC + on-demand

### 🚀 Release Management (`release-management.yml`)

**Automated Releases:**

- 📋 **Version Management**: Automatic version bumping
- 📝 **Changelog Generation**: Automated release notes
- 🔨 **Cross-Platform Builds**: Linux, macOS, Windows, ARM64
- 📦 **GitHub Releases**: Automated release publishing
- 📚 **Documentation Updates**: CHANGELOG.md maintenance

**Release Types:**

- **Patch**: Bug fixes (0.0.X)
- **Minor**: New features (0.X.0)
- **Major**: Breaking changes (X.0.0)
- **Pre-release**: Alpha/Beta releases

### 📚 Documentation Deployment (`deploy-docs.yml`)

**Documentation Pipeline:**

- 📖 **GitHub Pages**: Automated documentation deployment
- 🔄 **Jekyll Integration**: Static site generation
- 📝 **Architecture Docs**: System design documentation
- 🗺️ **Roadmap**: Development planning documents

### 🔧 MCP Server Validation (`validate-mcp.yml`)

**MCP Configuration:**

- ✅ **Server Validation**: MCP server configuration checks
- 🔗 **Integration Testing**: MCP server connectivity
- 📊 **Performance Monitoring**: MCP server response times

### 🤖 Copilot Instructions (`update-copilot-instructions.yml`)

**Automated Updates:**

- 📝 **Instruction Refresh**: Daily copilot instruction updates
- 🔄 **Documentation Sync**: Keep instructions current
- 🤖 **AI Enhancement**: Improve development assistance

### 📊 Pipeline Status & Monitoring

**Quality Gates:**

- 🏆 **Quality Gate**: Automated pass/fail assessment
- 🚨 **Issue Creation**: Automated alerts for failures
- 📈 **Metrics Tracking**: Performance and quality metrics
- 📋 **Summary Reports**: Comprehensive pipeline summaries

**Branch Protection:**

- Required status checks for all quality gates
- Coverage threshold enforcement
- Security scan requirements
- Performance regression prevention

### 🎛️ Manual Pipeline Control

**Workflow Dispatch Options:**

```yaml
# Example: Run only security checks
pipeline_type: security-only
skip_performance: true

# Example: Full pipeline without security
pipeline_type: full
skip_security: true
```

**Available Controls:**

- `pipeline_type`: Choose which pipeline stages to run
- `skip_performance`: Bypass performance benchmarking
- `skip_security`: Skip security scanning
- `release_type`: Choose version bump type
- `prerelease`: Create pre-release versions

### 📈 Pipeline Artifacts

**Generated Artifacts:**

- 📊 **Test Reports**: Detailed test execution results
- 📈 **Coverage Reports**: Code coverage analysis
- 🔒 **Security Reports**: Vulnerability assessments
- ⚡ **Performance Data**: Benchmark results and comparisons
- 📦 **Release Binaries**: Cross-platform executables
- 📋 **Analysis Reports**: Dependency and license analysis

### 🚨 Automated Alerts & Issues

**Smart Notifications:**

- 🚨 **Security Issues**: Automatic vulnerability alerts
- 📈 **Performance Regressions**: Benchmark comparison alerts
- 📦 **Dependency Updates**: Outdated dependency notifications
- 🔧 **Build Failures**: Critical pipeline failure alerts
- 📊 **Quality Gate Failures**: PR blocking issue creation

### 🔄 Pipeline Integration

**Local Development:**

```bash
# Simulate CI locally
./dev.sh full

# Full validation pipeline
./dev.sh check

# Performance benchmarking
./dev.sh bench
```

**Integration Points:**

- Local/CI alignment through `dev.sh` script
- Automated issue tracking and management
- Performance baseline maintenance
- Security vulnerability tracking
- Dependency health monitoring

## 🔗 Navigation

- **[📖 Documentation](docs/)** - Design docs and architecture
- **[🗺️ Roadmap](docs/plan/MASTER_PLAN.md)** - Long-term development plan
- **[🏗️ Architecture](docs/architecture/)** - System design and decisions
- **[📝 Changelog](CHANGELOG.md)** - Version history and release notes

## 🤝 Contributing

This is a long-term, iterative project developed in public. Contributions are welcome!

1. Check the [Issues](https://github.com/acaradonna/goblin-camp/issues) for open tasks
2. Read the [Architecture docs](docs/architecture/) to understand the codebase
3. Run `./dev.sh` to set up your development environment
4. Create a branch, make changes, and submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Built with ❤️ by the Goblin Camp development team
