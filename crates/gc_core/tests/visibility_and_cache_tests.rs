use bevy_ecs::prelude::*;
use gc_core::prelude::*;

#[test]
fn path_cache_hits_on_repeat_requests() {
    let gen = MapGenerator::new(123);
    let map = gen.generate(32, 16);
    let mut svc = gc_core::path::PathService::new(8);

    let s = (1,1); let g = (30,14);
    let _ = svc.get(&map, s, g);
    let _ = svc.get(&map, s, g);
    let (hits, misses) = svc.stats();
    assert!(hits >= 1, "expected at least one cache hit");
    assert!(misses >= 1, "expected at least one cache miss");
}

#[test]
fn visibility_resource_contains_entity_tiles() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(16, 16));
    world.insert_resource(gc_core::fov::Visibility::default());
    let e = world.spawn((Position(2,2), VisionRadius(3))).id();

    let mut schedule = Schedule::default();
    schedule.add_systems((gc_core::fov::compute_visibility_system,));
    schedule.run(&mut world);

    let vis = world.resource::<gc_core::fov::Visibility>();
    let tiles = vis.per_entity.get(&e).expect("entity visibility missing");
    assert!(tiles.contains(&(2,2)));
}
