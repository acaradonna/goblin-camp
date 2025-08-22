use crate::components::{Stockpile, ZoneBounds};
use crate::world::Position;
use bevy_ecs::prelude::*;

/// Bundle for creating a stockpile entity
#[derive(Bundle)]
pub struct StockpileBundle {
    pub stockpile: Stockpile,
    pub position: Position,
    pub bounds: ZoneBounds,
}

impl StockpileBundle {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        let center = ((min_x + max_x) / 2, (min_y + max_y) / 2);
        Self {
            stockpile: Stockpile { accepts_any: true },
            position: Position(center.0, center.1),
            bounds: ZoneBounds::new(min_x, min_y, max_x, max_y),
        }
    }
}

/// Find the nearest stockpile to a given position within a world
/// Returns (entity, distance_squared) of the nearest stockpile, or None if no stockpiles exist
pub fn find_nearest_stockpile(
    world: &mut World,
    target_x: i32,
    target_y: i32,
) -> Option<(Entity, i32)> {
    let mut nearest: Option<(Entity, i32)> = None;
    let mut query = world.query_filtered::<(Entity, &Position, &ZoneBounds), With<Stockpile>>();

    for (entity, position, _bounds) in query.iter(world) {
        let dx = position.0 - target_x;
        let dy = position.1 - target_y;
        let distance_squared = dx * dx + dy * dy;

        match nearest {
            None => nearest = Some((entity, distance_squared)),
            Some((_, current_dist)) if distance_squared < current_dist => {
                nearest = Some((entity, distance_squared));
            }
            _ => {}
        }
    }

    nearest
}

/// Check if a position is within any stockpile zone
pub fn position_in_stockpile(world: &mut World, x: i32, y: i32) -> bool {
    let mut query = world.query_filtered::<&ZoneBounds, With<Stockpile>>();

    for bounds in query.iter(world) {
        if bounds.contains(x, y) {
            return true;
        }
    }
    false
}

/// Find all stockpiles that contain a given position
pub fn find_stockpiles_at_position(world: &mut World, x: i32, y: i32) -> Vec<Entity> {
    let mut query = world.query_filtered::<(Entity, &ZoneBounds), With<Stockpile>>();

    query
        .iter(world)
        .filter_map(|(entity, bounds)| {
            if bounds.contains(x, y) {
                Some(entity)
            } else {
                None
            }
        })
        .collect()
}
