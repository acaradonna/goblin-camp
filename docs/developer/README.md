# 🛠️ Developer Guide

> *Complete guide for Goblin Camp development*

Welcome to Goblin Camp development! This guide will help you understand the codebase, set up your development environment, and contribute effectively to the project.

## 🎯 Quick Navigation

- [Getting Started](#getting-started) - Setup and first steps
- [Architecture Overview](#architecture-overview) - Understanding the codebase
- [Development Workflow](#development-workflow) - Day-to-day development process
- [Documentation Hub](#documentation-hub) - All development guides
- [API Reference](#api-reference) - Detailed technical documentation
- [Common Tasks](#common-tasks) - Frequent development scenarios
- [Troubleshooting](#troubleshooting) - Solving common issues

---

## 📚 Documentation Hub

### Core Development Guides

**🏗️ [ECS Architecture Guide](./ecs-guide.md)**
- Deep dive into Entity-Component-System patterns
- Component design principles and best practices
- System organization and execution ordering
- Query patterns and performance optimization

**🔧 [Job System Deep Dive](./job-system.md)**
- Complete job execution pipeline documentation
- Task assignment and worker management
- Designation processing and job lifecycle
- Mining and hauling system implementation

**🧪 [Testing Guide](./testing.md)**
- Comprehensive testing strategies and patterns
- Unit, integration, and determinism testing
- ECS testing patterns and helper utilities
- Performance testing and benchmarking

**📊 [Code Coverage Guide](./coverage.md)**
- Code coverage tools and measurement (cargo-llvm-cov, SonarQube)
- Coverage thresholds and quality standards (90%+ core library)
- Local development and CI integration
- Coverage-driven development practices

**⚡ [Performance Guide](./performance.md)**
- Profiling tools and optimization techniques
- Memory management and allocation strategies
- System-level performance patterns
- Common bottlenecks and solutions

**🤝 [Contributing Guide](./contributing.md)**
- Complete contribution workflow
- Code standards and review process
- Development environment setup
- Community guidelines and communication

### API References

**📋 [Components Reference](./api/components.md)**
- Complete documentation of all ECS components
- Component usage patterns and examples
- Entity archetypes and component combinations
- Cross-references to related systems

**⚙️ [Systems Reference](./api/systems.md)**
- Detailed documentation of all ECS systems
- System parameters and execution requirements
- Performance characteristics and optimization notes
- Integration patterns and dependencies

---

## 🧠 Learning Paths

### For New Contributors

**1. Start Here** (30-60 minutes):
- Read [Architecture Overview](#architecture-overview) below
- Browse [ECS Architecture Guide](./ecs-guide.md) - focus on fundamentals
- Run the codebase locally following [Getting Started](#getting-started)

**2. Understand the Systems** (1-2 hours):
- Deep dive: [Job System Documentation](./job-system.md)
- Practice: [Common Tasks](#common-tasks) examples below
- Reference: [Components API](./api/components.md) for entity structure

**3. Start Contributing** (ongoing):
- Follow [Contributing Guide](./contributing.md) workflow
- Use [Testing Guide](./testing.md) for quality assurance
- Apply [Performance Guide](./performance.md) for optimization

### For Experienced Rust Developers

**Quick Start** (15-30 minutes):
- Skim [ECS Architecture Guide](./ecs-guide.md) - focus on Goblin Camp specifics
- Review [Systems Reference](./api/systems.md) - understand execution order
- Jump to [Common Tasks](#common-tasks) for immediate productivity

### For Game Development Veterans

**Focus Areas**:
- [Job System Deep Dive](./job-system.md) - unique task management approach
- [Performance Guide](./performance.md) - ECS optimization techniques
- [Testing Guide](./testing.md) - determinism and simulation testing

---

## 🚀 Getting Started

### Prerequisites

**Required**:
- Rust 1.70+ (latest stable recommended)
- Git 2.20+
- A modern IDE (VS Code with rust-analyzer recommended)

**Optional but Helpful**:
- `cargo-watch` for automatic rebuilds
- `cargo-clippy` for additional linting
- `cargo-audit` for security checks

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/acaradonna/goblin-camp.git
cd goblin-camp

# Install helpful tools
cargo install cargo-watch cargo-audit

# Verify everything works
cargo build
cargo test

# Run the CLI
cargo run --bin gc_cli

# Use development scripts
./dev.sh  # Convenience script for common tasks
```

### IDE Configuration

**VS Code** (recommended):
1. Install rust-analyzer extension
2. Install CodeLLDB for debugging
3. Open workspace: `code goblin-camp`
4. Accept workspace extension recommendations

**Other IDEs**:
- CLion with Rust plugin
- Vim/Neovim with rust-analyzer LSP
- Emacs with rust-mode

---

## 🏗️ Architecture Overview

Goblin Camp uses a **deterministic Entity-Component-System (ECS)** architecture built on Bevy ECS, designed for reliable simulation and gameplay.

### Core Concepts

**Entity-Component-System**:
- **Entities**: Game objects (workers, items, tiles)
- **Components**: Data attached to entities (Position, Inventory, Job assignments)
- **Systems**: Logic that operates on entities with specific components

**Deterministic Simulation**:
- Fixed timestep progression
- Seeded random number generation
- Predictable system execution order
- Reproducible game states

### Key Systems

**Job Management Pipeline**:
```
Player Designations → Jobs → Worker Assignment → Task Execution → Completion
```

**Spatial Systems**:
- Position tracking and movement
- Field of view calculations
- Pathfinding and navigation

**Item Management**:
- Item spawning and despawning
- Inventory and carrying mechanics
- Stockpile organization

### Project Structure

```
crates/
├── gc_core/           # Core simulation logic
│   ├── src/
│   │   ├── components.rs    # ECS components
│   │   ├── systems.rs       # ECS systems
│   │   ├── jobs.rs         # Job management
│   │   ├── world.rs        # World state
│   │   └── ...
│   └── tests/         # Unit and integration tests
└── gc_cli/            # Command-line interface
    └── src/main.rs    # Entry point
```

---

## 🔄 Development Workflow

### Daily Development Process

**1. Update and Prepare**:
```bash
git checkout main
git pull origin main
git checkout -b feature/your-feature-name
```

**2. Develop with Fast Feedback**:
```bash
# Auto-rebuild and test on changes
cargo watch -x test

# Auto-run specific test
cargo watch -x "test your_test_name"

# Auto-rebuild and run CLI
cargo watch -x "run --bin gc_cli"
```

**3. Verify Quality**:
```bash
# Run full test suite
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Check for security issues
cargo audit
```

**4. Submit Changes**:
```bash
git add .
git commit -m "feat: add new pathfinding algorithm

- Implement A* algorithm for efficient pathfinding
- Add tests for correctness and performance
- Update documentation with usage examples"

git push origin feature/your-feature-name
# Create pull request on GitHub
```

### Code Quality Standards

**Formatting**: Use `cargo fmt` before commits
**Testing**: All public functions must have tests
**Documentation**: Public APIs must be documented
**Performance**: Profile changes that affect simulation speed

---

## 📋 API Reference

### Quick Reference Links

**Core Components**:
- [Position](./api/components.md#position) - Spatial location
- [AssignedJob](./api/components.md#assignedjob) - Work assignments
- [Inventory](./api/components.md#inventory) - Item carrying
- [Worker Types](./api/components.md#worker-components) - Miner, Carrier roles

**Essential Systems**:
- [job_assignment_system](./api/systems.md#job_assignment_system) - Assigns tasks to workers
- [mining_execution_system](./api/systems.md#mining_execution_system) - Executes mining operations
- [hauling_execution_system](./api/systems.md#hauling_execution_system) - Moves items between locations

**Resources**:
- [JobBoard](./api/components.md#jobboard) - Global task queue
- [GameMap](./api/components.md#gamemap) - World tiles and spatial data
- [Time](./api/components.md#time) - Simulation clock

---

## ⚡ Common Tasks

### Adding a New Component

```rust
// In src/components.rs
#[derive(Component, Debug, Clone, PartialEq)]
pub struct YourComponent {
    pub field1: i32,
    pub field2: String,
}

impl Default for YourComponent {
    fn default() -> Self {
        Self {
            field1: 0,
            field2: String::new(),
        }
    }
}

// Add to lib.rs exports
pub use components::YourComponent;
```

**Test the Component**:
```rust
// In tests/your_component_tests.rs
#[test]
fn test_your_component_creation() {
    let component = YourComponent::default();
    assert_eq!(component.field1, 0);
    assert_eq!(component.field2, "");
}
```

### Creating a New System

```rust
// In src/systems.rs
pub fn your_new_system(
    entities_to_process: Query<(Entity, &YourComponent), With<SomeMarker>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, component) in entities_to_process.iter() {
        // System logic here
        if component.field1 > 0 {
            commands.entity(entity).insert(SomeOtherComponent);
        }
    }
}
```

**Add to System Schedule**:
```rust
// In main schedule creation
schedule.add_systems(your_new_system.after(existing_system));
```

### Adding a New Job Type

```rust
// In src/jobs.rs
#[derive(Debug, Clone, PartialEq)]
pub enum JobKind {
    Mine { target: (i32, i32) },
    Haul { from: (i32, i32), to: (i32, i32) },
    YourNewJob { parameter: String }, // Add your job type
}

// In appropriate system
pub fn your_job_execution_system(
    mut job_board: ResMut<JobBoard>,
    workers: Query<(Entity, &Position), With<YourWorkerType>>,
) {
    for (worker_entity, position) in workers.iter() {
        if let Some(job) = get_assigned_job(worker_entity, JobKind::YourNewJob { .. }) {
            // Execute your job logic
            execute_your_job(worker_entity, job);
        }
    }
}
```

### Writing Integration Tests

```rust
// In tests/your_feature_tests.rs
#[test]
fn test_your_workflow() {
    let mut world = World::new();
    setup_test_world(&mut world);
    
    // Setup test scenario
    let entity = world.spawn((
        YourComponent::default(),
        Position(5, 5),
    )).id();
    
    // Run systems
    let mut schedule = Schedule::default();
    schedule.add_systems(your_new_system);
    schedule.run(&mut world);
    
    // Verify results
    let result_component = world.get::<SomeOtherComponent>(entity);
    assert!(result_component.is_some());
}
```

### Debugging System Execution

```rust
// Add tracing to your system
pub fn your_system(
    query: Query<&YourComponent>,
) {
    tracing::info!("Starting your_system with {} entities", query.iter().count());
    
    for component in query.iter() {
        tracing::debug!("Processing component: {:?}", component);
        // System logic
    }
    
    tracing::info!("Completed your_system");
}

// Run with tracing
RUST_LOG=debug cargo run --bin gc_cli
```

---

## 🔧 Troubleshooting

### Common Build Issues

**"Cannot find crate" Errors**:
```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update
```

**Formatting Conflicts**:
```bash
# Fix formatting
cargo fmt

# Check what would change
cargo fmt --check
```

### Test Failures

**Non-Deterministic Test Failures**:
- Check for missing `DeterministicRng` usage
- Verify system execution order
- Look for race conditions in parallel queries

**Performance Test Failures**:
```bash
# Run performance tests specifically
cargo test --ignored

# Profile slow tests
cargo test your_slow_test -- --nocapture
```

### IDE Issues

**rust-analyzer Not Working**:
1. Reload VS Code window: `Ctrl+Shift+P` → "Developer: Reload Window"
2. Check Rust toolchain: `rustup show`
3. Clear rust-analyzer cache: `Ctrl+Shift+P` → "rust-analyzer: Restart Server"

**Missing Completions**:
- Ensure `Cargo.toml` is properly configured
- Check that all dependencies are resolved: `cargo check`

### Runtime Issues

**Simulation Not Progressing**:
- Check that `advance_time` system is running
- Verify system execution order
- Look for infinite loops in system logic

**Memory Usage Growing**:
- Profile with `cargo flamegraph`
- Check for entity leaks (entities not being despawned)
- Monitor resource usage with built-in performance tools

---

## 📚 Additional Resources

### Documentation

- [Architecture Documents](../architecture/) - High-level design decisions
- [Design Documents](../design/) - Detailed system designs
- [GitHub Issues](https://github.com/acaradonna/goblin-camp/issues) - Bug reports and feature requests

### External Resources

**Rust Learning**:
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)

**Bevy ECS**:
- [Bevy Book](https://bevyengine.org/learn/book/)
- [ECS Examples](https://github.com/bevyengine/bevy/tree/main/examples/ecs)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)

**Game Development**:
- [Game Programming Patterns](https://gameprogrammingpatterns.com/)
- [Entity-Component-System FAQ](https://github.com/SanderMertens/ecs-faq)

---

*This guide provides a comprehensive foundation for Goblin Camp development. For specific technical details, refer to the linked documentation and API references.*