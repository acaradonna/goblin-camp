# Designation System Design

The designation system provides a declarative interface for players to specify work that needs to be done in the colony. Designations are ECS entities that represent intended actions (like mining a tile) and are converted into executable jobs through a lifecycle management system.

## Overview

The designation system is the primary interface between player intent and job execution. Players create designations by placing markers in the world (e.g., "mine this tile"), and the system automatically converts these into jobs that agents can execute.

Key design principles:
- **Declarative**: Players specify *what* to do, not *how* to do it
- **Deterministic**: Same designations always produce identical job sequences
- **Deduplication**: Multiple designations at the same position are automatically merged
- **Lifecycle Management**: Clear state transitions prevent duplicate work

## System Architecture

### Core Components

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Designation   │───▶│ DesignationState │───▶│      Job        │
│   (Intent)      │    │   (Lifecycle)    │    │  (Execution)    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
        │                       │                       │
        │                       │                       │
    Position(x,y)          Active/Ignored/          JobKind::Mine
  MineDesignation            Consumed               {x, y}
```

### State Machine

```
                Initial Creation
                       │
                       ▼
    ┌─────────────────────────────────────┐
    │            ACTIVE                   │◄──── Default state
    │    (Ready for processing)           │
    └─────────────────┬───────────────────┘
                      │
          ┌───────────┼───────────┐
          │           │           │
    Same Position     │      Job Creation
    Detected          │      Triggered
          │           │           │
          ▼           │           ▼
    ┌─────────────┐   │   ┌─────────────┐
    │   IGNORED   │   │   │  CONSUMED   │
    │ (Duplicate) │   │   │ (Processed) │
    └─────────────┘   │   └─────────────┘
                      │
                      ▼
             ┌─────────────────┐
             │ No State Change │
             │ (Single Active) │
             └─────────────────┘
```

### System Flow

```
Tick N: Designation Creation
│
├─ Player creates MineDesignation(5,5) → State: Active
├─ Player creates MineDesignation(5,5) → State: Active  
└─ Player creates MineDesignation(6,6) → State: Active

                          │
                          ▼

Tick N+1: Deduplication Phase (designation_dedup_system)
│
├─ Scan all Active designations by position
├─ Position (5,5): 2 designations found
│  ├─ Keep first designation as Active
│  └─ Mark second designation as Ignored
└─ Position (6,6): 1 designation → remains Active

                          │
                          ▼

Tick N+1: Job Creation Phase (designation_to_jobs_system)  
│
├─ Process only Active designations
├─ Create JobKind::Mine{x:5, y:5} → Mark designation Consumed
├─ Skip Ignored designation at (5,5)
└─ Create JobKind::Mine{x:6, y:6} → Mark designation Consumed

                          │
                          ▼

Result: 2 jobs created, no duplicates
```

## Implementation Details

### Types and Components

#### DesignationState Enum
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DesignationState {
    #[default]
    Active,   // Ready to be processed into jobs
    Ignored,  // Duplicate designation that should be skipped  
    Consumed, // Processed designation (for future cleanup)
}
```

#### DesignationLifecycle Component
```rust
#[derive(Component, Debug, Default)]
pub struct DesignationLifecycle(pub DesignationState);
```

Bevy ECS component that tracks the lifecycle state of any designation entity.

#### Designation Types
```rust
#[derive(Component, Debug)]
pub struct MineDesignation;

#[derive(Bundle)]
pub struct DesignationBundle {
    pub pos: Position,
    pub kind: MineDesignation, 
    pub lifecycle: DesignationLifecycle,
}
```

Currently supports mining designations. Future designation types (Build, Chop, etc.) will follow the same pattern.

### System Implementation

#### designation_dedup_system
- **Purpose**: Prevent duplicate jobs by marking redundant designations
- **Algorithm**: O(n) HashMap-based position grouping
- **Timing**: Runs before job creation in the same tick
- **Behavior**: 
  - Scans all Active designations
  - Groups by position coordinates  
  - Marks all but the first at each position as Ignored

#### designation_to_jobs_system  
- **Purpose**: Convert Active designations into executable jobs
- **Dependencies**: Requires DesignationConfig.auto_jobs = true
- **Timing**: Runs after deduplication in the same tick
- **Behavior**:
  - Processes only Active designations
  - Creates corresponding Job entities via add_job()
  - Marks processed designations as Consumed

### Configuration

#### DesignationConfig Resource
```rust
#[derive(Resource, Default, Debug)]
pub struct DesignationConfig {
    pub auto_jobs: bool,  // Enable/disable automatic job creation
}
```

Controls whether designations automatically create jobs. Useful for testing and pausing job creation.

## Deduplication Rules

### Position-Based Deduplication
- **Rule**: Only one designation per (x, y) coordinate can be Active
- **Detection**: HashMap grouping by position tuple
- **Resolution**: First designation stays Active, others become Ignored
- **Scope**: Applies per designation type (future: MineDesignation vs BuildDesignation may coexist)

### Deterministic Ordering
- **Requirement**: Same input always produces same result
- **Implementation**: Entity iteration order is deterministic in Bevy ECS
- **Guarantee**: First entity at position always wins, regardless of creation order
- **Testing**: Integration tests verify consistent behavior across runs

## Consumption Semantics

### State Transitions
1. **Active → Consumed**: When designation creates a job
2. **Active → Ignored**: When duplicate is detected  
3. **Ignored → Ignored**: Ignored designations remain ignored
4. **Consumed → Consumed**: Consumed designations remain consumed

### Job Creation Rules
- Only Active designations create jobs
- Ignored designations are skipped entirely
- Consumed designations are skipped (already processed)
- One designation creates exactly one job

### Future Cleanup
The Consumed state enables future features:
- Automatic removal of processed designations
- Designation completion tracking
- Cancellation workflows
- Multi-step designation operations

## System Ordering Requirements

### Critical Dependencies
```rust
schedule.add_systems((
    (
        designations::designation_dedup_system,
        designations::designation_to_jobs_system,
    ).chain(),
    jobs::job_assignment_system,
));
```

### Ordering Guarantees
- **Deduplication BEFORE job creation**: Prevents duplicate jobs
- **Job creation BEFORE assignment**: Ensures jobs exist before assignment
- **Within-tick consistency**: All designation processing completes before job assignment

## Usage Patterns

### Basic Designation Creation
```rust
// Single designation
world.spawn((
    MineDesignation,
    Position(10, 15),
    DesignationLifecycle::default(), // Starts as Active
));

// Using bundle for convenience
world.spawn(DesignationBundle {
    pos: Position(10, 15),
    kind: MineDesignation,
    lifecycle: DesignationLifecycle::default(),
});
```

### Handling Duplicates
```rust
// Both designations at same position - only first creates job
world.spawn((MineDesignation, Position(5, 5), DesignationLifecycle::default()));
world.spawn((MineDesignation, Position(5, 5), DesignationLifecycle::default()));

// After dedup system runs:
// - First designation: Active → creates job → Consumed  
// - Second designation: Active → Ignored → stays Ignored
```

### Querying States
```rust
// Count designations by state
let mut active_count = 0;
let mut ignored_count = 0;
let mut consumed_count = 0;

for lifecycle in query.iter() {
    match lifecycle.0 {
        DesignationState::Active => active_count += 1,
        DesignationState::Ignored => ignored_count += 1,
        DesignationState::Consumed => consumed_count += 1,
    }
}
```

## Acceptance Criteria

### AC1: Deduplication Behavior
- **Given**: Multiple MineDesignation entities at the same position
- **When**: designation_dedup_system executes
- **Then**: Only the first designation remains Active, others become Ignored
- **Verification**: Integration test `duplicate_designations_marked_ignored`

### AC2: Single Designation Stability  
- **Given**: A single MineDesignation at a unique position
- **When**: designation_dedup_system executes
- **Then**: The designation remains in Active state
- **Verification**: Integration test `single_designation_remains_active`

### AC3: Position Independence
- **Given**: MineDesignation entities at different positions  
- **When**: designation_dedup_system executes
- **Then**: All designations remain Active (no interference)
- **Verification**: Integration test `different_positions_remain_active`

### AC4: Job Creation from Active Only
- **Given**: Designations in Active, Ignored, and Consumed states
- **When**: designation_to_jobs_system executes with auto_jobs=true
- **Then**: Only Active designations create jobs and become Consumed
- **Verification**: Integration test `only_active_designations_create_jobs`

### AC5: State Persistence
- **Given**: Designations marked as Ignored
- **When**: designation_dedup_system runs again
- **Then**: Ignored designations remain Ignored (no state reversion)
- **Verification**: Integration test `ignored_designations_stay_ignored`

### AC6: Full Pipeline Integration
- **Given**: Multiple designations with some duplicates
- **When**: Both dedup and job creation systems run in sequence
- **Then**: Exactly one job per unique position is created
- **Verification**: Integration test `full_pipeline_dedup_then_jobs`

### AC7: Configuration Respect
- **Given**: DesignationConfig.auto_jobs = false
- **When**: designation_to_jobs_system executes
- **Then**: No jobs are created regardless of Active designations
- **Verification**: Implicit in all tests using auto_jobs=false for isolated testing

### AC8: Deterministic Execution
- **Given**: Identical designation setups across multiple test runs
- **When**: Systems execute with same ordering
- **Then**: Results are identical (same entities marked Ignored/Consumed)
- **Verification**: System ordering via .chain() ensures deterministic execution

## Future Extensions

### Additional Designation Types
```rust
#[derive(Component, Debug)]
pub struct BuildDesignation { pub blueprint: BuildingType }

#[derive(Component, Debug)]  
pub struct ChopDesignation;

#[derive(Component, Debug)]
pub struct HaulDesignation { pub item: Entity, pub destination: Position }
```

### Area Designations
Support for multi-tile operations:
```rust
#[derive(Component, Debug)]
pub struct AreaDesignation {
    pub top_left: Position,
    pub bottom_right: Position,
    pub operation: DesignationType,
}
```

### Priority System
```rust
#[derive(Component, Debug)]
pub struct DesignationPriority(pub u8); // 1-9 priority levels
```

### Cancellation Support
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignationState {
    Active,
    Ignored, 
    Consumed,
    Cancelled, // New state for user cancellation
}
```

## Performance Considerations

### Algorithmic Complexity
- **Deduplication**: O(n) where n = number of Active designations
- **Memory**: O(p) where p = number of unique positions with designations
- **Scalability**: Efficient for typical colony sizes (thousands of designations)

### Optimization Opportunities
- **Spatial Indexing**: For large maps, could use grid-based spatial partitioning
- **Batch Processing**: Group designations by type for more efficient processing
- **Incremental Updates**: Only process new/changed designations rather than full scan

### Memory Management
- **Entity Cleanup**: Consider periodic removal of Consumed designations
- **Component Pooling**: Reuse DesignationLifecycle components for frequently created/destroyed designations

## Testing Strategy

### Integration Test Coverage
- **designation_lifecycle_tests.rs**: 7 comprehensive integration tests
- **State Transitions**: All valid state changes are tested
- **Edge Cases**: Single designations, duplicates, different positions
- **Pipeline Integration**: End-to-end designation → job creation flow

### Test Philosophy
- **Deterministic**: Tests use consistent entity IDs and ordering
- **Isolated**: Each test focuses on specific behavior
- **Comprehensive**: All acceptance criteria have corresponding tests
- **Maintainable**: Tests use clear assertions and descriptive names

### Validation Commands
```bash
# Run all designation tests
cargo test designation_lifecycle

# Run full validation pipeline  
./dev.sh check

# Run specific test categories
cargo test -- designation
```

## Related Documentation

- **Implementation Details**: `docs/design/designation_lifecycle.md`
- **Job System Integration**: `docs/design/ai_jobs.md`  
- **ECS Architecture**: `docs/architecture/`
- **Test Coverage**: `crates/gc_core/tests/designation_lifecycle_tests.rs`

## Change History

- **Initial Design**: Designation lifecycle system with state management
- **Deduplication**: Added position-based duplicate detection
- **Job Integration**: Connected designations to job creation pipeline
- **Testing**: Comprehensive integration test suite