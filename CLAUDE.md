# CLAUDE.md

AI collaboration guide for Goblin Camp development. This document encodes how to work with Claude on this colony management game project and should be updated as practices evolve.

## Project Overview

**Goblin Camp** is a complex colony management game inspired by Dwarf Fortress, built in Rust using Entity-Component-System (ECS) architecture. The game centers around managing a goblin colony with rich simulation systems for mining, crafting, combat, and survival.

**Key Technologies:**
- Rust 1.70+ with Bevy ECS for high-performance simulation
- Deterministic simulation with fixed-step timing and seeded RNG
- JSON serialization for save/load functionality
- CLI-first interface with planned TUI expansion

## Assistant Operating Checklist (Always Follow)

1. **Read context first**: Understand the current development phase from `docs/plan/MASTER_PLAN.md`
2. **Run quality checks**: Execute `./dev.sh check` before any commits - this is mandatory
3. **Plan before coding**: Use TodoWrite tool for multi-step tasks, break work into atomic commits
4. **Follow ECS patterns**: Understand component/system design, respect system execution order
5. **Test comprehensively**: Add unit tests, integration tests, and determinism tests for all changes
6. **Update documentation**: Inline comments, README updates, CHANGELOG.md for user-facing changes
7. **Validate with demos**: Run `./dev.sh demo` to verify changes work in practice
8. **Maintain determinism**: Never use std::time in simulation, use seeded RNG streams

## Core Principles

### 1. Read First, Plan, Then Act
- Always examine existing code patterns before proposing changes
- Read relevant files completely rather than making assumptions
- Use TodoWrite tool to plan multi-step work and track progress
- Understand the current master plan phase and constraints

### 2. Deterministic-First Development
- All simulation systems must be deterministic with seeded RNG
- Use `DeterministicRng` resource with separate streams per subsystem
- Test determinism with identical seeds producing identical results
- Never use wall-clock time in simulation logic

### 3. ECS Architecture Discipline
- Components are pure data structures with no methods
- Systems contain all logic and operate on component queries
- Use proper system ordering with `.chain()` and `.after()`/`.before()`
- Follow composition over inheritance patterns

### 4. Quality Gates Are Mandatory
- `./dev.sh check` must pass before any commit (format, lint, tests)
- Add comprehensive tests for all new functionality
- Benchmark performance-critical paths (pathfinding, FOV, job systems)
- Maintain zero clippy warnings

### 5. Game Domain Understanding
- Mining: Convert Wall→Floor, spawn Stone items, assign to miners
- Hauling: Move items to stockpiles via carriers with inventory
- Jobs: Hierarchical system with designation→job→assignment→execution
- Spatial simulation: All entities have Position, items are full ECS entities

## Architecture Deep Dive

### Crate Structure

```text
goblin-camp/
├── crates/gc_core/     # Core simulation engine
│   ├── src/
│   │   ├── components.rs    # All ECS components
│   │   ├── systems.rs      # Core simulation systems
│   │   ├── jobs.rs         # Job board and execution
│   │   ├── world.rs        # Spatial representation  
│   │   ├── path.rs         # A* pathfinding with caching
│   │   ├── fov.rs          # Field of view calculations
│   │   └── ...
│   └── tests/          # Integration tests
└── crates/gc_cli/      # CLI interface and demos
```

### Key ECS Patterns Used

#### Entity Composition

```rust
// Goblin miner entity
world.spawn((
    Goblin,              // Marker component
    Miner,               // Capability marker  
    Position(x, y),      // Spatial data
    AssignedJob(None),   // Current task
    Inventory(None),     // Can carry items
));

// Item entity (items are full ECS entities!)
world.spawn((
    Item { item_type: ItemType::Stone },
    Stone,               // Specific marker
    Position(x, y),      // Items have positions
    Carriable,           // Can be picked up
));
```

#### System Execution Order (Critical!)

```rust
schedule.add_systems((
    // Phase 1: Input processing
    designation_dedup_system,
    
    // Phase 2: Job management (must be chained!)
    (
        designation_to_jobs_system,
        job_assignment_system,  
    ).chain(),
    
    // Phase 3: Job execution
    (
        mining_execution_system,
        process_item_spawn_queue_system,
        auto_haul_system,
        hauling_execution_system,
    ),
    
    // Phase 4: Cleanup  
    advance_time,
));
```

### Component Design Guidelines

#### Marker Components (no data, just tags)

```rust
#[derive(Component, Debug)]
pub struct Goblin;  // Entity type marker

#[derive(Component, Debug)]  
pub struct Miner;   // Capability marker
```

#### Data Components (pure data, no methods)

```rust
#[derive(Component, Debug, Default)]
pub struct AssignedJob(pub Option<JobId>);

#[derive(Component, Debug, Default)]
pub struct Inventory(pub Option<Entity>);  // Carried item
```

#### Zone Components (spatial areas)

```rust
#[derive(Component, Debug, Clone)]
pub struct ZoneBounds {
    pub min_x: i32, pub min_y: i32,
    pub max_x: i32, pub max_y: i32,
}
```

## Development Workflow

### Daily Development Process

1. **Setup Check**: `./dev.sh` (builds, tests, verifies everything)
2. **Plan Work**: Use TodoWrite tool for complex tasks
3. **Code Changes**: Follow ECS patterns, add comprehensive tests
4. **Quality Check**: `./dev.sh check` (formatting, lint, tests)
5. **Demo Validation**: `./dev.sh demo` to verify changes work
6. **Commit**: Atomic commits with descriptive messages

### Essential Commands

```bash
# Setup and validation (run first)
./dev.sh                    # Complete setup: build + test + verify
./dev.sh check              # Quality gates: format + lint + test

# Development workflow  
./dev.sh demo               # Interactive demo menu
./dev.sh test               # Run all tests
./dev.sh lint               # Clippy linting
./dev.sh format             # Format code

# Specific demos for validation
cargo run -p gc_cli -- menu          # Interactive menu
cargo run -p gc_cli -- jobs          # Job system demo
cargo run -p gc_cli -- mapgen        # Map generation  
cargo run -p gc_cli -- path          # Pathfinding demo
```

### Testing Strategy

#### Test Categories Required

1. **Unit Tests**: Individual function testing in `src/` modules
2. **Integration Tests**: Multi-system workflows in `tests/`
3. **Determinism Tests**: Same seed → identical results
4. **Benchmarks**: Performance tests for hot paths

#### Integration Test Pattern

```rust
#[test]  
fn complete_mining_workflow() {
    let mut world = World::new();
    setup_deterministic_world(&mut world, 42); // Seeded RNG
    
    // Create entities: miner, designation, stockpile
    let miner = world.spawn((Miner, Position(5, 5), AssignedJob::default())).id();
    
    // Build schedule with proper system ordering
    let mut schedule = Schedule::default();
    schedule.add_systems(/* systems in correct order */);
    
    // Run simulation steps
    schedule.run(&mut world);  // Step 1: Create job
    schedule.run(&mut world);  // Step 2: Execute mining
    
    // Verify end-to-end results
    assert_wall_became_floor(&world, 5, 5);
    assert_stone_item_created(&world);
    assert_job_completed(&world, miner);
}
```

## Code Standards and Patterns

### Rust Style Requirements

- Use `cargo fmt` for consistent formatting
- Zero clippy warnings: `cargo clippy -- -D warnings`  
- Document all public APIs with examples
- Use descriptive variable names, especially in complex systems

### ECS Component Patterns

```rust
// ✅ Good: Pure data component
#[derive(Component, Debug, Default)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

// ❌ Bad: Methods on components
impl Health {
    pub fn heal(&mut self, amount: i32) { /* NO! */ }
}

// ✅ Good: Logic in systems  
fn healing_system(mut healers: Query<&mut Health, With<Injured>>) {
    // Healing logic here
}
```

### System Design Patterns

```rust
// Multi-pass system to avoid borrowing conflicts
pub fn hauling_execution_system(
    mut param_set: ParamSet<(
        Query<&mut AssignedJob, With<Carrier>>,
        Query<&mut Position, With<Item>>, 
    )>
) {
    // Phase 1: Plan updates
    let updates = collect_planned_updates(param_set.p0());
    
    // Phase 2: Apply updates
    apply_updates(param_set.p1(), updates);
}
```

### Documentation Standards

```rust
/// Executes mining jobs to convert Wall tiles to Floor and spawn Stone items
///
/// This system processes all Miner entities with assigned Mine jobs. Miners must
/// be adjacent to (or at) the target tile to successfully mine it. Mining converts
/// Wall tiles to Floor tiles and spawns Stone items as full ECS entities.
///
/// # System Dependencies
/// 
/// Must run after job_assignment_system and before auto_haul_system.
/// 
/// # Performance Notes
///
/// Processes all miners in parallel. Item spawning is deferred to avoid
/// entity creation during iteration.
pub fn mining_execution_system(/* params */) {
    // Implementation with detailed inline comments
}
```

## Game Domain Knowledge

### Mining System

- **Input**: Wall tiles marked with MineDesignation
- **Process**: Miner moves adjacent, converts Wall→Floor
- **Output**: Stone item spawned at mined location
- **Integration**: Auto-haul picks up stones for stockpiles

### Job System Architecture

```text
Player Input → Designation → Job Creation → Assignment → Execution → Completion
    ↓              ↓           ↓             ↓           ↓          ↓
MineDesignation → JobKind   → JobBoard    → AssignedJob → Systems  → Cleanup
```

### Item Management

- Items are **full ECS entities** with Position, not just data
- Inventory holds `Option<Entity>` reference to carried item
- Item movement updates entity Position directly
- Carriable marker enables pickup by Carrier entities

### Stockpile System

- Defined by Position + ZoneBounds components
- Auto-haul finds nearest stockpile for new items
- Hauling jobs move items from spawn point to stockpile center

### Spatial Representation

- 2D tile-based world with GameMap resource
- All entities have Position(x, y) component
- TileKind enum: Floor, Wall (more planned)
- Pathfinding uses A* with LRU caching

## Performance Considerations

### Hot Paths That Need Benchmarks

1. **Pathfinding**: A* algorithm with LRU cache
2. **FOV Calculations**: Line-of-sight for multiple entities  
3. **Job Assignment**: Spatial queries for nearest workers
4. **Item Hauling**: Multi-pass system with spatial updates

### Memory Management

- Reuse collections in hot paths: `Vec::with_capacity()`
- Cache pathfinding results with configurable LRU size
- Use `ParamSet` for complex multi-query systems
- Batch entity spawning to avoid mid-iteration spawns

### Determinism Performance

- Separate RNG streams prevent cross-contamination overhead
- Fixed-step timing eliminates frame-rate dependencies
- Explicit system ordering reduces scheduling overhead

## Common Development Tasks

### Adding New Component Type

1. Define in `components.rs` with derive macros
2. Add to prelude module exports
3. Update save/load serialization if persistent
4. Add documentation with usage examples
5. Write unit tests for component behavior

### Adding New System

1. Implement system function with proper Query types
2. Add to appropriate system phase in schedule ordering
3. Write integration tests covering system interactions
4. Add performance benchmark if system is hot path
5. Document system dependencies and execution requirements

### Creating New Job Type

1. Add variant to JobKind enum in `jobs.rs`
2. Update job execution systems to handle new type
3. Add job assignment logic for worker capabilities
4. Create integration tests for full job lifecycle
5. Update demo to showcase new job type

### Adding New Item Type

1. Add variant to ItemType enum
2. Create specific marker component (e.g., Wood, Metal)
3. Update item spawning systems  
4. Add stockpile filtering logic if needed
5. Test item lifecycle: spawn → pickup → haul → stockpile

## Quality Assurance Checklist

Before any commit, ensure:

- [ ] `./dev.sh check` passes completely (format, lint, tests)
- [ ] All new code has comprehensive tests (unit + integration)
- [ ] Determinism tests pass with identical seeds
- [ ] Performance benchmarks show no regressions
- [ ] Documentation updated for public API changes
- [ ] Demo validation confirms changes work as expected
- [ ] CHANGELOG.md updated for user-visible changes
- [ ] System ordering preserved for deterministic execution

## Troubleshooting Guide

### Common ECS Issues

#### "Borrow checker conflicts in systems"

- Use `ParamSet` for multiple mutable queries
- Split into multiple systems with proper ordering
- Use read-only queries where possible

#### "System ordering problems"

- Use `.chain()` for dependent systems
- Add explicit `.after()` and `.before()` constraints
- Test with determinism tests to catch ordering issues

#### "Items not being hauled"

- Check auto_haul_system runs after item spawn systems
- Verify stockpiles exist with proper Position + Stockpile components
- Ensure carriers have Carrier + Inventory components

### Performance Issues

#### "Pathfinding too slow"

- Increase LRU cache size in PathService
- Add benchmarks to measure actual impact
- Consider batching pathfinding requests

#### "Tests failing intermittently"

- Check for non-deterministic behavior (missing seeded RNG)
- Verify proper system execution order
- Add determinism tests with fixed seeds

### Development Environment

#### "./dev.sh failing"

- Ensure Rust 1.70+ installed: `rustup update`
- Clear target directory: `cargo clean`
- Check disk space for build artifacts
- Verify all dependencies available

## Project-Specific Insights

### Master Plan Phases

- **M0 (Complete)**: ECS foundation, map, FOV, A*, jobs MVP, save/load, CLI
- **M1 (Current)**: Determinism + designation lifecycle management
- **M2 (Active)**: Job execution MVP with mining→items→stockpiles pipeline
- **M3 (Planned)**: TUI prototype with interactive interface

### Architecture Decision Records (ADRs)

- **Time and Determinism**: Fixed-step scheduling, seeded RNG streams
- Reference: `docs/architecture/adr/0001-time-determinism.md`

### Code Organization Philosophy

- Core simulation engine (`gc_core`) remains headless and testable
- User interfaces (`gc_cli`, future `gc_tui`) are thin presentation layers
- All game logic lives in ECS systems for maximum testability
- Integration tests verify complete workflows rather than isolated functions

---

## Maintenance

This document should be updated when:

- New architectural patterns are established
- Development workflow changes
- New quality gates are added  
- Performance requirements change
- New project phases begin

**Last Updated**: Generated by Claude Code for comprehensive Goblin Camp development assistance