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
pub struct JobQueue(pub Vec<JobId>);

pub fn add_job(queue: &mut ResMut<JobQueue>, reg: &mut ResMut<JobRegistry>, kind: JobKind) -> JobId {
    let id = JobId(Uuid::new_v4());
    reg.0.insert(id, Job { id, kind });
    queue.0.push(id);
    id
}

pub fn take_next_matching<F>(queue: &mut ResMut<JobQueue>, reg: &Res<JobRegistry>, pred: F) -> Option<JobId>
where F: Fn(&Job) -> bool {
    if queue.0.is_empty() { return None; }
    let mut idx = None;
    for (i, jid) in queue.0.iter().enumerate() {
        if let Some(job) = reg.0.get(jid) { if pred(job) { idx = Some(i); break; } }
    }
    if let Some(i) = idx { Some(queue.0.remove(i)) } else { None }
}

pub fn get_job<'a>(reg: &'a Res<JobRegistry>, id: JobId) -> Option<&'a Job> { reg.0.get(&id) }
pub fn remove_job(reg: &mut ResMut<JobRegistry>, id: JobId) { reg.0.remove(&id); }

pub fn miner_assignment_system(
    mut queue: ResMut<JobQueue>,
    reg: Res<JobRegistry>,
    mut q_idle: Query<&mut AssignedJob, With<crate::components::Miner>>,
) {
    for mut assigned in q_idle.iter_mut() {
        if assigned.0.is_none() {
            if let Some(id) = take_next_matching(&mut queue, &reg, |j| matches!(j.kind, JobKind::Mine { .. })) {
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
        let Some(job) = reg.0.get(&job_id).cloned() else { continue };
        match job.kind {
            JobKind::Mine { x, y } => {
                if let Some(kind) = map.get_tile(x, y) {
                    if matches!(kind, crate::world::TileKind::Wall) {
                        let _ = map.set_tile(x, y, crate::world::TileKind::Floor);
                    }
                }
                assigned.0 = None;
                reg.0.remove(&job_id);
            }
            _ => {}
        }
    }
}
