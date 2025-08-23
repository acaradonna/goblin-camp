//! Inventory system for agents carrying items

use crate::components::Inventory;
use crate::world::Position;
use bevy_ecs::prelude::*;

/// Pick up an item from the world into an agent's inventory
/// Returns true if successful, false if agent already carries something or item doesn't exist
pub fn pick_up_item(world: &mut World, agent_entity: Entity, item_entity: Entity) -> bool {
    // First check if item exists and agent has an inventory
    let item_exists = world.get::<Position>(item_entity).is_some();
    if !item_exists {
        return false;
    }

    // Check if agent has inventory and it's empty
    if let Some(mut inventory) = world.get_mut::<Inventory>(agent_entity) {
        if inventory.0.is_some() {
            return false; // Already carrying something
        }

        // Move item to inventory
        inventory.0 = Some(item_entity);
        true
    } else {
        false // Agent doesn't have inventory component
    }
}

/// Put down an item from an agent's inventory into the world at a specific position
/// Returns true if successful, false if agent doesn't carry anything
pub fn put_down_item(world: &mut World, agent_entity: Entity, world_position: (i32, i32)) -> bool {
    // Check if agent has inventory with an item
    if let Some(inventory) = world.get_mut::<Inventory>(agent_entity) {
        if let Some(item_entity) = inventory.0 {
            // First drop the inventory borrow, then try to update the item position
            drop(inventory);

            // Try to set item position in world
            if let Some(mut position) = world.get_mut::<Position>(item_entity) {
                position.0 = world_position.0;
                position.1 = world_position.1;

                // Now get inventory back and clear it
                if let Some(mut inventory) = world.get_mut::<Inventory>(agent_entity) {
                    inventory.0 = None;
                }
                true
            } else {
                // Item entity is invalid, do not clear inventory
                false
            }
        } else {
            false // Not carrying anything
        }
    } else {
        false // Agent doesn't have inventory component
    }
}

/// Check if an agent is carrying any item
pub fn is_carrying_item(world: &World, agent_entity: Entity) -> bool {
    world
        .get::<Inventory>(agent_entity)
        .map(|inventory| inventory.0.is_some())
        .unwrap_or(false)
}

/// Get the entity of the item being carried, if any
pub fn get_carried_item(world: &World, agent_entity: Entity) -> Option<Entity> {
    world
        .get::<Inventory>(agent_entity)
        .and_then(|inventory| inventory.0)
}
