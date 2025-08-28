# ⚙️ Systems API Reference

> *Complete reference for all ECS systems in Goblin Camp*

This document provides detailed documentation for every system in the Goblin Camp simulation, including their responsibilities, parameters, and execution order requirements.

## 🎯 Quick Navigation

- [Core Simulation Systems](#core-simulation-systems) - Time, movement, and basic operations
- [Job Management Systems](#job-management-systems) - Task assignment and execution
- [Designation Systems](#designation-systems) - Player input processing
- [Spatial Systems](#spatial-systems) - Movement, pathfinding, and FOV
- [Item Systems](#item-systems) - Resource creation and management
- [System Execution Order](#system-execution-order) - Critical ordering requirements

---

## ⏰ Core Simulation Systems

### `advance_time`

```rust
pub fn advance_time(mut time: ResMut<Time>)
```

**Purpose**: Advances the simulation clock by one tick
**Phase**: Late simulation (after all other systems)
**Frequency**: Every frame
**Dependencies**: Should run last in the schedule

**Parameters**:
- `time: ResMut<Time>` - Mutable access to global time resource

**Behavior**:
- Increments `time.ticks` by 1
- Provides deterministic time progression
- Used by other systems for timing-based behavior

**Example Usage**:
```rust
fn check_time_system(time: Res<Time>) {
    if time.ticks % 100 == 0 {
        println!("100 ticks have passed");
    }
}
```

**System Ordering**:
```rust
schedule.add_systems(advance_time.after(all_other_systems));
```

---

## 🔧 Job Management Systems

### `job_assignment_system`

```rust
pub fn job_assignment_system(
    mut job_board: ResMut<JobBoard>,
    mut available_workers: Query<(Entity, &mut AssignedJob), 
                                Or<(With<Miner>, With<Carrier>)>>,
    miners: Query<(), With<Miner>>,
    carriers: Query<(), With<Carrier>>,
)
```

**Purpose**: Assigns available jobs to capable workers
**Phase**: Job processing phase
**Frequency**: Every tick
**Dependencies**: Must run after job creation systems

**Parameters**:
- `job_board: ResMut<JobBoard>` - Global job repository
- `available_workers: Query<...>` - Workers without current assignments
- `miners: Query<(), With<Miner>>` - Mining-capable entities
- `carriers: Query<(), With<Carrier>>` - Hauling-capable entities

**Algorithm**:
1. Find workers with `AssignedJob(None)`
2. Match job types to worker capabilities:
   - `JobKind::Mine` → Workers with `Miner` component
   - `JobKind::Haul` → Workers with `Carrier` component
3. Assign job ID to worker's `AssignedJob` component
4. Mark job as assigned in job board

**Example Job Assignment**:
```rust
// Worker becomes assigned
AssignedJob(None) → AssignedJob(Some(job_id))

// Job gets assigned
Job { assigned_to: None } → Job { assigned_to: Some(worker_entity) }
```

### `mining_execution_system`

```rust
pub fn mining_execution_system(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    mut active_jobs: ResMut<ActiveJobs>,
    mut q_miners: Query<(&mut AssignedJob, &Position), With<Miner>>,
)
```

**Purpose**: Executes mining jobs to convert walls to floors and spawn items
**Phase**: Job execution phase
**Frequency**: Every tick
**Dependencies**: Must run after `job_assignment_system`

**Parameters**:
- `commands: Commands` - For spawning item entities
- `map: ResMut<GameMap>` - For tile modification
- `active_jobs: ResMut<ActiveJobs>` - Currently executing jobs
- `q_miners: Query<...>` - Miners with assigned jobs

**Execution Flow**:
1. For each miner with an assigned mining job:
2. Verify miner is at target location
3. Check target tile is `TileKind::Wall`
4. Convert wall to `TileKind::Floor`
5. Spawn stone item at location
6. Remove job from active jobs
7. Clear worker's job assignment

**Item Spawning**:
```rust
commands.spawn((
    Item::stone(),
    Position(target.0, target.1),
    Carriable,
    Stone,
));
```

### `hauling_execution_system`

```rust
pub fn hauling_execution_system(
    _commands: Commands,
    mut active_jobs: ResMut<ActiveJobs>,
    mut param_set: ParamSet<(
        Query<(&mut AssignedJob, &mut Inventory, &mut Position), 
              (With<Carrier>, Without<Miner>)>,
        Query<(Entity, &mut Position), (With<Item>, With<Carriable>)>,
    )>,
)
```

**Purpose**: Executes hauling jobs to transport items between locations
**Phase**: Job execution phase
**Frequency**: Every tick
**Dependencies**: Must run after `auto_haul_system`

**Complex Multi-Phase Algorithm**:

**Phase 1**: Plan carrier movements and item interactions
- Examine all carriers with haul jobs
- Determine if carrying item (drop phase) or need pickup
- Plan position updates and inventory changes

**Phase 2**: Locate items for pickup
- Find items at pickup locations
- Match carriers needing items with available items

**Phase 3**: Execute carrier updates
- Move carriers to target positions
- Update inventories (pickup/drop items)
- Complete jobs when items are delivered

**Phase 4**: Update item positions
- Move carried items to destination positions
- Synchronize item and carrier locations

**Multi-Tick Execution**:
```rust
// Tick 1: Carrier moves to item, picks up
Inventory(None) + Position(start) → Inventory(Some(item)) + Position(pickup)

// Tick 2: Carrier moves to destination, drops item
Inventory(Some(item)) + Position(pickup) → Inventory(None) + Position(destination)
```

### `auto_haul_system`

```rust
pub fn auto_haul_system(
    mut job_board: ResMut<JobBoard>,
    mut rng: ResMut<DeterministicRng>,
    q_items: Query<&Position, (With<Item>, Added<Item>)>,
    q_stockpiles: Query<&Position, With<Stockpile>>,
)
```

**Purpose**: Automatically creates hauling jobs for newly spawned items
**Phase**: Job generation phase
**Frequency**: Every tick
**Dependencies**: Must run after item spawning systems

**Parameters**:
- `job_board: ResMut<JobBoard>` - For creating new jobs
- `rng: ResMut<DeterministicRng>` - For deterministic job IDs
- `q_items: Query<..., Added<Item>>` - Items created this tick only
- `q_stockpiles: Query<...>` - Available storage locations

**Logic**:
1. Process only items with `Added<Item>` filter
2. Find nearest stockpile using Euclidean distance
3. Create `JobKind::Haul` from item position to stockpile
4. Add job to job board with priority 1

**Nearest Stockpile Algorithm**:
```rust
fn find_nearest_stockpile(
    stockpiles: &Query<&Position, With<Stockpile>>,
    item_pos: &Position,
) -> Option<Position> {
    stockpiles.iter()
        .min_by_key(|stockpile_pos| {
            let dx = item_pos.0 - stockpile_pos.0;
            let dy = item_pos.1 - stockpile_pos.1;
            dx * dx + dy * dy  // Squared distance
        })
        .copied()
}
```

---

## 📋 Designation Systems

### `designation_dedup_system`

```rust
pub fn designation_dedup_system(
    mut designations: Query<&mut DesignationLifecycle, With<MineDesignation>>,
    positions: Query<&Position, With<MineDesignation>>,
)
```

**Purpose**: Prevents duplicate jobs by marking overlapping designations as ignored
**Phase**: Early designation processing
**Frequency**: Every tick
**Dependencies**: Must run before `designation_to_jobs_system`

**Algorithm**:
1. Group all mine designations by position
2. For each position with multiple designations:
   - Keep first designation as `Active`
   - Mark remaining as `Ignored`
3. Ignored designations are skipped by job generation

**Deduplication Logic**:
```rust
// Before deduplication
Position(5,5) → [Designation1(Active), Designation2(Active), Designation3(Active)]

// After deduplication  
Position(5,5) → [Designation1(Active), Designation2(Ignored), Designation3(Ignored)]
```

### `designation_to_jobs_system`

```rust
pub fn designation_to_jobs_system(
    mut commands: Commands,
    mut job_board: ResMut<JobBoard>,
    mut rng: ResMut<DeterministicRng>,
    designations: Query<(Entity, &Position, &DesignationLifecycle), With<MineDesignation>>,
    map: Res<GameMap>,
)
```

**Purpose**: Converts active player designations into executable jobs
**Phase**: Job generation phase
**Frequency**: Every tick
**Dependencies**: Must run after `designation_dedup_system`

**Parameters**:
- `commands: Commands` - For despawning processed designations
- `job_board: ResMut<JobBoard>` - For creating new jobs
- `rng: ResMut<DeterministicRng>` - For deterministic job IDs
- `designations: Query<...>` - All mine designations with lifecycle state
- `map: Res<GameMap>` - For validating mining targets

**Processing Logic**:
1. Filter designations with `DesignationState::Active`
2. Validate target position contains `TileKind::Wall`
3. Create `JobKind::Mine` job with target position
4. Add job to job board
5. Despawn processed designation entity

**Job Creation**:
```rust
let job_id = add_job(
    &mut job_board,
    JobKind::Mine { target: (pos.0, pos.1) },
    1, // Priority
    &mut rng.job_rng,
);
```

---

## 🎯 Spatial Systems

### `movement`

```rust
pub fn movement(mut positions: Query<&mut Position>)
```

**Purpose**: Basic movement system template (currently minimal implementation)
**Phase**: Movement phase
**Frequency**: Every tick
**Dependencies**: No specific dependencies

**Current Implementation**: Placeholder for future pathfinding-based movement

**Future Enhancements**:
- Pathfinding integration for goal-directed movement
- Collision detection and avoidance
- Speed modifiers and movement costs
- Animation and interpolation support

---

## 🏗️ System Execution Order

### Critical Ordering Requirements

The simulation requires specific system execution order for deterministic behavior:

```rust
schedule.add_systems((
    // Phase 1: Input Processing
    designation_dedup_system,
    
    // Phase 2: Job Generation
    designation_to_jobs_system.after(designation_dedup_system),
    
    // Phase 3: Job Assignment  
    job_assignment_system.after(designation_to_jobs_system),
    
    // Phase 4: Job Execution
    mining_execution_system.after(job_assignment_system),
    auto_haul_system.after(mining_execution_system),
    hauling_execution_system.after(auto_haul_system),
    
    // Phase 5: Spatial Updates
    movement.after(hauling_execution_system),
    
    // Phase 6: Time Advancement
    advance_time.after(movement),
));
```

### Phase Explanations

**Phase 1 - Input Processing**:
- `designation_dedup_system`: Prevent duplicate designations
- Ensures clean input state for job generation

**Phase 2 - Job Generation**:
- `designation_to_jobs_system`: Convert designations to jobs
- Must run after deduplication to avoid duplicate jobs

**Phase 3 - Job Assignment**:
- `job_assignment_system`: Assign jobs to workers
- Requires jobs to exist before assignment

**Phase 4 - Job Execution**:
- `mining_execution_system`: Execute mining jobs, spawn items
- `auto_haul_system`: Create haul jobs for new items
- `hauling_execution_system`: Execute hauling jobs
- Order ensures items exist before hauling jobs are created

**Phase 5 - Spatial Updates**:
- `movement`: Update entity positions
- Future: pathfinding, collision detection

**Phase 6 - Time Advancement**:
- `advance_time`: Increment simulation clock
- Must run last to ensure all frame processing is complete

### Determinism Requirements

**Fixed Execution Order**: Systems must run in the same order every tick
**No Parallel Execution**: Systems that modify the same components cannot run in parallel
**Seeded Randomness**: All random operations use `DeterministicRng`

---

## 🧪 System Testing Patterns

### Unit Testing Individual Systems

```rust
#[test]
fn test_job_assignment_system() {
    let mut world = World::new();
    world.insert_resource(JobBoard::default());
    
    // Create worker and job
    let worker = world.spawn((Miner, AssignedJob(None))).id();
    let mut job_board = world.resource_mut::<JobBoard>();
    add_job(&mut job_board, JobKind::Mine { target: (5, 5) }, 1, &mut rng);
    
    // Run system
    let mut schedule = Schedule::default();
    schedule.add_systems(job_assignment_system);
    schedule.run(&mut world);
    
    // Verify assignment
    let assigned_job = world.get::<AssignedJob>(worker).unwrap();
    assert!(assigned_job.0.is_some());
}
```

### Integration Testing System Chains

```rust
#[test]
fn test_designation_to_execution_pipeline() {
    let mut world = World::new();
    setup_test_world(&mut world);
    
    // Create designation
    world.spawn((
        MineDesignation,
        Position(5, 5),
        DesignationLifecycle { state: DesignationState::Active },
    ));
    
    // Run full pipeline
    let mut schedule = Schedule::default();
    schedule.add_systems((
        designation_dedup_system,
        designation_to_jobs_system.after(designation_dedup_system),
        job_assignment_system.after(designation_to_jobs_system),
        mining_execution_system.after(job_assignment_system),
    ));
    
    schedule.run(&mut world);
    
    // Verify results
    assert_eq!(designation_count(&world), 0); // Designation consumed
    assert_eq!(job_count(&world), 0);         // Job completed
    assert_eq!(item_count(&world), 1);        // Item spawned
}
```

## 🔍 System Debugging

### Performance Monitoring

```rust
fn debug_system_performance(
    time: Res<Time>,
    job_board: Res<JobBoard>,
    workers: Query<&AssignedJob>,
) {
    if time.ticks % 100 == 0 {
        let total_jobs = job_board.jobs.len();
        let assigned_workers = workers.iter()
            .filter(|job| job.0.is_some())
            .count();
            
        println!("Tick {}: {} jobs, {} workers assigned", 
                time.ticks, total_jobs, assigned_workers);
    }
}
```

### State Validation

```rust
fn validate_system_state(
    job_board: Res<JobBoard>,
    active_jobs: Res<ActiveJobs>,
    workers: Query<&AssignedJob>,
) {
    // Verify job board consistency
    for (job_id, job) in &job_board.jobs {
        if let Some(worker) = job.assigned_to {
            let worker_assignment = workers.get(worker).unwrap();
            assert_eq!(worker_assignment.0, Some(*job_id));
        }
    }
    
    // Verify active jobs are subset of job board
    for active_job_id in active_jobs.jobs.keys() {
        assert!(job_board.jobs.contains_key(active_job_id));
    }
}
```

## 📚 Related Documentation

- [ECS Architecture Guide](../ecs-guide.md) - Understanding ECS patterns
- [Job System Deep Dive](../job-system.md) - Detailed job system documentation
- [Components Reference](./components.md) - Components used by these systems
- [Performance Guide](../performance.md) - System optimization techniques

---

*This reference covers all systems in the current codebase. For component-specific information, see the [Components Reference](./components.md).*
