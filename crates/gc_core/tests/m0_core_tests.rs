use bevy_ecs::prelude::*;
use gc_core::prelude::*;
use gc_core::world::TileKind;

#[test]
fn los_through_wall_blocks() {
    let gen = MapGenerator::new(1);
    let mut map = gen.generate(20, 10);
    // place a wall between (1,1) and (18,8)
    map.set_tile(10, 5, TileKind::Wall);
    assert_eq!(los_visible(&map, 1, 1, 18, 8), false);
}

#[test]
fn a_star_finds_path_on_floor() {
    let gen = MapGenerator::new(2);
    let map = gen.generate(30, 15);
    let start = (1, 1);
    let goal = (28, 13);
    let path = astar_path(&map, start, goal);
    assert!(path.is_some());
}

#[test]
fn save_load_roundtrip() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(8, 8));
    world.spawn((Name("G".into()), Position(1, 1), Velocity(0, 0)));
    let save = save_world(&mut world);
    let json = serde_json::to_string(&save).unwrap();

    let mut w2 = World::new();
    load_world(serde_json::from_str(&json).unwrap(), &mut w2);
    let mut q = w2.query::<(&Name, &Position)>();
    let got: Vec<_> = q.iter(&w2).map(|(n, p)| (n.0.clone(), p.0, p.1)).collect();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, "G");
}
