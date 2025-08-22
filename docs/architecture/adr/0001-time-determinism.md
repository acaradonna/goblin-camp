# ADR-0001: Time and Determinism Strategy

## Status

Accepted

## Date

2025-01-22

## Context

Goblin Camp aims to be a deterministic colony simulation where identical inputs (seed, player actions) produce identical outputs. This is critical for:

- Reproducible debugging and testing
- Replay systems and demos
- Multiplayer synchronization (future)
- Performance benchmarking consistency

Currently, the simulation uses Bevy ECS with ad-hoc system ordering and basic RNG seeding only for map generation. To achieve full determinism, we need explicit control over:

1. **Time Management**: How simulation time advances
2. **System Ordering**: When systems execute within each tick
3. **RNG Streams**: How random number generation is managed

## Decision

### Fixed-Step Tick Scheduling

We will implement a **fixed-step simulation tick** where:

- Each tick represents a fixed unit of simulation time (e.g., 1 game second)
- All systems run exactly once per tick in deterministic order
- No wall-clock time dependencies in simulation logic
- The simulation can run as fast or slow as hardware allows

```rust
// Pseudo-code structure
struct Time {
    tick: u64,          // Current simulation tick
    delta: Duration,    // Fixed time per tick (e.g., 1 second)
}

fn run_simulation_tick(world: &mut World, schedule: &mut Schedule) {
    // Increment time
    world.resource_mut::<Time>().tick += 1;
    
    // Run all systems in deterministic order
    schedule.run(world);
}
```

### System Execution Ordering

Systems will be organized into **ordered stages** within each tick:

1. **Input Processing**: Handle player commands, load designations
2. **World Updates**: Environment, map changes, object state
3. **AI Processing**: Job assignment, pathfinding, decision making  
4. **Physics/Movement**: Entity movement, collision resolution
5. **Output/Effects**: Sound, particles, UI updates (non-simulation)

```rust
// Pseudo-code system organization
schedule.add_systems((
    // Stage 1: Input Processing
    (load_designations_system, process_player_commands_system)
        .chain(),
    
    // Stage 2: World Updates  
    (mining_execution_system, item_spawning_system)
        .chain()
        .after(load_designations_system),
    
    // Stage 3: AI Processing
    (designation_to_jobs_system, job_assignment_system, pathfinding_system)
        .chain()
        .after(item_spawning_system),
        
    // Stage 4: Physics/Movement
    (movement_system, confine_to_map_system)
        .chain()
        .after(pathfinding_system),
));
```

### RNG Stream Management

We will use **separate RNG streams** for different subsystems to avoid cross-contamination:

```rust
#[derive(Resource)]
struct DeterministicRng {
    master_seed: u64,
    mapgen_rng: StdRng,      // For terrain generation
    job_rng: StdRng,         // For job selection randomness
    combat_rng: StdRng,      // For combat calculations (future)
    // ... additional streams as needed
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self {
            master_seed: seed,
            mapgen_rng: StdRng::seed_from_u64(seed),
            job_rng: StdRng::seed_from_u64(seed.wrapping_add(1)),
            combat_rng: StdRng::seed_from_u64(seed.wrapping_add(2)),
        }
    }
}
```

### Determinism Constraints

To maintain determinism, the following constraints must be observed:

**Forbidden:**
- `std::time::SystemTime` or wall-clock dependencies in simulation
- Floating-point operations that vary by platform/compiler
- Hash map iteration order dependencies
- Unordered parallel system execution
- Non-deterministic external I/O during simulation

**Required:**
- All random numbers sourced from `DeterministicRng`
- Explicit system ordering via `.chain()` or `.after()`
- Deterministic data structures (e.g., `BTreeMap` instead of `HashMap` for iteration)
- Fixed-point arithmetic where appropriate

## Implementation Plan

This ADR establishes the architectural foundation for these M1 milestone items:

1. **Time Resource + Schedule**: Implement `Time` resource and fixed-tick runner
2. **System Ordering**: Add explicit ordering annotations to existing systems  
3. **RNG Centralization**: Replace ad-hoc seeding with `DeterministicRng` resource
4. **Validation**: Add determinism tests that verify identical save states

## Consequences

### Positive

- **Reproducible Debugging**: Same seed = same behavior every time
- **Reliable Testing**: Simulation outcomes can be deterministically tested
- **Performance Benchmarking**: Consistent baseline for optimization work
- **Future-Proof**: Foundation for replay systems and multiplayer

### Negative

- **Implementation Complexity**: More rigid system organization required
- **Performance Overhead**: Some flexibility lost in system scheduling
- **Development Constraints**: Developers must be mindful of determinism rules

### Mitigation

- Provide clear guidelines and examples for deterministic system development
- Add automated tests to catch determinism violations
- Use linting/static analysis to detect problematic patterns

## References

- [Bevy ECS Scheduling Guide](https://docs.rs/bevy/latest/bevy/ecs/schedule/index.html)
- [RNG Best Practices for Game Development](https://www.pcg-random.org/)
- Dwarf Fortress Technical Details (inspiration for deterministic design)
- `docs/plan/MASTER_PLAN.md` - M1 Determinism + Lifecycle milestone