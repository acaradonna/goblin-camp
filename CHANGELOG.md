# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### 🏗️ **Mining & Item Management Pipeline**

- **Complete mining-to-stockpile workflow** - Fully functional mining operations with automatic hauling
  - Mining jobs convert wall tiles to floors and spawn stone items
  - Stone items are real ECS entities with Position and Item components
  - Auto-haul system automatically assigns items to nearby stockpiles
  - Multi-pass hauling execution for complex item movement scenarios

#### 📦 **Advanced Inventory System**

- **Spatial inventory management** - Items exist as full ECS entities in the world
  - `Item` component with ItemKind enum (Stone, Wood, Metal, Food, etc.)
  - `Inventory` component for entities that can carry items
  - Proper item pickup, carry, and drop mechanics
  - Integration with job system for hauling assignments

#### 🏪 **Stockpile System**

- **Zone-based storage areas** - Defined storage zones with automatic item organization
  - `Stockpile` component with item type filtering
  - `ZoneBounds` for rectangular storage area definition
  - Spatial queries to find nearest available stockpiles
  - Integration with hauling system for automatic item delivery

#### 💼 **Enhanced Job System**

- **Hierarchical job assignment** - Improved job board with specialized job types
  - `JobKind` enum with Mining, Hauling, Building, and Crafting variants
  - Job execution system with proper state management
  - Job assignment system considering goblin capabilities and proximity
  - Comprehensive job lifecycle from creation to completion

#### 🎯 **Pathfinding Optimizations**

- **LRU caching for performance** - Cached pathfinding with batch processing
  - PathService with configurable cache size (default 1000 entries)
  - Batch pathfinding operations for improved performance
  - Cache hit tracking and performance metrics
  - Integration with hauling and movement systems

#### 🔧 **Development Infrastructure**

- **Enhanced CI/CD pipeline** - Comprehensive GitHub Actions workflow
  - Rust toolchain setup with caching
  - Full test suite execution (unit, integration, doc tests)
  - Code formatting and linting checks
  - Cross-platform testing (Linux, macOS, Windows)
  - Benchmark execution and performance monitoring

#### 📖 **Comprehensive Documentation**

- **Inline code documentation** - Complete code comments throughout the codebase
  - Detailed module-level documentation explaining architecture
  - Function and struct documentation with usage examples
  - System interaction explanations and data flow descriptions
  - Integration examples and troubleshooting guides

#### ✅ **Designation State Management**

- **DesignationState component system** - Implements lifecycle management for designations to prevent duplicate jobs
  - `DesignationState` enum with `Active`, `Ignored`, and `Consumed` states
  - `DesignationLifecycle` component wrapper for ECS integration
  - `designation_dedup_system` to mark duplicate designations at same position as ignored
  - Updated `designation_to_jobs_system` to only process active designations
  - Comprehensive integration tests covering all deduplication scenarios
  - Proper system ordering using Bevy's `.chain()` for deterministic execution

### Fixed

- **Mining job execution** - Fixed wall-to-floor conversion and item spawning
- **Hauling system reliability** - Resolved issues with item pickup and delivery
- **Designation deduplication** - Prevents multiple jobs at the same position
- **Pathfinding edge cases** - Improved handling of blocked paths and unreachable destinations
- **System ordering** - Proper deterministic execution order for all simulation systems
- **Save/load compatibility** - Enhanced serialization for new component types

### Enhanced

- **Performance optimizations** - Pathfinding caching and spatial query improvements
- **Test coverage** - Expanded integration tests for mining, hauling, and inventory systems
- **Error handling** - Improved error messages and failure recovery
- **Code quality** - Zero clippy warnings and consistent formatting throughout

### Technical Details

- All existing functionality remains backward compatible
- No breaking API changes for core simulation engine
- Full test coverage including determinism tests
- Enhanced benchmarking for pathfinding and job systems
- Comprehensive documentation with architecture diagrams

## [0.1.0] - Foundation Release

### Added

- Core ECS scaffolding using Bevy ECS
- Map generation with procedural noise
- A* pathfinding with LRU caching
- Field of view (FOV) and line-of-sight calculations
- Job board and assignment system
- Save/load JSON snapshot functionality
- CLI interface with interactive demo menu
- Comprehensive test suite covering core functionality
- Development tooling (`dev.sh` script)
- Documentation and project roadmap

### Technical Foundation

- Rust workspace with `gc_core` and `gc_cli` crates
- Integration with pathfinding, noise, and serde crates
- ASCII visualization for maps and pathfinding results
- Configurable map dimensions and simulation parameters
