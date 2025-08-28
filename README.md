# 🏰 Goblin Camp

> *A complex, deep simulation/colony management game inspired by Dwarf Fortress, reimagined around goblins.*

[![CI Status](https://github.com/acaradonna/goblin-camp/workflows/CI/badge.svg)](https://github.com/acaradonna/goblin-camp/actions)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://acaradonna.github.io/goblin-camp/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

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

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For cloning the repository

### 📥 Installation

```bash
git clone https://github.com/acaradonna/goblin-camp.git
cd goblin-camp
./dev.sh  # One-command setup: builds, tests, and verifies everything
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
├── docs/                 # 📚 Design documentation
│   ├── architecture/     # System architecture
│   ├── design/          # Feature design docs
│   └── plan/            # Development roadmap
└── target/              # Build artifacts
```

## 🧪 Development

### Using the dev script (recommended)

- Setup: `./dev.sh` (builds, tests, and verifies everything works)
- Demo menu: `./dev.sh demo`
- Quick checks: `./dev.sh check` (fast formatting and lint checks)
- Benchmarks: `./dev.sh bench`

### Manual commands

- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run -p gc_cli`
- Docs: `cargo doc --open`

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

*Built with ❤️ by the Goblin Camp development team*
