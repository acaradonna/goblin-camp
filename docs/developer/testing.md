# 🧪 Testing Guide

> *Comprehensive testing strategies for Goblin Camp development*

This guide covers testing approaches, patterns, and best practices for ensuring code quality and simulation determinism in Goblin Camp.

## 🎯 Quick Navigation

- [Testing Philosophy](#testing-philosophy) - Our approach to testing
- [Test Categories](#test-categories) - Unit, integration, and determinism tests
- [ECS Testing Patterns](#ecs-testing-patterns) - Testing systems and components
- [Determinism Testing](#determinism-testing) - Ensuring reproducible behavior
- [Performance Testing](#performance-testing) - Benchmarking and optimization
- [Test Organization](#test-organization) - File structure and naming conventions

---

## 🎯 Testing Philosophy

### Core Principles

**Determinism First**: All tests must pass consistently across runs
**Fast Feedback**: Unit tests should run in milliseconds
**Integration Coverage**: Test real system interactions
**Regression Prevention**: Catch breaking changes early

### Testing Pyramid

```
    /\
   /  \     E2E Tests (Few)
  /____\    - Full simulation runs
 /      \   - Save/load validation
/________\  Integration Tests (Some)
           - Multi-system workflows
          - Job execution pipelines
         
          Unit Tests (Many)
          - Individual systems
          - Component behavior
          - Utility functions
```

---

## 📊 Test Categories

### Unit Tests

**Purpose**: Test individual functions and systems in isolation
**Location**: `crates/gc_core/tests/`
**Naming**: `*_tests.rs`

**Example Structure**:
```rust
#[cfg(test)]
mod mining_tests {
    use super::*;
    
    #[test]
    fn test_mine_designation_creates_job() {
        // Setup
        let mut world = World::new();
        setup_minimal_world(&mut world);
        
        // Action
        execute_system(&mut world, designation_to_jobs_system);
        
        // Assert
        assert_eq!(job_count(&world), 1);
    }
}
```

### Integration Tests

**Purpose**: Test interactions between multiple systems
**Location**: `crates/gc_core/tests/`
**Naming**: `*_lifecycle_tests.rs`, `*_integration_tests.rs`

**Example Test**:
```rust
#[test]
fn test_mine_to_haul_pipeline() {
    let mut world = World::new();
    setup_full_world(&mut world);
    
    // Create mine designation
    world.spawn((
        MineDesignation,
        Position(5, 5),
        DesignationLifecycle::active(),
    ));
    
    // Run mining systems
    run_systems(&mut world, &[
        designation_dedup_system,
        designation_to_jobs_system,
        job_assignment_system,
        mining_execution_system,
    ]);
    
    // Verify mining completed and haul job created
    assert_eq!(wall_count(&world), 9);  // 10 -> 9 walls
    assert_eq!(floor_count(&world), 2); // 1 -> 2 floors
    assert_eq!(item_count(&world), 1);  // Stone spawned
    
    // Run hauling systems
    run_systems(&mut world, &[
        auto_haul_system,
        job_assignment_system,
        hauling_execution_system,
    ]);
    
    // Verify item was hauled to stockpile
    let item_positions = get_item_positions(&world);
    let stockpile_positions = get_stockpile_positions(&world);
    assert!(item_positions.iter().any(|pos| stockpile_positions.contains(pos)));
}
```

### Determinism Tests

**Purpose**: Ensure identical inputs produce identical outputs
**Location**: `crates/gc_core/tests/determinism_tests.rs`

**Pattern**:
```rust
#[test]
fn test_mining_determinism() {
    let seed = 12345;
    let result1 = run_simulation_with_seed(seed, 100); // 100 ticks
    let result2 = run_simulation_with_seed(seed, 100);
    
    assert_eq!(result1.final_state, result2.final_state);
    assert_eq!(result1.job_history, result2.job_history);
    assert_eq!(result1.entity_positions, result2.entity_positions);
}

fn run_simulation_with_seed(seed: u64, ticks: u32) -> SimulationResult {
    let mut world = World::new();
    world.insert_resource(DeterministicRng::new(seed));
    setup_test_scenario(&mut world);
    
    let mut schedule = create_main_schedule();
    
    for _ in 0..ticks {
        schedule.run(&mut world);
    }
    
    extract_simulation_state(&world)
}
```

---

## ⚙️ ECS Testing Patterns

### Testing Individual Systems

**Setup Pattern**:
```rust
fn setup_test_world() -> World {
    let mut world = World::new();
    
    // Add required resources
    world.insert_resource(JobBoard::default());
    world.insert_resource(ActiveJobs::default());
    world.insert_resource(DeterministicRng::new(12345));
    world.insert_resource(create_test_map());
    
    world
}

fn execute_system<S>(world: &mut World, system: S) 
where
    S: IntoSystemConfigs<()>,
{
    let mut schedule = Schedule::default();
    schedule.add_systems(system);
    schedule.run(world);
}
```

**Query Testing**:
```rust
#[test]
fn test_miner_assignment() {
    let mut world = setup_test_world();
    
    // Spawn test entities
    let miner = world.spawn((
        Miner,
        AssignedJob(None),
        Position(0, 0),
    )).id();
    
    let carrier = world.spawn((
        Carrier,
        AssignedJob(None),
        Position(1, 1),
    )).id();
    
    // Add mining job
    let mut job_board = world.resource_mut::<JobBoard>();
    add_job(&mut job_board, JobKind::Mine { target: (5, 5) }, 1, &mut rng);
    
    // Run assignment system
    execute_system(&mut world, job_assignment_system);
    
    // Verify only miner got the mining job
    let miner_job = world.get::<AssignedJob>(miner).unwrap();
    let carrier_job = world.get::<AssignedJob>(carrier).unwrap();
    
    assert!(miner_job.0.is_some());
    assert!(carrier_job.0.is_none());
}
```

### Testing Component Interactions

**Multi-Component Tests**:
```rust
#[test]
fn test_inventory_and_position_sync() {
    let mut world = setup_test_world();
    
    // Carrier at (0, 0)
    let carrier = world.spawn((
        Carrier,
        Position(0, 0),
        Inventory(None),
    )).id();
    
    // Item at (5, 5)
    let item = world.spawn((
        Item::stone(),
        Position(5, 5),
        Carriable,
    )).id();
    
    // Create haul job
    let mut job_board = world.resource_mut::<JobBoard>();
    add_job(&mut job_board, JobKind::Haul { 
        from: (5, 5), 
        to: (10, 10) 
    }, 1, &mut rng);
    
    // Assign and execute job
    execute_system(&mut world, job_assignment_system);
    execute_system(&mut world, hauling_execution_system);
    
    // Verify carrier picked up item and both moved
    let carrier_pos = world.get::<Position>(carrier).unwrap();
    let carrier_inv = world.get::<Inventory>(carrier).unwrap();
    let item_pos = world.get::<Position>(item).unwrap();
    
    assert_eq!(carrier_pos.0, 5);
    assert_eq!(carrier_pos.1, 5);
    assert_eq!(carrier_inv.0, Some(item));
    assert_eq!(item_pos.0, 5);
    assert_eq!(item_pos.1, 5);
}
```

### Testing System Ordering

**Dependency Verification**:
```rust
#[test]
fn test_system_execution_order() {
    let mut world = setup_test_world();
    
    // Create designation
    world.spawn((
        MineDesignation,
        Position(5, 5),
        DesignationLifecycle::active(),
    ));
    
    // Run systems in correct order
    let mut schedule = Schedule::default();
    schedule.add_systems((
        designation_dedup_system,
        designation_to_jobs_system.after(designation_dedup_system),
        job_assignment_system.after(designation_to_jobs_system),
    ));
    
    schedule.run(&mut world);
    
    // Verify each stage completed
    assert_eq!(designation_count(&world), 0); // Consumed
    assert_eq!(job_count(&world), 1);         // Created
    assert_eq!(assigned_worker_count(&world), 1); // Assigned
}

#[test]
#[should_panic(expected = "System dependency violation")]
fn test_wrong_system_order_fails() {
    let mut world = setup_test_world();
    
    // Try to run in wrong order
    let mut schedule = Schedule::default();
    schedule.add_systems((
        job_assignment_system,  // This should fail
        designation_to_jobs_system.after(job_assignment_system),
    ));
    
    schedule.run(&mut world);
}
```

---

## 🔄 Determinism Testing

### State Verification

**Complete State Capture**:
```rust
#[derive(Debug, PartialEq)]
struct SimulationState {
    time_ticks: u32,
    entity_count: usize,
    job_board_state: String,
    map_hash: u64,
    entity_positions: BTreeMap<Entity, Position>,
    inventories: BTreeMap<Entity, Option<Entity>>,
}

fn capture_simulation_state(world: &World) -> SimulationState {
    let time = world.resource::<Time>();
    let job_board = world.resource::<JobBoard>();
    let map = world.resource::<GameMap>();
    
    let mut entity_positions = BTreeMap::new();
    let mut inventories = BTreeMap::new();
    
    // Capture all entity states
    for (entity, pos) in world.query::<&Position>().iter() {
        entity_positions.insert(entity, *pos);
    }
    
    for (entity, inv) in world.query::<&Inventory>().iter() {
        inventories.insert(entity, inv.0);
    }
    
    SimulationState {
        time_ticks: time.ticks,
        entity_count: world.entities().len(),
        job_board_state: format!("{:?}", job_board),
        map_hash: calculate_map_hash(map),
        entity_positions,
        inventories,
    }
}
```

**Random Number Testing**:
```rust
#[test]
fn test_rng_determinism() {
    let seed = 42;
    
    let sequence1 = generate_random_sequence(seed, 100);
    let sequence2 = generate_random_sequence(seed, 100);
    
    assert_eq!(sequence1, sequence2);
}

fn generate_random_sequence(seed: u64, count: usize) -> Vec<u32> {
    let mut rng = DeterministicRng::new(seed);
    (0..count).map(|_| rng.job_rng.next_u32()).collect()
}
```

### Cross-Platform Testing

**Environment Independence**:
```rust
#[test]
fn test_platform_determinism() {
    let test_data = vec![
        (12345, 100),
        (67890, 50),
        (11111, 200),
    ];
    
    for (seed, ticks) in test_data {
        let result = run_deterministic_test(seed, ticks);
        
        // Store results in platform-specific files
        let expected_file = format!("test_data/determinism_{}_{}.json", seed, ticks);
        
        if std::path::Path::new(&expected_file).exists() {
            let expected: SimulationState = load_expected_result(&expected_file);
            assert_eq!(result, expected);
        } else {
            save_expected_result(&expected_file, &result);
            println!("Created baseline for seed={}, ticks={}", seed, ticks);
        }
    }
}
```

---

## ⚡ Performance Testing

### Benchmarking Systems

**Using Criterion**:
```rust
// crates/gc_core/benches/systems.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_job_assignment(c: &mut Criterion) {
    let mut world = setup_large_world(); // 1000 workers, 100 jobs
    
    c.bench_function("job_assignment_1000_workers", |b| {
        b.iter(|| {
            execute_system(black_box(&mut world), job_assignment_system);
        });
    });
}

fn benchmark_hauling_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("hauling");
    
    for item_count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("hauling_execution", item_count),
            item_count,
            |b, &item_count| {
                let mut world = setup_world_with_items(item_count);
                b.iter(|| execute_system(&mut world, hauling_execution_system));
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_job_assignment, benchmark_hauling_execution);
criterion_main!(benches);
```

### Memory Usage Testing

**Memory Monitoring**:
```rust
#[test]
fn test_memory_growth() {
    let initial_memory = get_memory_usage();
    
    let mut world = setup_test_world();
    
    // Run simulation for extended period
    let mut schedule = create_main_schedule();
    for _ in 0..10000 {
        schedule.run(&mut world);
    }
    
    let final_memory = get_memory_usage();
    let growth = final_memory - initial_memory;
    
    // Ensure memory growth is reasonable
    assert!(growth < 100 * 1024 * 1024); // Less than 100MB growth
}

fn get_memory_usage() -> usize {
    // Platform-specific memory measurement
    // Use tools like `peak_alloc` or system monitoring
    0 // Placeholder
}
```

### Profiling Integration

**Performance Regression Detection**:
```rust
#[test]
fn test_performance_regression() {
    let baseline_times = load_baseline_performance();
    
    let mut current_times = HashMap::new();
    
    // Measure current performance
    for (test_name, test_fn) in get_performance_tests() {
        let start = std::time::Instant::now();
        test_fn();
        let duration = start.elapsed();
        current_times.insert(test_name, duration);
    }
    
    // Compare with baseline
    for (test_name, baseline_time) in baseline_times {
        let current_time = current_times[&test_name];
        let regression = current_time.as_secs_f64() / baseline_time.as_secs_f64();
        
        assert!(regression < 1.5, 
               "Performance regression in {}: {:.2}x slower", 
               test_name, regression);
    }
}
```

---

## 📁 Test Organization

### File Structure

```
crates/gc_core/tests/
├── m0_core_tests.rs           # Basic functionality
├── m2_job_execution_tests.rs  # Job system workflows
├── determinism_tests.rs       # Determinism validation
├── designation_lifecycle_tests.rs  # Designation processing
├── inventory_tests.rs         # Item management
├── mining_tests.rs           # Mining system tests
├── stockpile_tests.rs        # Stockpile behavior
├── visibility_and_cache_tests.rs  # FOV and caching
└── helpers/
    ├── mod.rs                # Test utilities
    ├── world_setup.rs        # World creation helpers
    ├── assertions.rs         # Custom assertions
    └── test_data.rs          # Test scenarios
```

### Test Naming Conventions

**Function Names**:
```rust
test_{system_name}_{behavior}_{condition}

// Examples:
test_job_assignment_assigns_mining_job_to_miner()
test_hauling_execution_moves_item_to_stockpile()
test_designation_dedup_ignores_duplicate_positions()
```

**Test Categories**:
```rust
// Unit tests
#[test]
fn test_single_system_behavior() {}

// Integration tests  
#[test]
fn test_multi_system_workflow() {}

// Determinism tests
#[test]
fn test_deterministic_behavior() {}

// Performance tests
#[test]
#[ignore] // Run with `cargo test -- --ignored`
fn test_performance_characteristics() {}
```

### Helper Functions

**Common Test Utilities**:
```rust
// helpers/world_setup.rs
pub fn setup_minimal_world() -> World {
    let mut world = World::new();
    world.insert_resource(Time::default());
    world.insert_resource(DeterministicRng::new(42));
    world
}

pub fn setup_full_world() -> World {
    let mut world = setup_minimal_world();
    world.insert_resource(JobBoard::default());
    world.insert_resource(ActiveJobs::default());
    world.insert_resource(create_test_map());
    add_test_entities(&mut world);
    world
}

pub fn create_test_map() -> GameMap {
    let mut map = GameMap::new();
    // Add walls, floors, stockpiles
    map
}

// helpers/assertions.rs
pub fn assert_job_completed(world: &World, job_id: JobId) {
    let job_board = world.resource::<JobBoard>();
    assert!(!job_board.jobs.contains_key(&job_id));
}

pub fn assert_entity_at_position(world: &World, entity: Entity, expected: Position) {
    let position = world.get::<Position>(entity).unwrap();
    assert_eq!(*position, expected);
}
```

---

## 🚀 Running Tests

### Test Commands

```bash
# Run all tests
cargo test

# Run specific test file
cargo test mining_tests

# Run with output
cargo test -- --nocapture

# Run determinism tests only
cargo test determinism

# Run performance tests (normally ignored)
cargo test -- --ignored

# Run benchmarks
cargo bench

# Generate test coverage
cargo tarpaulin --out Html
```

### Continuous Integration

**Test Pipeline**:
```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run unit tests
        run: cargo test --lib
      - name: Run integration tests
        run: cargo test --test '*'
      - name: Run determinism tests
        run: cargo test determinism -- --test-threads=1
      - name: Run benchmarks
        run: cargo bench
```

### Test Data Management

**Version-Controlled Test Data**:
```
tests/test_data/
├── determinism_baselines/     # Expected outputs for determinism tests
├── performance_baselines/     # Performance regression data
├── scenarios/                 # Predefined test scenarios
└── maps/                     # Test map configurations
```

---

## 📚 Related Documentation

- [Developer Guide](../README.md) - Getting started with development
- [ECS Architecture](../ecs-guide.md) - Understanding the component system
- [Job System](../job-system.md) - Job execution pipeline details
- [Performance Guide](../performance.md) - Optimization strategies

---

*This guide provides comprehensive testing approaches for Goblin Camp. Follow these patterns to maintain code quality and simulation reliability.*
