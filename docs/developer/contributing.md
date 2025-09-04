# 🤝 Contribution Guide

> *How to contribute to Goblin Camp development*

This guide walks you through contributing to Goblin Camp, from setting up your development environment to submitting high-quality pull requests.

## 🎯 Quick Navigation

- [Getting Started](#getting-started) - Setup and first steps
- [Development Workflow](#development-workflow) - Day-to-day contribution process
- [Code Standards](#code-standards) - Quality expectations and guidelines
- [Testing Requirements](#testing-requirements) - Ensuring code quality
- [Pull Request Process](#pull-request-process) - How to submit changes
- [Community Guidelines](#community-guidelines) - Working together effectively

---

## 🚀 Getting Started

### Prerequisites

**Required Tools**:
- Rust 1.70+ (latest stable recommended)
- Git 2.20+
- A modern IDE (VS Code with rust-analyzer recommended)

**Optional but Helpful**:
- `cargo-watch` for automatic rebuilds
- `cargo-clippy` for additional linting
- `cargo-audit` for security checks

### Environment Setup

**1. Fork and Clone**:

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/goblin-camp.git
cd goblin-camp

# Add upstream remote
git remote add upstream https://github.com/acaradonna/goblin-camp.git
```

**2. Install Dependencies**:

```bash
# Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install helpful tools
cargo install cargo-watch cargo-audit

# Verify installation
cargo --version
cargo test
```

**3. IDE Configuration**:

VS Code with recommended extensions:
- rust-analyzer (Rust language support)
- CodeLLDB (debugging)
- Better TOML (Cargo.toml editing)
- GitLens (Git integration)

### First Build

```bash
# Build the project
cargo build

# Run tests to ensure everything works
cargo test

# Run the CLI tool
cargo run --bin gc_cli

# Run with development scripts
./dev.sh  # Runs common development tasks
```

---

## 🔄 Development Workflow

### Feature Development Process

**1. Planning Phase**:

```bash
# Always start with latest upstream changes
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/new-pathfinding-algorithm

# Check for existing issues or create one
# Discuss approach in issue comments before major changes
```

**2. Development Phase**:

```bash
# Make changes in small, focused commits
git add src/pathfinding.rs
git commit -m "Add A* pathfinding implementation

- Implement basic A* algorithm for efficient pathfinding
- Add heuristic function for Manhattan distance
- Include tests for path correctness and performance"

# Keep branch updated with upstream
git fetch upstream
git rebase upstream/main
```

**3. Testing Phase**:

```bash
# Run full test suite
cargo test

# Run specific tests for your changes
cargo test pathfinding

# Run performance tests
cargo test --ignored

# Check code formatting
cargo fmt --check

# Run clippy for additional lints
cargo clippy -- -D warnings
```

### PR Validation

**Automated Validation**: All pull requests are automatically validated for commit message format and branch naming standards.

**Local Validation**:
```bash
# Validate your PR before pushing
./dev.sh pr-validate

# Run full validation including PR checks
./dev.sh check
```

**Branch Naming Requirements**:
- `feat/description-here` - New features
- `fix/description-here` - Bug fixes  
- `docs/description-here` - Documentation changes
- `refactor/description-here` - Code refactoring
- `test/description-here` - Test additions/changes
- `chore/description-here` - Maintenance tasks

Use lowercase letters, numbers, and hyphens only.

### Commit Message Guidelines

**Format**:
```
Short summary (50 chars or less)

Longer explanation if needed (wrap at 72 chars):
- Use bullet points for multiple changes
- Explain WHY, not just what changed
- Reference issue numbers: Fixes #123
```

**Examples**:

```bash
# Good commit messages
git commit -m "Fix job assignment race condition

Prevents multiple workers from being assigned to the same job
by checking assignment status before confirming job allocation.

Fixes #45"

git commit -m "Add inventory component to entity system

- Allows entities to carry multiple items
- Integrates with existing hauling system
- Includes capacity limits and item stacking

Related to #67"

# Bad commit messages
git commit -m "fix stuff"
git commit -m "update code"
git commit -m "work in progress"
```

### Code Organization

**File Structure for New Features**:

```
src/
├── lib.rs                 # Public API exports
├── components.rs          # Add new components here
├── systems.rs             # Add new systems here
├── your_feature.rs        # Major new systems get own file
└── ...

tests/
├── your_feature_tests.rs  # Unit tests for your feature
└── integration_tests.rs   # Cross-system integration tests

docs/
├── design/
│   └── your_feature.md    # Design documentation
└── developer/
    └── your_feature.md    # Technical guide
```

**Adding New Components**:

```rust
// In components.rs
#[derive(Component, Debug, Clone, PartialEq)]
pub struct YourComponent {
    pub field1: i32,
    pub field2: String,
}

// Always include helpful implementations
impl Default for YourComponent {
    fn default() -> Self {
        Self {
            field1: 0,
            field2: String::new(),
        }
    }
}

// Add documentation
impl YourComponent {
    /// Creates a new YourComponent with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Creates YourComponent with specific field1 value
    pub fn with_field1(field1: i32) -> Self {
        Self { field1, ..Default::default() }
    }
}
```

**Adding New Systems**:

```rust
// In systems.rs or your_feature.rs
pub fn your_new_system(
    // Always document complex queries
    entities_to_process: Query<(Entity, &YourComponent, &Position), With<SomeMarker>>,
    
    // Group related resources
    mut commands: Commands,
    mut resource: ResMut<YourResource>,
    
    // Use descriptive parameter names
    time: Res<Time>,
) {
    // Include performance monitoring for complex systems
    let _span = tracing::info_span!("your_new_system").entered();
    
    for (entity, component, position) in entities_to_process.iter() {
        // Your system logic here
        process_entity(entity, component, position, &mut commands);
    }
}

// Always include tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_your_new_system() {
        let mut world = World::new();
        // Test setup
        
        let mut schedule = Schedule::default();
        schedule.add_systems(your_new_system);
        schedule.run(&mut world);
        
        // Assertions
        assert_eq!(expected_result, actual_result);
    }
}
```

---

## 📏 Code Standards

### Rust Style Guidelines

**Follow Rust Standard Style**:

```bash
# Format code before committing
cargo fmt

# Check formatting in CI
cargo fmt --check
```

**Naming Conventions**:

```rust
// Components: PascalCase
struct PlayerCharacter { }
struct AssignedJob { }

// Systems: snake_case with descriptive names
fn job_assignment_system() { }
fn mining_execution_system() { }

// Functions: snake_case, descriptive
fn calculate_distance(start: Position, end: Position) -> f32 { }
fn can_assign_job(worker: Entity, job: &Job) -> bool { }

// Constants: SCREAMING_SNAKE_CASE
const MAX_INVENTORY_SIZE: usize = 10;
const DEFAULT_WALKING_SPEED: f32 = 1.0;
```

### Documentation Standards

**Public APIs Must Be Documented**:

```rust
/// Represents a job that can be assigned to workers
///
/// Jobs contain all information needed for a worker to complete a task,
/// including the job type, target location, and priority level.
///
/// # Examples
///
/// ```
/// let mining_job = Job::new(JobKind::Mine { target: (5, 5) }, 1);
/// assert_eq!(mining_job.priority, 1);
/// ```
#[derive(Debug, Clone)]
pub struct Job {
    /// The specific type of work to be performed
    pub kind: JobKind,
    /// Priority level (higher numbers = higher priority)
    pub priority: u32,
    /// Optional worker assignment
    pub assigned_to: Option<Entity>,
}

impl Job {
    /// Creates a new job with the specified kind and priority
    ///
    /// # Arguments
    ///
    /// * `kind` - The type of job to create
    /// * `priority` - Priority level for job assignment
    ///
    /// # Examples
    ///
    /// ```
    /// let job = Job::new(JobKind::Haul { from: (0,0), to: (5,5) }, 2);
    /// ```
    pub fn new(kind: JobKind, priority: u32) -> Self {
        Self {
            kind,
            priority,
            assigned_to: None,
        }
    }
}
```

**Complex Logic Should Be Explained**:

```rust
/// Executes hauling jobs with multi-phase position updates
///
/// This system uses a complex multi-phase approach to avoid borrowing
/// conflicts when updating both carrier and item positions simultaneously.
/// 
/// Phase 1: Collect planned movements for carriers
/// Phase 2: Update carrier positions based on job requirements  
/// Phase 3: Update item positions to match carrying carriers
/// Phase 4: Complete jobs when items reach destinations
pub fn hauling_execution_system(
    // ... parameters
) {
    // Phase 1: Plan carrier movements
    let planned_movements = collect_carrier_plans(&carriers);
    
    // Phase 2: Execute carrier updates
    apply_carrier_movements(planned_movements);
    
    // ... continue with detailed comments for each phase
}
```

### Error Handling

**Use Appropriate Error Types**:

```rust
// Define specific error types for your domain
#[derive(Debug, thiserror::Error)]
pub enum PathfindingError {
    #[error("No path exists from {from:?} to {to:?}")]
    NoPathExists { from: Position, to: Position },
    
    #[error("Invalid starting position: {pos:?}")]
    InvalidStart { pos: Position },
    
    #[error("Map bounds exceeded: {pos:?}")]
    OutOfBounds { pos: Position },
}

// Use Result types for fallible operations
pub fn find_path(start: Position, goal: Position) -> Result<Vec<Position>, PathfindingError> {
    if !is_valid_position(start) {
        return Err(PathfindingError::InvalidStart { pos: start });
    }
    
    // ... pathfinding logic
    
    Ok(path)
}
```

---

## 🧪 Testing Requirements

### Test Coverage Requirements

**All Public Functions Must Have Tests**:

```rust
// src/pathfinding.rs
pub fn find_shortest_path(start: Position, goal: Position) -> Option<Vec<Position>> {
    // Implementation
}

// tests/pathfinding_tests.rs
#[cfg(test)]
mod pathfinding_tests {
    use super::*;

    #[test]
    fn test_find_shortest_path_basic() {
        let start = Position(0, 0);
        let goal = Position(2, 2);
        
        let path = find_shortest_path(start, goal).unwrap();
        
        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&goal));
        assert!(path.len() >= 3); // Minimum path length
    }
    
    #[test]
    fn test_find_shortest_path_no_path() {
        // Test with blocked path
        let result = find_shortest_path(blocked_start, blocked_goal);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_find_shortest_path_same_position() {
        let pos = Position(5, 5);
        let path = find_shortest_path(pos, pos).unwrap();
        assert_eq!(path, vec![pos]);
    }
}
```

**Integration Tests for System Interactions**:

```rust
// tests/job_system_integration_tests.rs
#[test]
fn test_complete_mining_workflow() {
    let mut world = World::new();
    setup_test_world(&mut world);
    
    // Create mining designation
    let designation = world.spawn((
        MineDesignation,
        Position(5, 5),
        DesignationLifecycle::active(),
    )).id();
    
    // Add worker
    let worker = world.spawn((
        Miner,
        Position(0, 0),
        AssignedJob(None),
    )).id();
    
    // Run systems in order
    run_systems(&mut world, &[
        designation_to_jobs_system,
        job_assignment_system,
        mining_execution_system,
    ]);
    
    // Verify complete workflow
    assert_eq!(designation_count(&world), 0); // Designation consumed
    assert_eq!(job_count(&world), 0);         // Job completed  
    assert_eq!(item_count(&world), 1);        // Item spawned
    assert_worker_has_no_job(&world, worker); // Worker available
}
```

### Performance Testing

**Include Benchmarks for Complex Operations**:

```rust
// benches/pathfinding.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_pathfinding(c: &mut Criterion) {
    let map = create_large_test_map(100, 100);
    
    c.bench_function("pathfinding_across_map", |b| {
        b.iter(|| {
            find_shortest_path(
                black_box(Position(0, 0)),
                black_box(Position(99, 99))
            )
        });
    });
}

criterion_group!(benches, benchmark_pathfinding);
criterion_main!(benches);
```

### Test Organization

**Structure Tests Logically**:

```
tests/
├── unit/                  # Individual function tests
│   ├── pathfinding_tests.rs
│   ├── job_tests.rs
│   └── inventory_tests.rs
├── integration/           # Multi-system tests
│   ├── mining_workflow_tests.rs
│   ├── hauling_workflow_tests.rs
│   └── save_load_tests.rs
├── determinism/           # Determinism validation
│   └── simulation_determinism_tests.rs
└── helpers/              # Test utilities
    ├── mod.rs
    ├── world_setup.rs
    └── assertions.rs
```

---

## 📝 Pull Request Process

### Before Submitting

**Pre-submission Checklist**:

- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Documentation updated for public API changes
- [ ] CHANGELOG.md updated with user-facing changes
- [ ] Commit messages follow guidelines
- [ ] Branch is up-to-date with upstream main

### PR Description Template

**Use This Template**:

```markdown
## Summary
Brief description of what this PR does and why.

## Changes
- List specific changes made
- Use bullet points for clarity
- Link to relevant issues: Fixes #123

## Testing
- Describe testing performed
- Include any new test cases added
- Note any manual testing done

## Breaking Changes
- List any breaking changes
- Explain migration path if applicable

## Documentation
- [ ] Updated relevant documentation
- [ ] Added/updated code comments
- [ ] Updated CHANGELOG.md

## Checklist
- [ ] Tests pass locally
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
```

### Review Process

**What Reviewers Look For**:

1. **Correctness**: Does the code work as intended?
2. **Clarity**: Is the code easy to understand?
3. **Performance**: Are there any performance implications?
4. **Testing**: Is the code adequately tested?
5. **Documentation**: Are public APIs documented?
6. **Maintainability**: Will this code be easy to maintain?

**Responding to Feedback**:

```bash
# Make requested changes
git add changed_files
git commit -m "Address review feedback

- Fix variable naming in pathfinding module
- Add documentation for new public functions
- Improve error handling in job assignment"

# Push updates
git push origin feature/your-branch-name
```

### Merge Process

**After Approval**:

1. Ensure branch is up-to-date with main
2. Squash commits if requested by maintainers
3. Wait for maintainer to merge (don't merge your own PRs)
4. Delete feature branch after merge

---

## 🌟 Community Guidelines

### Communication Standards

**Be Respectful and Constructive**:
- Focus on the code, not the person
- Provide specific, actionable feedback
- Assume good intentions
- Ask questions when something is unclear

**Example Feedback**:

```markdown
# Good feedback
"This function could be more efficient by using a HashMap instead of 
searching through the Vec each time. Here's an example: [code snippet]"

"I'm not sure I understand the logic here. Could you add a comment 
explaining why we need to check this condition twice?"

# Poor feedback  
"This is wrong."
"Why did you do it this way?"
"This code is bad."
```

### Getting Help

**Where to Ask Questions**:

1. **GitHub Issues**: Bug reports, feature requests
2. **GitHub Discussions**: General questions, design discussions
3. **Code Comments**: Specific implementation questions in PRs

**How to Ask Good Questions**:

```markdown
## Good Question Format

**What I'm trying to do:**
I want to add a new component for tracking entity health.

**What I've tried:**
I added the component to components.rs and created a system to update health,
but I'm getting borrowing errors when trying to access both Health and Position.

**Specific error:**
[paste error message]

**Code:**
[minimal code example]
```

### Recognition

**Contributors Are Valued**:
- All contributors are listed in CONTRIBUTORS.md
- Significant contributions are highlighted in release notes
- First-time contributors get extra support and encouragement

**Types of Contributions Welcomed**:
- Code improvements and new features
- Bug reports and fixes
- Documentation improvements
- Test coverage expansion
- Performance optimizations
- Design feedback and suggestions

---

## 📚 Additional Resources

### Development Tools

**Recommended VS Code Extensions**:
- rust-analyzer: Rust language support
- CodeLLDB: Debugging support
- GitLens: Enhanced Git integration
- Todo Tree: Track TODO comments
- Better TOML: Cargo.toml syntax highlighting

**Useful Cargo Commands**:

```bash
# Development workflow
cargo watch -x test           # Auto-run tests on changes
cargo watch -x "run --bin gc_cli"  # Auto-rebuild and run

# Code quality
cargo clippy --all-targets --all-features  # Full linting
cargo audit                   # Security audit
cargo outdated               # Check for outdated dependencies

# Performance analysis
cargo bench                  # Run benchmarks
cargo flamegraph --bin gc_cli  # Generate performance flamegraph
```

### Learning Resources

**Rust-Specific**:
- [The Rust Book](https://doc.rust-lang.org/book/) - Essential Rust learning
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Advanced topics

**Bevy ECS**:
- [Bevy Book](https://bevyengine.org/learn/book/) - ECS fundamentals
- [ECS Guide](../ecs-guide.md) - Our specific ECS patterns
- [Systems Reference](../api/systems.md) - System implementation details

### Project-Specific Documentation

- [Developer Guide](../README.md) - Getting started with development
- [Architecture Overview](../../architecture/01_overview.md) - High-level design
- [Design Documents](../../design/) - Detailed system designs
- [Testing Guide](../testing.md) - Testing strategies and patterns

---

*Thank you for contributing to Goblin Camp! Your efforts help make this project better for everyone.*
