use bevy_ecs::prelude::*;
use crate::jobs::{JobKind, add_job, JobQueue, JobRegistry};
use std::collections::HashSet;

#[derive(Component, Debug)]
pub struct MineDesignation;

#[derive(Bundle)]
pub struct DesignationBundle {
    pub pos: crate::world::Position,
    pub kind: MineDesignation,
}

#[derive(Resource, Default, Debug)]
pub struct DesignationConfig { pub auto_jobs: bool }

pub fn designation_to_jobs_system(
    config: Res<DesignationConfig>,
    mut queue: ResMut<JobQueue>,
    mut reg: ResMut<JobRegistry>,
    mut commands: Commands,
    q: Query<(Entity, &crate::world::Position), With<MineDesignation>>,
) {
    if !config.auto_jobs { return; }
    let mut seen: HashSet<(i32,i32)> = HashSet::new();
    for (e, pos) in q.iter() {
        if !seen.insert((pos.0, pos.1)) { continue; }
        add_job(&mut queue, &mut reg, JobKind::Mine { x: pos.0, y: pos.1 });
        // Consume designation entity after creating job
        commands.entity(e).despawn();
    }
}
