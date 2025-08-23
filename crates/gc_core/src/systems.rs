use crate::world::*;
use bevy_ecs::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

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

/// Increments the tick counter; place at the end of the schedule for clarity
pub fn advance_time(mut time: ResMut<Time>) {
    time.ticks += 1;
}

/// Mining execution system - processes Mine jobs and converts Wall->Floor, spawns Stone items
pub fn mining_execution_system(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    mut job_board: ResMut<crate::jobs::JobBoard>,
    mut q_miners: Query<&mut crate::components::AssignedJob, With<crate::components::Miner>>,
) {
    let mut jobs_to_complete = Vec::new();

    for mut assigned_job in q_miners.iter_mut() {
        if let Some(job_id) = assigned_job.0 {
            // Find the job in the board
            if let Some(job) = job_board.0.iter().find(|j| j.id == job_id) {
                if let crate::jobs::JobKind::Mine { x, y } = job.kind {
                    // Check if there's a wall at this position
                    if let Some(crate::world::TileKind::Wall) = map.get_tile(x, y) {
                        // Convert wall to floor
                        map.set_tile(x, y, crate::world::TileKind::Floor);

                        // Spawn a stone item at this position
                        commands.spawn((
                            crate::components::Item {
                                item_type: crate::components::ItemType::Stone,
                            },
                            crate::components::Carriable,
                            Position(x, y),
                            Name("Stone".to_string()),
                        ));
                    }

                    // Mark job for completion
                    jobs_to_complete.push(job_id);
                }
            }
        }
    }

    // Remove completed jobs and clear assignments
    for job_id in jobs_to_complete {
        // Remove job from board
        job_board.0.retain(|job| job.id != job_id);

        // Clear miner assignments
        for mut assigned_job in q_miners.iter_mut() {
            if assigned_job.0 == Some(job_id) {
                assigned_job.0 = None;
            }
        }
    }
}

/// Hauling execution system - processes Haul jobs and moves items to stockpiles
pub fn hauling_execution_system(
    mut job_board: ResMut<crate::jobs::JobBoard>,
    mut q_haulers: Query<
        &mut crate::components::AssignedJob,
        (
            With<crate::components::Carrier>,
            Without<crate::components::Miner>,
        ),
    >,
    mut q_items: Query<
        &mut Position,
        (
            With<crate::components::Item>,
            With<crate::components::Carriable>,
        ),
    >,
) {
    let mut jobs_to_complete = Vec::new();

    for assigned_job in q_haulers.iter_mut() {
        if let Some(job_id) = assigned_job.0 {
            // Find the job in the board
            if let Some(job) = job_board.0.iter().find(|j| j.id == job_id) {
                if let crate::jobs::JobKind::Haul { from, to } = job.kind {
                    // For simplicity, find any item at the 'from' position and move it to 'to'
                    for mut item_pos in q_items.iter_mut() {
                        if item_pos.0 == from.0 && item_pos.1 == from.1 {
                            // Move the item to the destination
                            item_pos.0 = to.0;
                            item_pos.1 = to.1;
                            break;
                        }
                    }

                    // Mark job for completion
                    jobs_to_complete.push(job_id);
                }
            }
        }
    }

    // Remove completed jobs and clear assignments
    for job_id in jobs_to_complete {
        // Remove job from board
        job_board.0.retain(|job| job.id != job_id);

        // Clear hauler assignments
        for mut assigned_job in q_haulers.iter_mut() {
            if assigned_job.0 == Some(job_id) {
                assigned_job.0 = None;
            }
        }
    }
}

/// System to automatically create haul jobs for items that need to be moved to stockpiles
pub fn auto_haul_system(
    mut job_board: ResMut<crate::jobs::JobBoard>,
    q_items: Query<&Position, With<crate::components::Item>>,
    q_stockpiles: Query<&Position, With<crate::components::Stockpile>>,
) {
    // For each item, if there's a stockpile available, create a haul job
    for item_pos in q_items.iter() {
        // Find the nearest stockpile (simplified to just pick the first one)
        if let Some(stockpile_pos) = q_stockpiles.iter().next() {
            // Only create haul job if item is not already at stockpile
            if item_pos.0 != stockpile_pos.0 || item_pos.1 != stockpile_pos.1 {
                // Check if a haul job already exists for this item position
                let haul_job_exists = job_board.0.iter().any(|job| {
                    matches!(job.kind, crate::jobs::JobKind::Haul { from, .. }
                        if from.0 == item_pos.0 && from.1 == item_pos.1)
                });

                if !haul_job_exists {
                    crate::jobs::add_job(
                        &mut job_board,
                        crate::jobs::JobKind::Haul {
                            from: (item_pos.0, item_pos.1),
                            to: (stockpile_pos.0, stockpile_pos.1),
                        },
                    );
                }
            }
        }
    }
}
