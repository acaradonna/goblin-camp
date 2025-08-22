# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **DesignationState component system** - Implements lifecycle management for designations to prevent duplicate jobs ([#10](https://github.com/acaradonna/goblin-camp/issues/10))
  - `DesignationState` enum with `Active`, `Ignored`, and `Consumed` states
  - `DesignationLifecycle` component wrapper for ECS integration
  - `designation_dedup_system` to mark duplicate designations at same position as ignored
  - Updated `designation_to_jobs_system` to only process active designations
  - Comprehensive integration tests covering all deduplication scenarios
  - Proper system ordering using Bevy's `.chain()` for deterministic execution
- **Enhanced designation system** - Updated `DesignationBundle` to include lifecycle component with sensible defaults

### Fixed
- Duplicate job creation when multiple designations exist at the same position
- Non-deterministic behavior in designation processing order

### Technical Details
- All existing functionality remains backward compatible
- No breaking API changes
- Full test coverage with 7 new integration tests
- Zero clippy warnings and proper code formatting maintained

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