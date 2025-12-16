use bevy_ecs::prelude::*;
use gc_core::components::ItemType;
use gc_core::prelude::*;
use gc_core::world::TileKind;

#[test]
fn los_through_wall_blocks() {
    let gen = MapGenerator::new();
    let mut map = gen.generate(20, 10, 1);
    // place a wall between (1,1) and (18,8)
    map.set_tile(10, 5, TileKind::Wall);
    assert!(!los_visible(&map, 1, 1, 18, 8));
}

#[test]
fn a_star_finds_path_on_floor() {
    let gen = MapGenerator::new();
    let map = gen.generate(30, 15, 2);
    let start = (1, 1);
    let goal = (28, 13);
    let path = astar_path(&map, start, goal);
    assert!(path.is_some());
}

#[test]
fn save_load_roundtrip() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(8, 8));
    // Insert Time and RNG with specific values to validate roundtrip
    world.insert_resource(gc_core::systems::Time {
        tick_ms: 200,
        ticks: 42,
    });
    world.insert_resource(gc_core::systems::DeterministicRng::new(12345));
    world.spawn((Name("G".into()), Position(1, 1), Velocity(0, 0)));
    let save = save_world(&mut world);
    let json = serde_json::to_string(&save).unwrap();

    let mut w2 = World::new();
    load_world(serde_json::from_str(&json).unwrap(), &mut w2);
    // Validate time and RNG seed restoration with exact values
    let time = w2.resource::<gc_core::systems::Time>();
    assert_eq!(time.tick_ms, 200);
    assert_eq!(time.ticks, 42);
    let rng = w2.resource::<gc_core::systems::DeterministicRng>();
    assert_eq!(rng.master_seed, 12345);
    let mut q = w2.query::<(&Name, &Position)>();
    let got: Vec<_> = q.iter(&w2).map(|(n, p)| (n.0.clone(), p.0, p.1)).collect();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, "G");
}

#[test]
fn item_entity_creation() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(10, 10));

    // Spawn a stone item at position (5, 5)
    let _stone_entity = world
        .spawn((
            Name("Stone Chunk".into()),
            Position(5, 5),
            Item::stone(),
            Carriable,
        ))
        .id();

    // Verify the item was created with correct components
    let mut q = world.query::<(&Name, &Position, &Item, &Carriable)>();
    let items: Vec<_> = q.iter(&world).collect();
    assert_eq!(items.len(), 1);

    let (name, pos, item, _carriable) = items[0];
    assert_eq!(name.0, "Stone Chunk");
    assert_eq!(pos.0, 5);
    assert_eq!(pos.1, 5);
    assert_eq!(item.item_type, ItemType::Stone);
}

#[test]
fn item_save_load_roundtrip() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(10, 10));

    // Create a stone item
    world.spawn((
        Name("Stone".into()),
        Position(3, 4),
        Item::stone(),
        Carriable,
    ));

    // Save and reload
    let save = save_world(&mut world);
    let json = serde_json::to_string(&save).unwrap();

    let mut w2 = World::new();
    load_world(serde_json::from_str(&json).unwrap(), &mut w2);

    // Verify the item was preserved
    let mut q = w2.query::<(&Name, &Position, &Item, &Carriable)>();
    let items: Vec<_> = q.iter(&w2).collect();
    assert_eq!(items.len(), 1);

    let (name, pos, item, _carriable) = items[0];
    assert_eq!(name.0, "Stone");
    assert_eq!(pos.0, 3);
    assert_eq!(pos.1, 4);
    assert_eq!(item.item_type, ItemType::Stone);
}

#[test]
fn load_legacy_save_without_determinism_fields() {
    // Simulate old save format (pre-determinism fields)
    let old_format_json = r#"{
        "width": 5,
        "height": 5,
        "tiles": ["Floor", "Floor", "Floor", "Floor", "Floor",
                  "Floor", "Floor", "Floor", "Floor", "Floor",
                  "Floor", "Floor", "Floor", "Floor", "Floor",
                  "Floor", "Floor", "Floor", "Floor", "Floor",
                  "Floor", "Floor", "Floor", "Floor", "Floor"],
        "entities": [
            {"name": "TestGoblin", "pos": [2, 2], "vel": [0, 0], "item_type": null, "carriable": false}
        ]
    }"#;

    let mut world = World::new();
    let save: gc_core::save::SaveGame = serde_json::from_str(old_format_json).unwrap();
    load_world(save, &mut world);

    // Verify defaults are applied
    let time = world.resource::<gc_core::systems::Time>();
    assert_eq!(time.tick_ms, 100); // default
    assert_eq!(time.ticks, 0); // default
    let rng = world.resource::<gc_core::systems::DeterministicRng>();
    assert_eq!(rng.master_seed, 0); // default

    // Verify entity was loaded
    let mut q = world.query::<(&Name, &Position)>();
    let entities: Vec<_> = q.iter(&world).collect();
    assert_eq!(entities.len(), 1);
    assert_eq!(entities[0].0 .0, "TestGoblin");
}
