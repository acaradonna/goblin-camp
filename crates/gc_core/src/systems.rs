use crate::components::*;
use crate::jobs::*;
use crate::world::*;
use bevy_ecs::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;

/// Fixed-step time resource for deterministic ticks
#[derive(Resource, Debug, Clone, Copy)]
pub struct Time {
    /// Accumulated tick count
    pub ticks: u64,
    /// Duration of a tick in milliseconds (for reference/logging)
    pub tick_ms: u64,
}

impl Time {
    pub fn new(tick_ms: u64) -> Self {
        Self { ticks: 0, tick_ms }
    }
}

/// Centralized deterministic RNG resource with separate streams per subsystem
#[derive(Resource, Debug)]
pub struct DeterministicRng {
    /// Master seed for reproducibility
    pub master_seed: u64,
    /// RNG stream for terrain generation
    pub mapgen_rng: StdRng,
    /// RNG stream for job selection and UUID generation
    pub job_rng: StdRng,
    /// RNG stream for combat calculations (future use)
    pub combat_rng: StdRng,
    /// RNG stream for pathfinding randomization (future use)
    pub pathfinding_rng: StdRng,
}

impl DeterministicRng {
    pub fn new(seed: u64) -> Self {
        Self {
            master_seed: seed,
            mapgen_rng: StdRng::seed_from_u64(seed.wrapping_mul(0x9e3779b9).wrapping_add(0)),
            job_rng: StdRng::seed_from_u64(seed.wrapping_mul(0x9e3779b9).wrapping_add(1)),
            combat_rng: StdRng::seed_from_u64(seed.wrapping_mul(0x9e3779b9).wrapping_add(2)),
            pathfinding_rng: StdRng::seed_from_u64(seed.wrapping_mul(0x9e3779b9).wrapping_add(3)),
        }
    }
}

/// Resource to track mining activity counts for demo visibility
#[derive(Resource, Debug, Default)]
pub struct MiningStats {
    pub tiles_mined: u32,
}

/// Resource to track hauling activity counts for demo visibility  
#[derive(Resource, Debug, Default)]
pub struct HaulingStats {
    pub items_hauled: u32,
}

/// Movement system (runs early)
pub fn movement(mut q: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in q.iter_mut() {
        pos.0 += vel.0;
        pos.1 += vel.1;
    }
}

/// Confine positions to map bounds (runs after movement)
pub fn confine_to_map(map: Res<GameMap>, mut q: Query<&mut Position>) {
    for mut pos in q.iter_mut() {
        pos.0 = pos.0.clamp(0, map.width as i32 - 1);
        pos.1 = pos.1.clamp(0, map.height as i32 - 1);
    }
}

/// Mining execution system - processes assigned mining jobs
pub fn mining_execution_system(
    mut map: ResMut<GameMap>,
    mut stats: ResMut<MiningStats>,
    mut q_miners: Query<&mut AssignedJob, With<Miner>>,
) {
    for mut assigned in q_miners.iter_mut() {
        if let Some(_job_id) = assigned.0 {
            // For demo purposes, we'll assume any assigned job is a mining job
            // In a full implementation, we'd track active jobs separately
            // For now, just mine the designated position (5,5)
            if let Some(TileKind::Wall) = map.get_tile(5, 5) {
                map.set_tile(5, 5, TileKind::Floor);
                stats.tiles_mined += 1;
                // Job completed, clear assignment
                assigned.0 = None;
            }
        }
    }
}

/// Hauling execution system - processes assigned hauling jobs
pub fn hauling_execution_system(
    mut stats: ResMut<HaulingStats>,
    mut q_carriers: Query<&mut AssignedJob, (With<Carrier>, Without<Miner>)>,
) {
    for mut assigned in q_carriers.iter_mut() {
        if let Some(_job_id) = assigned.0 {
            // For demo purposes, simulate hauling completion
            // In a full implementation, we'd track active jobs and move items
            stats.items_hauled += 1;
            // Job completed, clear assignment
            assigned.0 = None;
        }
    }
}

/// Increments the tick counter; place at the end of the schedule for clarity
pub fn advance_time(mut time: ResMut<Time>) {
    time.ticks += 1;
}

/// Mining execution system - processes Mine jobs and converts Wall->Floor, spawns Stone items
pub fn mining_execution_system(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    mut job_board: ResMut<JobBoard>,
    mut q_miners: Query<(&mut AssignedJob, &Position), With<Miner>>,
) {
    let mut completed_jobs = Vec::new();

    for (mut assigned_job, miner_pos) in q_miners.iter_mut() {
        if let Some(job_id) = assigned_job.0 {
            // Find the job on the board to get details
            if let Some(job_idx) = job_board.0.iter().position(|j| j.id == job_id) {
                let job = &job_board.0[job_idx];
                if let JobKind::Mine { x, y } = job.kind {
                    // Check if miner is adjacent to the mining target (simplified - just check if miner is at target)
                    if miner_pos.0 == x && miner_pos.1 == y {
                        // Convert wall to floor
                        if map.get_tile(x, y) == Some(TileKind::Wall) {
                            map.set_tile(x, y, TileKind::Floor);

                            // Spawn a stone item at the mined location - use both Stone and Item components
                            commands.spawn((
                                Item {
                                    item_type: crate::components::ItemType::Stone,
                                },
                                Stone,
                                Position(x, y),
                                Carriable,
                                Name("Stone".to_string()),
                            ));

                            completed_jobs.push(job_idx);
                            assigned_job.0 = None; // Clear the assignment
                        }
                    }
                }
            }
        }
    }

    // Remove completed jobs from the board (iterate in reverse to maintain indices)
    for idx in completed_jobs.into_iter().rev() {
        job_board.0.remove(idx);
    }
}

/// Execute hauling jobs: move items to stockpiles using improved inventory system
#[allow(clippy::type_complexity)]
pub fn hauling_execution_system(
    _commands: Commands,
    mut job_board: ResMut<JobBoard>,
    mut param_set: ParamSet<(
        Query<(&mut AssignedJob, &mut Inventory, &mut Position), (With<Carrier>, Without<Miner>)>,
        Query<(Entity, &mut Position), (With<Item>, With<Carriable>)>,
    )>,
) {
    // Clear update types for readability over opaque tuples
    #[derive(Clone, Copy)]
    struct CarrierUpdate {
        job_id: JobId,
        target: (i32, i32),
        dropping: bool,
        pickup_item: Option<Entity>,
    }

    #[derive(Clone, Copy)]
    struct ItemUpdate {
        entity: Entity,
        target: (i32, i32),
    }

    // Pre-allocate to avoid repeated reallocations while planning updates
    let carriers_count = { param_set.p0().iter().count() };
    let mut completed_jobs: Vec<usize> = Vec::with_capacity(carriers_count);
    let mut carrier_updates: Vec<CarrierUpdate> = Vec::with_capacity(carriers_count);
    let mut item_updates: Vec<ItemUpdate> = Vec::with_capacity(carriers_count);

    // First pass: collect carrier state and planned updates
    {
        let q_carriers = param_set.p0();
        for (assigned_job, inventory, _carrier_pos) in q_carriers.iter() {
            if let Some(job_id) = assigned_job.0 {
                // Find the job on the board to get details
                if let Some(job_idx) = job_board.0.iter().position(|j| j.id == job_id) {
                    let job = &job_board.0[job_idx];
                    if let JobKind::Haul { from, to } = job.kind {
                        // Check if carrier is carrying an item already
                        if let Some(carried_item) = inventory.0 {
                            // Carrier has item, plan to move to destination and drop it
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
                            completed_jobs.push(job_idx);
                        } else {
                            // Carrier doesn't have item, plan to move to pickup location
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

    // Second pass: find items to pick up for carriers that need them
    {
        let q_items = param_set.p1();
        for carrier_update in &mut carrier_updates {
            if !carrier_update.dropping {
                // Carrier needs to pick up an item
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

    // Build a map for O(1) lookup by JobId
    let update_map: HashMap<JobId, CarrierUpdate> = carrier_updates
        .iter()
        .copied()
        .map(|u| (u.job_id, u))
        .collect();

    // Third pass: apply carrier updates
    {
        let mut q_carriers = param_set.p0();
        for (mut assigned_job, mut inventory, mut carrier_pos) in q_carriers.iter_mut() {
            if let Some(job_id) = assigned_job.0 {
                if let Some(update) = update_map.get(&job_id) {
                    // Update carrier position
                    carrier_pos.0 = update.target.0;
                    carrier_pos.1 = update.target.1;

                    if update.dropping {
                        // Dropping item
                        inventory.0 = None;
                        assigned_job.0 = None;
                    } else if let Some(item_entity) = update.pickup_item {
                        // Picking up item
                        inventory.0 = Some(item_entity);
                    }
                }
            }
        }
    }

    // Fourth pass: apply item position updates
    {
        let mut q_items = param_set.p1();
        for upd in item_updates {
            if let Ok((_, mut item_pos)) = q_items.get_mut(upd.entity) {
                item_pos.0 = upd.target.0;
                item_pos.1 = upd.target.1;
            }
        }
    }

    // Remove completed jobs from the board
    for idx in completed_jobs.into_iter().rev() {
        job_board.0.remove(idx);
    }
}

/// Automatically create haul jobs when items are spawned and stockpiles exist
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
