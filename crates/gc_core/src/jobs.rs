use crate::components::AssignedJob;
use crate::systems::DeterministicRng;
use bevy_ecs::prelude::*;
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

pub fn add_job(
    board: &mut ResMut<JobBoard>,
    kind: JobKind,
    rng: &mut StdRng,
) -> JobId {
    // Generate deterministic UUID using job_rng stream
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);
    let id = JobId(Uuid::from_bytes(bytes));
    board.0.push(Job { id, kind });
    id
}

pub fn take_next_job(board: &mut ResMut<JobBoard>) -> Option<Job> {
    board.0.pop()
}

pub fn job_assignment_system(
    mut board: ResMut<JobBoard>,
    mut q_idle: Query<
        &mut AssignedJob,
        (
            With<crate::components::Carrier>,
            Without<crate::components::Miner>,
        ),
    >,
) {
    for mut assigned in q_idle.iter_mut() {
        if assigned.0.is_none() {
            if let Some(job) = take_next_job(&mut board) {
                assigned.0 = Some(job.id);
                // For MVP we just drop the job; execution systems would track active jobs.
            }
        }
    }
}
