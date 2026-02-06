use crate::components::*;
use crate::jobs::*;
use crate::world::*;
use bevy_ecs::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;

/// Core Systems for Goblin Camp Simulation
///
/// This module contains the main ECS systems that drive the simulation.
/// Systems handle movement, job execution, time management, and other
/// core gameplay mechanics. All systems are designed for deterministic
/// execution using fixed-step timing and seeded RNG.
/// Fixed-step time resource for deterministic ticks
/// Provides frame-rate independent timing for reproducible simulation
/// All game logic should use tick count rather than wall-clock time
#[derive(Resource, Debug, Clone, Copy)]
pub struct Time {
    /// Accumulated tick count since simulation start
    /// Each tick represents one fixed simulation step
    pub ticks: u64,
    /// Duration of a tick in milliseconds (for reference/logging only)
    /// Actual timing is handled by the scheduler, not this value
    pub tick_ms: u64,
}

impl Time {
    /// Create a new time resource with specified tick duration
    /// The tick_ms value is for reference only; actual timing depends on scheduler
    pub fn new(tick_ms: u64) -> Self {
        Self { ticks: 0, tick_ms }
    }
}

/// Centralized deterministic RNG resource with separate streams per subsystem
/// Ensures reproducible simulation by providing seeded RNG streams
/// Each subsystem gets its own stream to avoid cross-contamination
#[derive(Resource, Debug)]
pub struct DeterministicRng {
    /// Master seed for reproducibility - can be used to recreate entire simulation
    pub master_seed: u64,
    /// RNG stream for terrain generation and map creation
    pub mapgen_rng: StdRng,
    /// RNG stream for job selection and UUID generation
    pub job_rng: StdRng,
    /// RNG stream for combat calculations (future use)
    pub combat_rng: StdRng,
    /// RNG stream for pathfinding randomization (future use)
    pub pathfinding_rng: StdRng,
}

impl DeterministicRng {
    /// Create new deterministic RNG with separate streams from master seed
    /// Uses different offsets to ensure each stream is independent
    /// All streams derived from the same master seed for reproducibility
    pub fn new(seed: u64) -> Self {
        Self {
            master_seed: seed,
            // Use different multipliers and offsets to create independent streams
            mapgen_rng: StdRng::seed_from_u64(seed.wrapping_mul(0x9e3779b9).wrapping_add(0)),
            job_rng: StdRng::seed_from_u64(seed.wrapping_mul(0x9e3779b9).wrapping_add(1)),
            combat_rng: StdRng::seed_from_u64(seed.wrapping_mul(0x9e3779b9).wrapping_add(2)),
            pathfinding_rng: StdRng::seed_from_u64(seed.wrapping_mul(0x9e3779b9).wrapping_add(3)),
        }
    }
}

/// Movement system (runs early in the schedule)
/// Applies velocity to position for all entities with both components
/// This is a basic kinematic system for entity movement
pub fn movement(mut q: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in q.iter_mut() {
        pos.0 += vel.0;
        pos.1 += vel.1;
    }
}

/// Movement behavior configuration
/// Controls whether job execution teleports or moves stepwise toward targets
#[derive(Resource, Debug, Clone, Copy)]
pub struct MovementConfig {
    /// When true, entities only move one step toward their target per tick
    /// When false, systems may teleport to targets for simplicity/tests
    pub stepwise: bool,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self { stepwise: true }
    }
}

/// Confine positions to map bounds (runs after movement)
/// Prevents entities from moving outside the valid map area
/// Clamps positions to the map boundaries for safety
pub fn confine_to_map(map: Res<GameMap>, mut q: Query<&mut Position>) {
    for mut pos in q.iter_mut() {
        pos.0 = pos.0.clamp(0, map.width as i32 - 1);
        pos.1 = pos.1.clamp(0, map.height as i32 - 1);
    }
}

/// Increments the tick counter; place at the end of the schedule for clarity
/// This system should run last to properly count completed simulation steps
/// Provides the authoritative time source for the simulation
pub fn advance_time(mut time: ResMut<Time>) {
    time.ticks += 1;
}

/// Mining execution system - processes Mine jobs and converts Wall->Floor, spawns Stone items
/// This is the core mining system that executes mining jobs assigned to Miner entities
/// Miners must be adjacent to (or at) the target tile to successfully mine it
/// Mining converts Wall tiles to Floor tiles and spawns Stone items at the mined location
pub fn mining_execution_system(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    mut active_jobs: ResMut<ActiveJobs>,
    mut q_miners: Query<(&mut AssignedJob, &Position), With<Miner>>,
) {
    for (mut assigned_job, miner_pos) in q_miners.iter_mut() {
        if let Some(job_id) = assigned_job.0 {
            if let Some(job) = active_jobs.jobs.get(&job_id) {
                if let JobKind::Mine { x, y } = job.kind {
                    // Allow mining when adjacent (including same tile)
                    // This gives miners a 3x3 area of effect around their position
                    let dx = (miner_pos.0 - x).abs();
                    let dy = (miner_pos.1 - y).abs();
                    if dx <= 1 && dy <= 1 && map.get_tile(x, y) == Some(TileKind::Wall) {
                        // Convert Wall to Floor (the primary mining action)
                        map.set_tile(x, y, TileKind::Floor);

                        // Spawn a stone item at the mined location
                        // Items are full entities with position and carriable properties
                        commands.spawn((
                            Item {
                                item_type: crate::components::ItemType::Stone,
                            },
                            Stone,
                            Position(x, y),
                            Carriable,
                            Name("Stone".to_string()),
                        ));

                        // Complete job - remove from active jobs and clear assignment
                        active_jobs.jobs.remove(&job_id);
                        assigned_job.0 = None;
                    }
                }
            } else {
                // Job missing in active jobs; clear assignment defensively
                // This can happen if jobs are manually removed or due to system ordering
                assigned_job.0 = None;
            }
        }
    }
}

/// Execute hauling jobs: move items to stockpiles using improved inventory system
/// This is a complex system that handles item transportation from pickup to delivery
/// Uses a multi-pass approach to avoid borrowing conflicts and ensure consistent state
/// Supports both immediate delivery (pickup+drop in one tick) and staged hauling
#[allow(clippy::type_complexity)]
pub fn hauling_execution_system(
    _commands: Commands,
    mut active_jobs: ResMut<ActiveJobs>,
    config: Option<Res<MovementConfig>>,
    mut param_set: ParamSet<(
        Query<(&mut AssignedJob, &mut Inventory, &mut Position), (With<Carrier>, Without<Miner>)>,
        Query<(Entity, &mut Position), (With<Item>, With<Carriable>)>,
    )>,
) {
    // Internal structs for tracking planned updates
    // This approach prevents borrowing conflicts by collecting all planned changes first

    /// Planned update for a carrier entity during hauling execution
    #[derive(Clone, Copy)]
    struct CarrierUpdate {
        job_id: JobId,
        target: (i32, i32),          // Where the carrier should move
        from: (i32, i32),            // Original pickup location
        dropping: bool,              // Whether carrier is dropping an item this tick
        pickup_item: Option<Entity>, // Item entity to pick up (if any)
    }

    /// Planned update for an item entity being hauled
    #[derive(Clone, Copy)]
    struct ItemUpdate {
        entity: Entity,
        target: (i32, i32), // Where the item should be moved
    }

    // Pre-allocate collections to avoid repeated reallocations during planning
    let carriers_count = { param_set.p0().iter().count() };
    let mut carrier_updates: Vec<CarrierUpdate> = Vec::with_capacity(carriers_count);
    let mut item_updates: Vec<ItemUpdate> = Vec::with_capacity(carriers_count);
    let mut completed_jobs: Vec<JobId> = Vec::with_capacity(carriers_count);
    // First pass: collect carrier state and plan updates
    // Examines all carriers with haul jobs and determines what actions to take
    {
        let q_carriers = param_set.p0();
        let stepwise = config.map(|c| c.stepwise).unwrap_or(false);
        for (assigned_job, inventory, carrier_pos) in q_carriers.iter() {
            if let Some(job_id) = assigned_job.0 {
                if let Some(job) = active_jobs.jobs.get(&job_id) {
                    if let JobKind::Haul { from, to } = job.kind {
                        if let Some(carried_item) = inventory.0 {
                            // Carrier has item, plan movement toward destination
                            let target = if stepwise {
                                step_toward(carrier_pos.0, carrier_pos.1, to.0, to.1)
                            } else {
                                to
                            };
                            let will_drop = !stepwise || (target.0 == to.0 && target.1 == to.1);
                            carrier_updates.push(CarrierUpdate {
                                job_id,
                                target,
                                from,
                                dropping: will_drop,
                                pickup_item: None,
                            });
                            if will_drop {
                                item_updates.push(ItemUpdate {
                                    entity: carried_item,
                                    target: to,
                                });
                                // Job completes on drop
                                completed_jobs.push(job_id);
                            }
                        } else {
                            // Carrier needs to pick up item first
                            // If carrier is already at the pickup location, only pick up this tick.
                            // Otherwise, allow immediate deliver (pickup-and-drop) within one tick to satisfy
                            // simple pipeline tests that expect single-step hauling.
                            if carrier_pos.0 == from.0 && carrier_pos.1 == from.1 {
                                carrier_updates.push(CarrierUpdate {
                                    job_id,
                                    target: from,
                                    from,
                                    dropping: false,
                                    pickup_item: None,
                                });
                            } else {
                                // Move toward pickup or allow immediate delivery depending on config
                                if stepwise {
                                    let target =
                                        step_toward(carrier_pos.0, carrier_pos.1, from.0, from.1);
                                    carrier_updates.push(CarrierUpdate {
                                        job_id,
                                        target,
                                        from,
                                        dropping: false,
                                        pickup_item: None,
                                    });
                                } else {
                                    // Immediate delivery path for testing compatibility
                                    carrier_updates.push(CarrierUpdate {
                                        job_id,
                                        target: to,
                                        from,
                                        dropping: true,
                                        pickup_item: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Second pass: find items to pick up for carriers that need them
    // Matches carriers with items at their pickup locations
    {
        let q_items = param_set.p1();
        for carrier_update in &mut carrier_updates {
            if !carrier_update.dropping {
                // Carrier needs to pick up an item
                let pickup_pos = carrier_update.target;
                for (item_entity, item_pos) in q_items.iter() {
                    if item_pos.0 == pickup_pos.0 && item_pos.1 == pickup_pos.1 {
                        // Mark that we can pick up the item this tick at pickup position
                        carrier_update.pickup_item = Some(item_entity);
                        break;
                    }
                }
            } else if carrier_update.pickup_item.is_none() {
                // Immediate deliver path: find item at 'from' and move it to target in the same tick.
                // This supports single-tick hauling for simple test scenarios
                let pickup_pos = carrier_update.from;
                for (item_entity, item_pos) in q_items.iter() {
                    if item_pos.0 == pickup_pos.0 && item_pos.1 == pickup_pos.1 {
                        item_updates.push(ItemUpdate {
                            entity: item_entity,
                            target: carrier_update.target,
                        });
                        completed_jobs.push(carrier_update.job_id);
                        break;
                    }
                }
            }
        }
    }

    // Build a map for O(1) lookup by JobId during application phase
    let update_map: HashMap<JobId, CarrierUpdate> = carrier_updates
        .iter()
        .copied()
        .map(|u| (u.job_id, u))
        .collect();

    // Third pass: apply carrier updates
    // Updates carrier positions, inventories, and job assignments
    {
        let mut q_carriers = param_set.p0();
        for (mut assigned_job, mut inventory, mut carrier_pos) in q_carriers.iter_mut() {
            if let Some(job_id) = assigned_job.0 {
                if let Some(update) = update_map.get(&job_id) {
                    // Update carrier position to target location
                    carrier_pos.0 = update.target.0;
                    carrier_pos.1 = update.target.1;

                    if update.dropping {
                        // Dropping item - clear inventory and complete job
                        inventory.0 = None;
                        assigned_job.0 = None;
                    } else if let Some(item_entity) = update.pickup_item {
                        // Picking up item - add to inventory
                        inventory.0 = Some(item_entity);
                    }
                }
            }
        }
    }

    // Fourth pass: apply item position updates
    // Moves items to their destination positions when hauled
    {
        let mut q_items = param_set.p1();
        for upd in item_updates {
            if let Ok((_, mut item_pos)) = q_items.get_mut(upd.entity) {
                item_pos.0 = upd.target.0;
                item_pos.1 = upd.target.1;
            }
        }
    }

    // Mark completed jobs as done in ActiveJobs
    // Removes completed haul jobs from the active job tracker
    for job_id in completed_jobs.into_iter() {
        active_jobs.jobs.remove(&job_id);
    }
}

/// Take one Manhattan step from (x,y) toward (tx,ty)
fn step_toward(x: i32, y: i32, tx: i32, ty: i32) -> (i32, i32) {
    let dx = (tx - x).signum();
    let dy = (ty - y).signum();
    // Move exactly one Manhattan step: prefer horizontal first for determinism
    if x != tx {
        (x + dx, y)
    } else if y != ty {
        (x, y + dy)
    } else {
        (x, y)
    }
}

/// Automatically create haul jobs when items are spawned and stockpiles exist
/// This system creates hauling jobs for newly spawned items (like from mining)
/// Uses the `Added<Item>` filter to only process items created this tick
/// Finds the nearest stockpile and creates a haul job from item to stockpile
pub fn auto_haul_system(
    mut job_board: ResMut<JobBoard>,
    mut rng: ResMut<DeterministicRng>,
    q_items: Query<&Position, (With<Item>, Added<Item>)>,
    q_stockpiles: Query<&Position, With<Stockpile>>,
) {
    // Find nearest stockpile for each new item
    for item_pos in q_items.iter() {
        if let Some(stockpile_pos) = find_nearest_stockpile(&q_stockpiles, item_pos) {
            add_job(
                &mut job_board,
                JobKind::Haul {
                    from: (item_pos.0, item_pos.1),
                    to: (stockpile_pos.0, stockpile_pos.1),
                },
                &mut rng.job_rng,
            );
        }
    }
}

/// Helper function to find the nearest stockpile to an item
/// Uses Euclidean distance to determine the closest stockpile
/// Returns None if no stockpiles exist in the world
fn find_nearest_stockpile(
    stockpiles: &Query<&Position, With<Stockpile>>,
    item_pos: &Position,
) -> Option<Position> {
    let mut nearest: Option<Position> = None;
    let mut min_distance = f32::INFINITY;

    for stockpile_pos in stockpiles.iter() {
        let dx = (stockpile_pos.0 - item_pos.0) as f32;
        let dy = (stockpile_pos.1 - item_pos.1) as f32;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < min_distance {
            min_distance = distance;
            nearest = Some(*stockpile_pos);
        }
    }

    nearest
}
