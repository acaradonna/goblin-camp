use crate::components::{DesignationLifecycle, DesignationState};
use crate::jobs::{add_job, JobBoard, JobKind};
use bevy_ecs::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug)]
pub struct MineDesignation;

#[derive(Bundle)]
pub struct DesignationBundle {
    pub pos: crate::world::Position,
    pub kind: MineDesignation,
    pub lifecycle: DesignationLifecycle,
}

impl Default for DesignationBundle {
    fn default() -> Self {
        Self {
            pos: crate::world::Position(0, 0),
            kind: MineDesignation,
            lifecycle: DesignationLifecycle::default(),
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct DesignationConfig {
    pub auto_jobs: bool,
}

/// System that deduplicates designations by marking later ones at the same position as Ignored
pub fn designation_dedup_system(
    mut q_designations: Query<
        (Entity, &crate::world::Position, &mut DesignationLifecycle),
        With<MineDesignation>,
    >,
) {
    // Collect all active designations by position
    let mut position_map: HashMap<(i32, i32), Vec<Entity>> = HashMap::new();

    // First pass: collect entities by position, only considering Active designations
    for (entity, pos, lifecycle) in q_designations.iter() {
        if lifecycle.0 == DesignationState::Active {
            let position = (pos.0, pos.1);
            position_map.entry(position).or_default().push(entity);
        }
    }

    // Find entities to mark as ignored (all but first at each position)
    let mut entities_to_ignore: Vec<Entity> = Vec::new();
    for entities in position_map.values() {
        if entities.len() > 1 {
            // Keep the first, mark the rest as ignored
            entities_to_ignore.extend(entities.iter().skip(1));
        }
    }

    // Second pass: mark duplicates as ignored
    for (entity, _pos, mut lifecycle) in q_designations.iter_mut() {
        if entities_to_ignore.contains(&entity) {
            lifecycle.0 = DesignationState::Ignored;
        }
    }
}

pub fn designation_to_jobs_system(
    config: Res<DesignationConfig>,
    mut board: ResMut<JobBoard>,
    mut q: Query<(&crate::world::Position, &mut DesignationLifecycle), With<MineDesignation>>,
) {
    if !config.auto_jobs {
        return;
    }

    // Only process active designations and mark them consumed to prevent duplicates
    for (pos, mut lifecycle) in q.iter_mut() {
        if lifecycle.0 == DesignationState::Active {
            add_job(&mut board, JobKind::Mine { x: pos.0, y: pos.1 });
            lifecycle.0 = DesignationState::Consumed;
        }
    }
}
