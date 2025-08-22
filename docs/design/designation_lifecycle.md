# Designation Lifecycle System

The designation lifecycle system provides state management for designations to prevent duplicate job creation and enable deterministic simulation behavior.

## Overview

When multiple designations are created at the same position (e.g., multiple `MineDesignation` entities at coordinates (5,5)), the system ensures only one job is created through a deduplication mechanism.

## Components

### DesignationState

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DesignationState {
    #[default]
    Active,   // Ready to be processed into jobs
    Ignored,  // Duplicate designation that should be skipped
    Consumed, // Processed designation (for future use)
}
```

### DesignationLifecycle

```rust
#[derive(Component, Debug, Default)]
pub struct DesignationLifecycle(pub DesignationState);
```

A Bevy ECS component wrapper around `DesignationState` that can be attached to designation entities.

## Systems

### designation_dedup_system

Runs **before** job creation to identify and mark duplicate designations:

1. **Collect Phase**: Gathers all `Active` designations by position using a HashMap
2. **Mark Phase**: For each position with multiple designations, marks all but the first as `Ignored`

**Algorithm**: O(n) where n is the number of active designations

### designation_to_jobs_system

Processes designations into jobs:

- Only considers designations in `Active` state
- Skips `Ignored` and `Consumed` designations
- Creates jobs through the existing `add_job()` function

## System Ordering

Critical for deterministic behavior:

```rust
schedule.add_systems((
    (
        designations::designation_dedup_system,
        designations::designation_to_jobs_system,
    ).chain(),
    jobs::job_assignment_system,
));
```

The `.chain()` ensures deduplication always runs before job creation within the same schedule tick.

## Usage Examples

### Creating Designations

```rust
// Both designations at same position
world.spawn((
    MineDesignation, 
    Position(5, 5), 
    DesignationLifecycle::default()  // Starts as Active
));

world.spawn((
    MineDesignation, 
    Position(5, 5), 
    DesignationLifecycle::default()  // Will become Ignored
));

// Different positions remain independent
world.spawn((
    MineDesignation, 
    Position(6, 5), 
    DesignationLifecycle::default()  // Stays Active
));
```

### Using DesignationBundle

```rust
world.spawn(DesignationBundle {
    pos: Position(10, 10),
    kind: MineDesignation,
    lifecycle: DesignationLifecycle::default(),  // Active by default
});
```

## Behavior Guarantees

1. **Deduplication**: Only one designation per position creates a job
2. **Determinism**: System ordering ensures consistent execution
3. **Backward Compatibility**: Existing code continues to work unchanged
4. **State Persistence**: Designation states survive across multiple system runs

## Future Extensions

The `Consumed` state is designed for future features:

- Removing processed designations from the world
- Tracking designation completion status
- Implementing designation cancellation workflows
- Supporting more complex designation types (areas, multi-step operations)

## Testing

Comprehensive integration tests validate:

- Single designations remain active
- Duplicate detection at same position
- Independence of different positions  
- Full pipeline behavior (dedup → job creation)
- State persistence across system runs

See `crates/gc_core/tests/designation_lifecycle_tests.rs` for complete test coverage.