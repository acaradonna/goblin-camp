use crate::components::{AssignedJob, ItemType};
use crate::world::{GameMap, Position, TileKind};
use bevy_ecs::prelude::*;
use rand::rngs::StdRng;
use rand::Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JobId(pub Uuid);

#[derive(Debug, Clone)]
pub enum JobKind {
    Mine { x: i32, y: i32 },
    Haul { from: (i32, i32), to: (i32, i32) },
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: JobId,
    pub kind: JobKind,
}

#[derive(Resource, Default, Debug)]
pub struct JobBoard(pub Vec<Job>);

/// Event emitted when an item should be spawned in the world
#[derive(Debug, Clone)]
pub struct ItemSpawnRequest {
    pub item_type: ItemType,
    pub position: (i32, i32),
}

/// Resource to track item spawn requests
#[derive(Resource, Default, Debug)]
pub struct ItemSpawnQueue {
    pub requests: Vec<ItemSpawnRequest>,
}

pub fn add_job(board: &mut ResMut<JobBoard>, kind: JobKind, rng: &mut StdRng) -> JobId {
    // Generate deterministic UUID using job_rng stream
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);
    let id = JobId(Uuid::from_bytes(bytes));
    board.0.push(Job { id, kind });
    id
}

pub fn add_job_direct(board: &mut JobBoard, kind: JobKind) -> JobId {
    let id = JobId(Uuid::new_v4());
    board.0.push(Job { id, kind });
    id
}

pub fn take_next_job(board: &mut ResMut<JobBoard>) -> Option<Job> {
    board.0.pop()
}

pub fn job_assignment_system(
    mut board: ResMut<JobBoard>,
    mut q_idle: Query<&mut AssignedJob, With<crate::components::Carrier>>,
) {
    for mut assigned in q_idle.iter_mut() {
        if assigned.0.is_none() {
            if let Some(job) = take_next_job(&mut board) {
                assigned.0 = Some(job.id);
                // For simplified demo, just drop the job; execution systems track active jobs.
            }
        }
    }
}

/// Assigns mining jobs specifically to miners
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
#[derive(Resource, Default, Debug)]
pub struct ActiveJobs {
    pub jobs: std::collections::HashMap<JobId, Job>,
}

/// System that executes mining jobs by converting Wall tiles to Floor and emitting ItemSpawn events
pub fn mine_job_execution_system(
    mut map: ResMut<GameMap>,
    mut item_spawn_queue: ResMut<ItemSpawnQueue>,
    mut active_jobs: ResMut<ActiveJobs>,
    mut q_miners: Query<(&mut AssignedJob, &Position), With<crate::components::Miner>>,
) {
    for (mut assigned_job, _miner_pos) in q_miners.iter_mut() {
        if let Some(job_id) = assigned_job.0 {
            // Look up the job details from active jobs or try to execute
            // For this MVP, we'll assume the job exists and try to find any minable walls
            if let Some(job) = active_jobs.jobs.get(&job_id) {
                if let JobKind::Mine { x, y } = job.kind {
                    if let Some(current_tile) = map.get_tile(x, y) {
                        if current_tile == TileKind::Wall {
                            // Convert Wall to Floor
                            map.set_tile(x, y, TileKind::Floor);

                            // Queue ItemSpawn request for stone
                            item_spawn_queue.requests.push(ItemSpawnRequest {
                                item_type: ItemType::Stone,
                                position: (x, y),
                            });
                        }
                    }

                    // Job is complete, remove from active jobs and clear assignment
                    active_jobs.jobs.remove(&job_id);
                    assigned_job.0 = None;
                }
            } else {
                // Job not found in active jobs, clear assignment
                assigned_job.0 = None;
            }
        }
    }
}
