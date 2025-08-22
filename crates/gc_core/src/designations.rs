use crate::jobs::{add_job, JobBoard, JobKind};
use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct MineDesignation;

#[derive(Bundle)]
pub struct DesignationBundle {
    pub pos: crate::world::Position,
    pub kind: MineDesignation,
}

#[derive(Resource, Default, Debug)]
pub struct DesignationConfig {
    pub auto_jobs: bool,
}

pub fn designation_to_jobs_system(
    config: Res<DesignationConfig>,
    mut board: ResMut<JobBoard>,
    mut commands: Commands,
    q: Query<(Entity, &crate::world::Position), With<MineDesignation>>,
) {
    if !config.auto_jobs {
        return;
    }
    for (e, pos) in q.iter() {
        add_job(&mut board, JobKind::Mine { x: pos.0, y: pos.1 });
        // Consume the designation immediately to prevent duplicate job creation on subsequent ticks.
        commands.entity(e).despawn();
    }
}
