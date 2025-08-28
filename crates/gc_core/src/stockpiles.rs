use crate::components::{Stockpile, ZoneBounds};
use crate::world::Position;
use bevy_ecs::prelude::*;

/// Stockpile System for Item Storage and Organization
///
/// This module provides functionality for creating and managing stockpiles,
/// which are designated storage areas where items can be hauled and organized.
/// Stockpiles are zone-based entities that accept items within their spatial bounds.
/// Bundle for creating a complete stockpile entity
/// Combines all necessary components for a functional stockpile
#[derive(Bundle)]
pub struct StockpileBundle {
    /// Stockpile component defining what items are accepted
    pub stockpile: Stockpile,
    /// Position component for the stockpile's center point
    pub position: Position,
    /// Zone bounds defining the spatial area of the stockpile
    pub bounds: ZoneBounds,
}

impl StockpileBundle {
    /// Create a new stockpile with specified rectangular bounds
    /// The position is automatically set to the center of the bounds
    /// All item types are accepted by default (accepts: None)
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        let center = ((min_x + max_x) / 2, (min_y + max_y) / 2);
        Self {
            stockpile: Stockpile { accepts: None }, // Accept all item types
            position: Position(center.0, center.1),
            bounds: ZoneBounds::new(min_x, min_y, max_x, max_y),
        }
    }
}

/// Find the nearest stockpile to a given position within a world
/// Used by hauling systems to determine where items should be transported
/// Returns (entity, distance_squared) of the nearest stockpile, or None if no stockpiles exist
/// Uses squared distance to avoid expensive square root calculations
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
/// Useful for determining if an item is already in a stockpile
/// Returns true if the position overlaps with any stockpile bounds
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
/// Returns a vector of entity IDs for all stockpiles overlapping the position
/// Useful when stockpiles overlap or when you need to access all relevant stockpiles
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
