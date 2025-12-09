use crate::components::{
    ConstructionDesignation, DesignationLifecycle, DesignationState, MaterialReservation,
};
use crate::jobs::{add_job, JobBoard, JobKind};
use crate::systems::DeterministicRng;
use bevy_ecs::prelude::*;
use std::collections::HashMap;

/// Designation System for Player Input and Job Creation
///
/// This module implements the designation system, which allows players to mark
/// areas for specific tasks (mining, construction, etc.). Designations are
/// converted into jobs that workers can execute.
///
/// The system includes deduplication to prevent multiple jobs for the same location
/// and lifecycle management to track designation processing.
/// Component marking an entity as a mining designation
/// Mining designations mark tiles that should be converted from Wall to Floor
/// These are typically created by player input or scripted scenarios
#[derive(Component, Debug)]
pub struct MineDesignation;

/// Bundle for creating complete designation entities
/// Provides a convenient way to spawn designations with all required components
#[derive(Bundle)]
pub struct DesignationBundle {
    /// World position of the designation
    pub pos: crate::world::Position,
    /// Type of designation (currently only mining)
    pub kind: MineDesignation,
    /// Lifecycle tracking for deduplication and processing
    pub lifecycle: DesignationLifecycle,
}

impl Default for DesignationBundle {
    /// Create a default mining designation at (0, 0)
    fn default() -> Self {
        Self {
            pos: crate::world::Position(0, 0),
            kind: MineDesignation,
            lifecycle: DesignationLifecycle::default(),
        }
    }
}

/// Configuration resource for designation behavior
/// Controls how designations are processed and converted to jobs
#[derive(Resource, Default, Debug)]
pub struct DesignationConfig {
    /// Whether to automatically create jobs from designations
    /// When true, Active designations are automatically converted to jobs
    /// When false, designations remain in place without creating jobs
    pub auto_jobs: bool,
}

/// System that deduplicates designations by marking later ones at the same position as Ignored
/// Prevents multiple jobs from being created for the same location
/// Uses a two-pass approach to avoid borrowing conflicts while maintaining deterministic behavior
///
/// The system preserves the first designation at each position and marks subsequent ones as Ignored.
/// Only Active designations are considered for deduplication - Ignored and Consumed designations are left unchanged.
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
            // Keep the first entity at this position, mark the rest as ignored
            // This ensures deterministic behavior regardless of query order
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

/// System that converts active designations into jobs on the job board
/// Processes designations marked as Active and creates corresponding jobs
/// Marks processed designations as Consumed to prevent duplicate job creation
///
/// Only runs when auto_jobs is enabled in DesignationConfig
/// Uses deterministic RNG to ensure reproducible job IDs
pub fn designation_to_jobs_system(
    config: Res<DesignationConfig>,
    mut board: ResMut<JobBoard>,
    mut rng: ResMut<DeterministicRng>,
    mut q: Query<(&crate::world::Position, &mut DesignationLifecycle), With<MineDesignation>>,
) {
    if !config.auto_jobs {
        return;
    }

    // Only process active designations and mark them consumed to prevent duplicates
    for (pos, mut lifecycle) in q.iter_mut() {
        if lifecycle.0 == DesignationState::Active {
            // Create a mining job for this designation
            add_job(
                &mut board,
                JobKind::Mine { x: pos.0, y: pos.1 },
                &mut rng.job_rng,
            );
            // Mark designation as consumed so it won't create another job
            lifecycle.0 = DesignationState::Consumed;
        }
    }
}

/// System that converts active construction designations into build jobs
/// Creates jobs for constructing walls, floors, and doors based on designation type
/// Reserves materials needed for construction by adding MaterialReservation component
/// Validates that all occupied cells meet placement requirements before creating jobs
///
/// Only runs when auto_jobs is enabled in DesignationConfig
/// Uses deterministic RNG to ensure reproducible job IDs
pub fn construction_designation_to_jobs_system(
    config: Res<DesignationConfig>,
    mut board: ResMut<JobBoard>,
    mut rng: ResMut<DeterministicRng>,
    mut commands: Commands,
    map: Res<crate::world::GameMap>,
    mut q: Query<(Entity, &ConstructionDesignation, &mut DesignationLifecycle)>,
) {
    if !config.auto_jobs {
        return;
    }

    // Process active construction designations
    for (entity, designation, mut lifecycle) in q.iter_mut() {
        if lifecycle.0 == DesignationState::Active {
            // Validate all occupied cells meet placement requirements
            let occupied = designation.occupied_cells();
            let mut all_valid = true;

            for (x, y) in &occupied {
                // Check if tile is within map bounds
                if !map.in_bounds(*x, *y) {
                    all_valid = false;
                    break;
                }

                // Validate tile type based on what we're building
                let tile = map.get_tile(*x, *y);
                let valid_tile = match tile {
                    Some(tile_kind) => match designation.buildable {
                        // Walls and doors can only be built on Floor tiles
                        crate::components::BuildableKind::Wall
                        | crate::components::BuildableKind::Door => {
                            matches!(tile_kind, crate::world::TileKind::Floor)
                        }
                        // Floors can be built on any passable tile (Floor, Water, Lava but not Wall)
                        crate::components::BuildableKind::Floor => {
                            !matches!(tile_kind, crate::world::TileKind::Wall)
                        }
                    },
                    None => false, // Out of bounds
                };

                if !valid_tile {
                    all_valid = false;
                    break;
                }
            }

            // Only create job if all tiles are valid
            if all_valid {
                // Reserve materials by adding MaterialReservation component
                // Initially empty - materials will be added when gathered by workers
                commands.entity(entity).insert(MaterialReservation::new());

                // Create a build job for this designation
                add_job(
                    &mut board,
                    JobKind::Build {
                        x: designation.position.0,
                        y: designation.position.1,
                        designation: entity,
                    },
                    &mut rng.job_rng,
                );

                // Mark designation as consumed so it won't create another job
                lifecycle.0 = DesignationState::Consumed;
            } else {
                // Mark as ignored if placement is invalid
                lifecycle.0 = DesignationState::Ignored;
            }
        }
    }
}
