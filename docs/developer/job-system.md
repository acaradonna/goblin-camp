# 🔧 Job System Deep Dive

> *Understanding the task assignment and execution pipeline in Goblin Camp*

The Job System is the central mechanism for coordinating work between entities in Goblin Camp. This guide covers the complete lifecycle from designation creation to job execution.

## 📖 Overview

The Job System follows a multi-stage pipeline:

1. **Designation Creation** - Player input or scripted commands
2. **Designation Processing** - Deduplication and lifecycle management  
3. **Job Generation** - Converting designations to executable tasks
4. **Job Assignment** - Matching jobs to capable workers
5. **Job Execution** - Workers performing the actual tasks
6. **Completion & Cleanup** - Finalizing results and state updates

## 🏗️ Core Architecture

### Job Board Resource

The central hub for all job management:

```rust
#[derive(Resource, Debug, Default)]
pub struct JobBoard {
    pub jobs: HashMap<JobId, Job>,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: JobId,
    pub kind: JobKind,
    pub priority: i32,
    pub assigned_to: Option<Entity>,
}
```

### Job Types

All possible work that can be assigned:

```rust
#[derive(Debug, Clone)]
pub enum JobKind {
    Mine { target: (i32, i32) },           // Convert Wall to Floor
    Haul { from: (i32, i32), to: (i32, i32) }, // Move item between locations
    // Future: Build, Craft, Defend, etc.
}
```

### Active Jobs Resource

Tracking currently executing jobs:

```rust
#[derive(Resource, Debug, Default)]
pub struct ActiveJobs {
    pub jobs: HashMap<JobId, Job>,
}
```

## 🎯 Designation System

### Designation Lifecycle

Designations progress through states to prevent duplicate processing:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignationState {
    Active,    // Ready to be processed into jobs
    Ignored,   // Duplicate designation that should be skipped  
    Consumed,  // Processed designation (for future use)
}

#[derive(Component, Debug)]
pub struct DesignationLifecycle {
    pub state: DesignationState,
}
```

### Designation Types

Current designation types:

```rust
#[derive(Component, Debug)]
pub struct MineDesignation;  // Mark tiles for mining

// Future: BuildDesignation, StockpileDesignation, etc.
```

### Designation Processing Pipeline

```rust
fn designation_dedup_system(
    mut designations: Query<&mut DesignationLifecycle, With<MineDesignation>>,
    positions: Query<&Position, With<MineDesignation>>,
) {
    // Group designations by position
    let mut position_map: HashMap<(i32, i32), Vec<Entity>> = HashMap::new();
    
    // Mark duplicates as Ignored
    for (entity, pos) in positions.iter() {
        position_map.entry((pos.0, pos.1)).or_default().push(entity);
    }
    
    // Keep first designation active, mark others as ignored
    for entities in position_map.values() {
        for &entity in entities.iter().skip(1) {
            if let Ok(mut lifecycle) = designations.get_mut(entity) {
                lifecycle.state = DesignationState::Ignored;
            }
        }
    }
}
```

## ⚙️ Job Generation

### Designation to Job Conversion

Converting player designations into executable jobs:

```rust
fn designation_to_jobs_system(
    mut commands: Commands,
    mut job_board: ResMut<JobBoard>,
    mut rng: ResMut<DeterministicRng>,
    designations: Query<
        (Entity, &Position, &DesignationLifecycle), 
        With<MineDesignation>
    >,
    map: Res<GameMap>,
) {
    for (entity, pos, lifecycle) in designations.iter() {
        // Only process Active designations
        if lifecycle.state != DesignationState::Active {
            continue;
        }
        
        // Validate target is mineable
        if map.get_tile(pos.0, pos.1) != Some(TileKind::Wall) {
            continue;
        }
        
        // Create mining job
        let job_id = add_job(
            &mut job_board,
            JobKind::Mine { target: (pos.0, pos.1) },
            1, // Priority
            &mut rng.job_rng,
        );
        
        // Remove processed designation
        commands.entity(entity).despawn();
    }
}
```

### Automatic Job Generation

Systems that create jobs without player input:

```rust
fn auto_haul_system(
    mut job_board: ResMut<JobBoard>,
    mut rng: ResMut<DeterministicRng>,
    q_items: Query<&Position, (With<Item>, Added<Item>)>, // New items only
    q_stockpiles: Query<&Position, With<Stockpile>>,
) {
    // Create haul jobs for newly spawned items
    for item_pos in q_items.iter() {
        if let Some(stockpile_pos) = find_nearest_stockpile(&q_stockpiles, item_pos) {
            add_job(
                &mut job_board,
                JobKind::Haul {
                    from: (item_pos.0, item_pos.1),
                    to: (stockpile_pos.0, stockpile_pos.1),
                },
                1,
                &mut rng.job_rng,
            );
        }
    }
}
```

## 👷 Job Assignment

### Worker Selection

Matching jobs to capable workers:

```rust
fn job_assignment_system(
    mut job_board: ResMut<JobBoard>,
    mut workers: Query<(Entity, &mut AssignedJob), 
                      (Or<(With<Miner>, With<Carrier>)>, Without<AssignedJob>)>,
    miners: Query<Entity, With<Miner>>,
    carriers: Query<Entity, With<Carrier>>,
) {
    // Find available workers (no current job assignment)
    let available_workers: Vec<Entity> = workers
        .iter()
        .filter(|(_, assigned)| assigned.0.is_none())
        .map(|(entity, _)| entity)
        .collect();
        
    // Assign jobs to workers based on capability
    for (job_id, job) in job_board.jobs.iter_mut() {
        if job.assigned_to.is_some() {
            continue; // Already assigned
        }
        
        // Find capable worker for this job type
        let capable_worker = match job.kind {
            JobKind::Mine { .. } => {
                available_workers.iter()
                    .find(|&&worker| miners.contains(worker))
                    .copied()
            },
            JobKind::Haul { .. } => {
                available_workers.iter()
                    .find(|&&worker| carriers.contains(worker))
                    .copied()
            },
        };
        
        if let Some(worker) = capable_worker {
            // Assign job to worker
            job.assigned_to = Some(worker);
            
            // Update worker's assignment
            if let Ok((_, mut assigned_job)) = workers.get_mut(worker) {
                assigned_job.0 = Some(*job_id);
            }
        }
    }
}
```

## 🏃 Job Execution

### Mining Job Execution

Converting wall tiles to floors and spawning items:

```rust
fn mining_execution_system(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    mut active_jobs: ResMut<ActiveJobs>,
    mut q_miners: Query<(&mut AssignedJob, &Position), With<Miner>>,
) {
    for (mut assigned_job, miner_pos) in q_miners.iter_mut() {
        if let Some(job_id) = assigned_job.0 {
            if let Some(job) = active_jobs.jobs.get(&job_id) {
                if let JobKind::Mine { target } = job.kind {
                    // Validate miner is at target location
                    if miner_pos.0 == target.0 && miner_pos.1 == target.1 {
                        // Perform mining operation
                        if map.get_tile(target.0, target.1) == Some(TileKind::Wall) {
                            // Convert wall to floor
                            map.set_tile(target.0, target.1, TileKind::Floor);
                            
                            // Spawn stone item at location
                            commands.spawn((
                                Item::stone(),
                                Position(target.0, target.1),
                                Carriable,
                                Stone,
                            ));
                            
                            // Complete job
                            active_jobs.jobs.remove(&job_id);
                            assigned_job.0 = None;
                        }
                    }
                }
            }
        }
    }
}
```

### Hauling Job Execution

Complex multi-phase system for item transportation:

```rust
fn hauling_execution_system(
    _commands: Commands,
    mut active_jobs: ResMut<ActiveJobs>,
    mut param_set: ParamSet<(
        Query<(&mut AssignedJob, &mut Inventory, &mut Position), 
              (With<Carrier>, Without<Miner>)>,
        Query<(Entity, &mut Position), (With<Item>, With<Carriable>)>,
    )>,
) {
    // Phase 1: Plan carrier movements and item pickups
    let mut carrier_updates = Vec::new();
    let mut item_updates = Vec::new();
    let mut completed_jobs = Vec::new();
    
    // Collect planned updates to avoid borrowing conflicts
    {
        let q_carriers = param_set.p0();
        for (assigned_job, inventory, carrier_pos) in q_carriers.iter() {
            if let Some(job_id) = assigned_job.0 {
                if let Some(job) = active_jobs.jobs.get(&job_id) {
                    if let JobKind::Haul { from, to } = job.kind {
                        if let Some(carried_item) = inventory.0 {
                            // Carrier has item - move to destination and drop
                            carrier_updates.push(CarrierUpdate {
                                job_id,
                                target: to,
                                dropping: true,
                                pickup_item: None,
                            });
                            
                            item_updates.push(ItemUpdate {
                                entity: carried_item,
                                target: to,
                            });
                            
                            completed_jobs.push(job_id);
                        } else {
                            // Carrier needs to pick up item first
                            carrier_updates.push(CarrierUpdate {
                                job_id,
                                target: from,
                                dropping: false,
                                pickup_item: None,
                            });
                        }
                    }
                }
            }
        }
    }
    
    // Phase 2: Find items to pick up
    {
        let q_items = param_set.p1();
        for carrier_update in &mut carrier_updates {
            if !carrier_update.dropping {
                let pickup_pos = carrier_update.target;
                for (item_entity, item_pos) in q_items.iter() {
                    if item_pos.0 == pickup_pos.0 && item_pos.1 == pickup_pos.1 {
                        carrier_update.pickup_item = Some(item_entity);
                        break;
                    }
                }
            }
        }
    }
    
    // Phase 3: Apply carrier updates
    {
        let mut q_carriers = param_set.p0();
        for (mut assigned_job, mut inventory, mut carrier_pos) in q_carriers.iter_mut() {
            if let Some(job_id) = assigned_job.0 {
                if let Some(update) = carrier_updates.iter()
                    .find(|u| u.job_id == job_id) {
                    
                    // Update carrier position
                    carrier_pos.0 = update.target.0;
                    carrier_pos.1 = update.target.1;
                    
                    if update.dropping {
                        // Drop item and complete job
                        inventory.0 = None;
                        assigned_job.0 = None;
                    } else if let Some(item_entity) = update.pickup_item {
                        // Pick up item
                        inventory.0 = Some(item_entity);
                    }
                }
            }
        }
    }
    
    // Phase 4: Apply item position updates
    {
        let mut q_items = param_set.p1();
        for item_update in item_updates {
            if let Ok((_, mut item_pos)) = q_items.get_mut(item_update.entity) {
                item_pos.0 = item_update.target.0;
                item_pos.1 = item_update.target.1;
            }
        }
    }
    
    // Phase 5: Remove completed jobs
    for job_id in completed_jobs {
        active_jobs.jobs.remove(&job_id);
    }
}
```

## 🔄 System Ordering

Critical execution order for deterministic behavior:

```rust
schedule.add_systems((
    // Phase 1: Designation processing
    designation_dedup_system,
    
    // Phase 2: Job generation  
    designation_to_jobs_system.after(designation_dedup_system),
    
    // Phase 3: Job assignment
    job_assignment_system.after(designation_to_jobs_system),
    
    // Phase 4: Job execution
    mining_execution_system.after(job_assignment_system),
    auto_haul_system.after(mining_execution_system),
    hauling_execution_system.after(auto_haul_system),
    
    // Phase 5: Time advancement
    advance_time.after(hauling_execution_system),
));
```

## 🎯 Performance Optimizations

### Job Board Efficiency

- Use `HashMap` for O(1) job lookup by ID
- Pre-allocate vectors for batch operations
- Minimize allocations during execution

### Worker Assignment Optimization

```rust
// Cache available workers to avoid repeated queries
let available_workers: Vec<Entity> = workers
    .iter()
    .filter(|(_, assigned)| assigned.0.is_none())
    .map(|(entity, _)| entity)
    .collect();

// Use capability-based worker pools
let miner_pool: HashSet<Entity> = miners.iter().collect();
let carrier_pool: HashSet<Entity> = carriers.iter().collect();
```

### Hauling System Efficiency

- Multi-phase execution to prevent borrowing conflicts
- Batch position updates to reduce system overhead
- Cache item-carrier relationships

## 🧪 Testing Job Systems

### Unit Tests

```rust
#[test]
fn test_job_assignment() {
    let mut world = World::new();
    world.insert_resource(JobBoard::default());
    
    // Create worker and job
    let worker = world.spawn((Miner, AssignedJob(None))).id();
    
    let mut job_board = world.resource_mut::<JobBoard>();
    add_job(&mut job_board, JobKind::Mine { target: (5, 5) }, 1, &mut rng);
    
    // Run assignment system
    let mut schedule = Schedule::default();
    schedule.add_systems(job_assignment_system);
    schedule.run(&mut world);
    
    // Verify assignment
    let assigned_job = world.get::<AssignedJob>(worker).unwrap();
    assert!(assigned_job.0.is_some());
}
```

### Integration Tests

```rust
#[test]
fn test_mining_to_haul_pipeline() {
    let mut world = World::new();
    
    // Setup: Map with wall, miner, stockpile
    world.insert_resource(GameMap::with_wall_at(5, 5));
    world.spawn((
        Miner, Carrier, 
        Position(5, 5), 
        AssignedJob(None), 
        Inventory(None)
    ));
    world.spawn(StockpileBundle::new(10, 10, 2, 2));
    
    // Create mining designation
    world.spawn((
        MineDesignation,
        Position(5, 5),
        DesignationLifecycle { state: DesignationState::Active },
    ));
    
    // Run full pipeline
    run_full_simulation_tick(&mut world);
    
    // Verify: Wall -> Floor, Item spawned, Haul job created
    assert_eq!(world.resource::<GameMap>().get_tile(5, 5), Some(TileKind::Floor));
    assert_eq!(item_count(&world), 1);
    assert_eq!(haul_job_count(&world), 1);
}
```

## 🔍 Debugging Job System

### Job State Inspection

```rust
fn debug_job_system(
    job_board: Res<JobBoard>,
    active_jobs: Res<ActiveJobs>,
    workers: Query<(Entity, &AssignedJob), Or<(With<Miner>, With<Carrier>)>>,
) {
    println!("=== Job Board Status ===");
    println!("Total jobs: {}", job_board.jobs.len());
    println!("Active jobs: {}", active_jobs.jobs.len());
    
    for (job_id, job) in &job_board.jobs {
        println!("Job {:?}: {:?} (assigned to {:?})", 
                job_id, job.kind, job.assigned_to);
    }
    
    println!("=== Worker Status ===");
    for (entity, assigned_job) in workers.iter() {
        println!("Worker {:?}: job={:?}", entity, assigned_job.0);
    }
}
```

### Performance Monitoring

```rust
fn job_performance_system(
    job_board: Res<JobBoard>,
    time: Res<Time>,
) {
    if time.ticks % 100 == 0 { // Every 100 ticks
        let unassigned_jobs = job_board.jobs.values()
            .filter(|job| job.assigned_to.is_none())
            .count();
            
        println!("Tick {}: {} unassigned jobs", time.ticks, unassigned_jobs);
    }
}
```

## 🚀 Future Enhancements

### Planned Features

1. **Job Priorities** - High-priority jobs assigned first
2. **Job Prerequisites** - Jobs that depend on other jobs
3. **Job Cancellation** - Remove invalid or unnecessary jobs
4. **Worker Specialization** - Skill levels and efficiency modifiers
5. **Path Validation** - Pre-check pathfinding before assignment

### Advanced Job Types

```rust
// Future job kinds
enum JobKind {
    // Current
    Mine { target: (i32, i32) },
    Haul { from: (i32, i32), to: (i32, i32) },
    
    // Planned
    Build { blueprint: BuildingType, target: (i32, i32) },
    Craft { recipe: RecipeId, workstation: Entity },
    Defend { target: Entity, duration: u32 },
    Research { project: ResearchId },
}
```

## 📚 Related Documentation

- [ECS Architecture Guide](./ecs-guide.md) - Understanding component patterns
- [Systems Reference](./api/systems.md) - Complete system documentation
- [Performance Guide](./performance.md) - Optimization techniques
- [Testing Guide](./testing.md) - Comprehensive testing strategies

---

*The Job System is central to Goblin Camp's gameplay. Understanding its architecture is essential for contributing to core systems and adding new features.*
