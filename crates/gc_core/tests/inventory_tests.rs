//! Integration tests for inventory system

use bevy_ecs::prelude::*;
use gc_core::prelude::*;

#[test]
fn inventory_component_defaults_to_empty() {
    let mut world = World::new();

    // Create an agent with inventory
    let agent = world
        .spawn((Goblin, Carrier, Inventory::default(), Position(5, 5)))
        .id();

    // Check that inventory is empty by default
    let inventory = world.get::<Inventory>(agent).unwrap();
    assert!(inventory.0.is_none());
}

#[test]
fn pick_up_item_success() {
    let mut world = World::new();

    // Create an agent with empty inventory
    let agent = world
        .spawn((Goblin, Carrier, Inventory::default(), Position(5, 5)))
        .id();

    // Create an item entity at position
    let item = world
        .spawn((Position(3, 3), Name("Stone".to_string())))
        .id();

    // Pick up the item
    let success = pick_up_item(&mut world, agent, item);

    assert!(success, "Pick up should succeed");

    // Check that agent now carries the item
    let inventory = world.get::<Inventory>(agent).unwrap();
    assert_eq!(inventory.0, Some(item));
}

#[test]
fn pick_up_item_fails_when_already_carrying() {
    let mut world = World::new();

    // Create an item entity for the first pickup
    let first_item = world
        .spawn((Position(2, 2), Name("First Stone".to_string())))
        .id();

    // Create an agent already carrying an item
    let agent = world
        .spawn((Goblin, Carrier, Inventory(Some(first_item)), Position(5, 5)))
        .id();

    // Create another item entity
    let second_item = world
        .spawn((Position(3, 3), Name("Second Stone".to_string())))
        .id();

    // Try to pick up the second item (should fail)
    let success = pick_up_item(&mut world, agent, second_item);

    assert!(
        !success,
        "Pick up should fail when already carrying something"
    );

    // Check that agent still carries the first item
    let inventory = world.get::<Inventory>(agent).unwrap();
    assert_eq!(inventory.0, Some(first_item));
}

#[test]
fn put_down_item_success() {
    let mut world = World::new();

    // Create an item entity
    let item = world
        .spawn((
            Position(0, 0), // Initial position (will be updated)
            Name("Stone".to_string()),
        ))
        .id();

    // Create an agent carrying the item
    let agent = world
        .spawn((Goblin, Carrier, Inventory(Some(item)), Position(5, 5)))
        .id();

    // Put down the item at a new position
    let target_pos = (10, 15);
    let success = put_down_item(&mut world, agent, target_pos);

    assert!(success, "Put down should succeed");

    // Check that agent no longer carries anything
    let inventory = world.get::<Inventory>(agent).unwrap();
    assert!(inventory.0.is_none());

    // Check that item is at the new position
    let item_position = world.get::<Position>(item).unwrap();
    assert_eq!(item_position.0, target_pos.0);
    assert_eq!(item_position.1, target_pos.1);
}

#[test]
fn put_down_item_fails_when_not_carrying() {
    let mut world = World::new();

    // Create an agent with empty inventory
    let agent = world
        .spawn((Goblin, Carrier, Inventory::default(), Position(5, 5)))
        .id();

    // Try to put down an item when not carrying anything
    let target_pos = (10, 15);
    let success = put_down_item(&mut world, agent, target_pos);

    assert!(!success, "Put down should fail when not carrying anything");
}

#[test]
fn is_carrying_item_check() {
    let mut world = World::new();

    // Create an item entity
    let item = world
        .spawn((Position(2, 2), Name("Stone".to_string())))
        .id();

    // Create an agent with empty inventory
    let empty_agent = world
        .spawn((Goblin, Carrier, Inventory::default(), Position(5, 5)))
        .id();

    // Create an agent carrying an item
    let carrying_agent = world
        .spawn((Goblin, Carrier, Inventory(Some(item)), Position(6, 6)))
        .id();

    // Check carrying status
    assert!(!is_carrying_item(&world, empty_agent));
    assert!(is_carrying_item(&world, carrying_agent));
}

#[test]
fn get_carried_item_check() {
    let mut world = World::new();

    // Create an item entity
    let item = world
        .spawn((Position(2, 2), Name("Stone".to_string())))
        .id();

    // Create an agent with empty inventory
    let empty_agent = world
        .spawn((Goblin, Carrier, Inventory::default(), Position(5, 5)))
        .id();

    // Create an agent carrying an item
    let carrying_agent = world
        .spawn((Goblin, Carrier, Inventory(Some(item)), Position(6, 6)))
        .id();

    // Check carried item
    assert_eq!(get_carried_item(&world, empty_agent), None);
    assert_eq!(get_carried_item(&world, carrying_agent), Some(item));
}
