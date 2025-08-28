use crate::components::{AssignedJob, Item, ItemType};
use crate::world::{GameMap, Position, TileKind};
use bevy_ecs::prelude::*;
use rand::rngs::StdRng;
use rand::Rng;
use uuid::Uuid;

/// Job System for Goblin Camp
/// 
/// This module implements the core job assignment and execution system.
/// Jobs represent tasks that entities can perform, such as mining or hauling items.
/// The system follows a job board pattern where jobs are posted, assigned to workers,
/// and then executed by specialized systems.

/// Unique identifier for jobs using UUID
/// Provides globally unique IDs that are deterministic when using seeded RNG
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JobId(pub Uuid);

/// Enumeration of different job types that can be assigned to entities
/// Each job type contains the specific parameters needed for execution
#[derive(Debug, Clone)]
pub enum JobKind {
    /// Mining job to convert a wall tile to floor at specific coordinates
    /// Parameters: target coordinates (x, y) to mine
    Mine { x: i32, y: i32 },
    /// Hauling job to move an item from one location to another
    /// Parameters: source position and destination position
    Haul { from: (i32, i32), to: (i32, i32) },
}

/// A job with its unique identifier and specific task details
/// Jobs are created on the job board and assigned to appropriate workers
#[derive(Debug, Clone)]
pub struct Job {
    /// Unique identifier for this job
    pub id: JobId,
    /// Specific type and parameters of the job
    pub kind: JobKind,
}

/// Resource representing the global job board where unassigned jobs are stored
/// Jobs are posted here by designation systems and taken by assignment systems
/// Uses a Vec as a simple LIFO queue (last posted, first assigned)
#[derive(Resource, Default, Debug)]
pub struct JobBoard(pub Vec<Job>);

/// Event emitted when an item should be spawned in the world
/// Used to decouple item creation from the systems that trigger it (like mining)
/// This allows proper system ordering and prevents timing issues
#[derive(Debug, Clone)]
pub struct ItemSpawnRequest {
    /// Type of item to spawn (Stone, Wood, etc.)
    pub item_type: ItemType,
    /// World coordinates where the item should be placed
    pub position: (i32, i32),
}

/// Resource to track item spawn requests that need to be processed
/// Acts as a queue between systems that generate items and the system that creates them
/// Ensures items are spawned in the correct order and timing
#[derive(Resource, Default, Debug)]
pub struct ItemSpawnQueue {
    /// Queue of pending item spawn requests
    pub requests: Vec<ItemSpawnRequest>,
}

/// Add a new job to the job board with a deterministic UUID
/// Uses the provided RNG to generate a reproducible job ID for deterministic simulation
/// Returns the JobId for reference by other systems
pub fn add_job(board: &mut ResMut<JobBoard>, kind: JobKind, rng: &mut StdRng) -> JobId {
    // Generate deterministic UUID using job_rng stream
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);
    let id = JobId(Uuid::from_bytes(bytes));
    board.0.push(Job { id, kind });
    id
}

/// Remove and return the next available job from the job board
/// Uses LIFO ordering (last in, first out) for simplicity
/// Returns None if no jobs are available
pub fn take_next_job(board: &mut ResMut<JobBoard>) -> Option<Job> {
    board.0.pop()
}

/// System that assigns available jobs to workers based on their capabilities
/// Miners get mining jobs, Carriers get hauling jobs
/// Only assigns one job per entity per system run to prevent over-assignment
/// Jobs are moved from the JobBoard to ActiveJobs when assigned
pub fn job_assignment_system(
    mut board: ResMut<JobBoard>,
    mut active_jobs: ResMut<ActiveJobs>,
    mut q_miners: Query<
        &mut AssignedJob,
        (
            With<crate::components::Miner>,
            Without<crate::components::Carrier>,
        ),
    >,
    mut q_carriers: Query<
        &mut AssignedJob,
        (
            With<crate::components::Carrier>,
            Without<crate::components::Miner>,
        ),
    >,
) {
    // Assign mining jobs to miners
    for mut assigned in q_miners.iter_mut() {
        if assigned.0.is_none() {
            // Find a mining job
            if let Some(pos) = board
                .0
                .iter()
                .position(|job| matches!(job.kind, JobKind::Mine { .. }))
            {
                let job = board.0.remove(pos);
                let job_id = job.id;
                // Store the job in active jobs for execution
                active_jobs.jobs.insert(job_id, job);
                assigned.0 = Some(job_id);
                break; // Only assign one job per system run
            }
        }
    }

    // Assign hauling jobs to carriers
    for mut assigned in q_carriers.iter_mut() {
        if assigned.0.is_none() {
            // Find a hauling job
            if let Some(pos) = board
                .0
                .iter()
                .position(|job| matches!(job.kind, JobKind::Haul { .. }))
            {
                let job = board.0.remove(pos);
                let job_id = job.id;
                // Store the job in active jobs for execution
                active_jobs.jobs.insert(job_id, job);
                assigned.0 = Some(job_id);
                break; // Only assign one job per system run
            }
        }
    }
}

/// Assigns mining jobs specifically to miners (specialized version)
/// Alternative to the general job_assignment_system when you only want mining assignment
/// More focused and predictable for testing specific mining scenarios
pub fn mining_job_assignment_system(
    mut board: ResMut<JobBoard>,
    mut active_jobs: ResMut<ActiveJobs>,
    mut q_miners: Query<&mut AssignedJob, With<crate::components::Miner>>,
) {
    for mut assigned in q_miners.iter_mut() {
        if assigned.0.is_none() {
            // Look for a mining job specifically
            if let Some(pos) = board
                .0
                .iter()
                .position(|job| matches!(job.kind, JobKind::Mine { .. }))
            {
                let job = board.0.remove(pos);
                let job_id = job.id;

                // Store the job in active jobs for execution
                active_jobs.jobs.insert(job_id, job);
                assigned.0 = Some(job_id);
            }
        }
    }
}

/// Resource to track active jobs being executed
/// Jobs are moved here from the JobBoard when assigned to workers
/// Contains the full job details needed for execution systems
#[derive(Resource, Default, Debug)]
pub struct ActiveJobs {
    /// Map of JobId to Job for quick lookup during execution
    pub jobs: std::collections::HashMap<JobId, Job>,
}

/// System that processes ItemSpawnQueue and creates actual item entities
/// This system runs after job execution systems to create items from queued requests
/// Decouples item creation from the systems that trigger it for better system ordering
pub fn process_item_spawn_queue_system(
    mut commands: Commands,
    mut spawn_queue: ResMut<ItemSpawnQueue>,
) {
    for request in spawn_queue.requests.drain(..) {
        let (x, y) = request.position;

        match request.item_type {
            ItemType::Stone => {
                // Create a complete stone item entity with all necessary components
                commands.spawn((
                    Item {
                        item_type: ItemType::Stone,
                    },
                    crate::components::Stone,
                    crate::world::Position(x, y),
                    crate::components::Carriable,
                    crate::world::Name("Stone".to_string()),
                ));
            }
        }
    }
}

/// System that executes mining jobs by converting Wall tiles to Floor and emitting ItemSpawn events
/// This is the core mining execution system that performs the actual work of mining
/// Miners with assigned Mine jobs will execute them here, modifying the world and creating items
pub fn mine_job_execution_system(
    mut map: ResMut<GameMap>,
    mut item_spawn_queue: ResMut<ItemSpawnQueue>,
    mut active_jobs: ResMut<ActiveJobs>,
    mut q_miners: Query<(&mut AssignedJob, &Position), With<crate::components::Miner>>,
) {
    for (mut assigned_job, _miner_pos) in q_miners.iter_mut() {
        if let Some(job_id) = assigned_job.0 {
            // Look up the job details from active jobs
            if let Some(job) = active_jobs.jobs.get(&job_id) {
                if let JobKind::Mine { x, y } = job.kind {
                    if let Some(current_tile) = map.get_tile(x, y) {
                        if current_tile == TileKind::Wall {
                            // Convert Wall to Floor (the primary mining action)
                            map.set_tile(x, y, TileKind::Floor);

                            // Queue ItemSpawn request for stone (mining produces stone items)
                            item_spawn_queue.requests.push(ItemSpawnRequest {
                                item_type: ItemType::Stone,
                                position: (x, y),
                            });
                        }
                    }

                    // Job is complete, clean up active job and clear assignment
                    active_jobs.jobs.remove(&job_id);
                    assigned_job.0 = None;
                }
            } else {
                // Job not found in active jobs, clear assignment defensively
                assigned_job.0 = None;
            }
        }
    }
}
