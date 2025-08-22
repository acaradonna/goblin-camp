use bevy_ecs::prelude::*;
use uuid::Uuid;
use crate::components::AssignedJob;
use std::collections::HashMap;

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
pub struct JobRegistry(pub HashMap<JobId, Job>);

#[derive(Resource, Default, Debug)]
pub struct JobQueue {
    pub mine_jobs: Vec<JobId>,
    pub haul_jobs: Vec<JobId>,
}

pub fn add_job(queue: &mut ResMut<JobQueue>, reg: &mut ResMut<JobRegistry>, kind: JobKind) -> JobId {
    let id = JobId(Uuid::new_v4());
    
    // Add to appropriate queue based on job kind
    match &kind {
        JobKind::Mine { .. } => queue.mine_jobs.push(id),
        JobKind::Haul { .. } => queue.haul_jobs.push(id),
    }
    
    reg.0.insert(id, Job { id, kind });
    id
}

pub fn take_next_mining_job(queue: &mut ResMut<JobQueue>) -> Option<JobId> {
    queue.mine_jobs.pop()
}

pub fn take_next_haul_job(queue: &mut ResMut<JobQueue>) -> Option<JobId> {
    queue.haul_jobs.pop()
}

pub fn get_job<'a>(reg: &'a Res<JobRegistry>, id: JobId) -> Option<&'a Job> { reg.0.get(&id) }
pub fn remove_job(reg: &mut ResMut<JobRegistry>, id: JobId) { reg.0.remove(&id); }

pub fn miner_assignment_system(
    mut queue: ResMut<JobQueue>,
    mut q_idle: Query<&mut AssignedJob, With<crate::components::Miner>>,
) {
    for mut assigned in q_idle.iter_mut() {
        if assigned.0.is_none() {
            if let Some(id) = take_next_mining_job(&mut queue) {
                assigned.0 = Some(id);
            }
        }
    }
}

pub fn mining_execution_system(
    mut map: ResMut<crate::world::GameMap>,
    mut reg: ResMut<JobRegistry>,
    mut q: Query<(&crate::world::Position, &mut AssignedJob), With<crate::components::Miner>>,
) {
    for (_pos, mut assigned) in q.iter_mut() {
        let Some(job_id) = assigned.0 else { continue };
        let Some(job) = reg.0.get(&job_id) else { continue };
        match &job.kind {
            JobKind::Mine { x, y } => {
                if let Some(kind) = map.get_tile(*x, *y) {
                    if matches!(kind, crate::world::TileKind::Wall) {
                        let _ = map.set_tile(*x, *y, crate::world::TileKind::Floor);
                    }
                }
                assigned.0 = None;
                reg.0.remove(&job_id);
            }
            _ => {
                eprintln!(
                    "Warning: mining_execution_system: Unhandled job kind for job_id {:?}: {:?}",
                    job_id, job.kind
                );
            }
        }
    }
}
