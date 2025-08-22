use bevy_ecs::prelude::*;
use crate::jobs::{JobKind, add_job, JobBoard};

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
    mut board: ResMut<JobBoard>,
    q: Query<&crate::world::Position, With<MineDesignation>>,
) {
    if !config.auto_jobs { return; }
    for pos in q.iter() {
        add_job(&mut board, JobKind::Mine { x: pos.0, y: pos.1 });
    }
}
