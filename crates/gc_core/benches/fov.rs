use bevy_ecs::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use gc_core::components::VisionRadius;
use gc_core::fov::{compute_visibility_system, los_visible, Visibility};
use gc_core::mapgen::MapGenerator;
use gc_core::world::{GameMap, Position, TileKind};

// Helper function to clone a GameMap
fn clone_map(map: &GameMap) -> GameMap {
    let mut new_map = GameMap::new(map.width, map.height);
    new_map.tiles = map.tiles.clone();
    new_map
}

fn bench_los_visible(c: &mut Criterion) {
    let mut group = c.benchmark_group("los_visible");

    // Create different map scenarios
    let gen = MapGenerator::new(42);
    let open_map = GameMap::new(100, 100);

    let complex_map = gen.generate(100, 100);

    let walled_map = {
        let mut map = GameMap::new(100, 100);
        // Add some walls to create line-of-sight obstacles
        for x in 10..90 {
            map.set_tile(x, 50, TileKind::Wall);
        }
        for y in 10..90 {
            map.set_tile(50, y, TileKind::Wall);
        }
        map
    };

    // Test different distances on open map
    for distance in [5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("open_map", distance),
            distance,
            |b, &distance| {
                b.iter(|| {
                    los_visible(
                        black_box(&open_map),
                        black_box(10),
                        black_box(10),
                        black_box(10 + distance),
                        black_box(10 + distance),
                    )
                })
            },
        );
    }

    // Test different distances on complex map (generated with walls/water)
    for distance in [5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("complex_map", distance),
            distance,
            |b, &distance| {
                b.iter(|| {
                    los_visible(
                        black_box(&complex_map),
                        black_box(10),
                        black_box(10),
                        black_box(10 + distance),
                        black_box(10 + distance),
                    )
                })
            },
        );
    }

    // Test line-of-sight through walls
    group.bench_function("through_walls", |b| {
        b.iter(|| {
            los_visible(
                black_box(&walled_map),
                black_box(5),
                black_box(5),
                black_box(95),
                black_box(95),
            )
        })
    });

    group.finish();
}

fn bench_compute_visibility_system(c: &mut Criterion) {
    let mut group = c.benchmark_group("compute_visibility_system");

    // Helper function to create a map and setup world with entities
    fn setup_world_with_map(
        width: u32,
        height: u32,
        num_entities: usize,
        vision_radius: i32,
    ) -> World {
        let gen = MapGenerator::new(123);
        let map = gen.generate(width, height);

        let mut world = World::new();
        world.insert_resource(map);
        world.insert_resource(Visibility::default());

        // Spawn entities with positions and vision
        for i in 0..num_entities {
            let x = (i as i32 * 5) % (width as i32 - 10) + 5;
            let y = (i as i32 * 3) % (height as i32 - 10) + 5;
            world.spawn((Position(x, y), VisionRadius(vision_radius)));
        }

        world
    }

    // Benchmark single entity with different vision radii
    for radius in [3, 8, 15, 25].iter() {
        group.bench_with_input(
            BenchmarkId::new("single_entity_medium_map", radius),
            radius,
            |b, &radius| {
                b.iter(|| {
                    let mut world = setup_world_with_map(100, 100, 1, radius);
                    let mut schedule = Schedule::default();
                    schedule.add_systems(compute_visibility_system);
                    schedule.run(black_box(&mut world));
                })
            },
        );
    }

    // Benchmark multiple entities on different map sizes
    let map_configs = [("small", 50, 50), ("medium", 100, 100), ("large", 200, 200)];

    for (map_name, width, height) in map_configs.iter() {
        for num_entities in [1, 5, 10, 20].iter() {
            // Setup world and schedule outside the timed closure
            let mut world = setup_world_with_map(*width, *height, *num_entities, 8);
            let mut schedule = Schedule::default();
            schedule.add_systems(compute_visibility_system);
            group.bench_with_input(
                BenchmarkId::new(format!("{}_map", map_name), num_entities),
                num_entities,
                |b, &_num_entities| {
                    b.iter(|| {
                        // If compute_visibility_system mutates the world, clone it here.
                        // Otherwise, reuse the same world.
                        // For safety, let's clone the world.
                        let mut world_clone = world.clone();
                        schedule.run(black_box(&mut world_clone));
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_fov_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("fov_patterns");

    // Test specific FOV patterns that might be performance-sensitive
    let gen = MapGenerator::new(456);
    let maze_map = {
        let mut map = gen.generate(80, 80);
        // Create a maze-like pattern with many walls
        for y in (0..80).step_by(4) {
            for x in (0..80).step_by(2) {
                if map.in_bounds(x, y) {
                    map.set_tile(x, y, TileKind::Wall);
                }
            }
        }
        map
    };

    let open_field = GameMap::new(80, 80); // All floor tiles

    // Benchmark visibility calculation in maze vs open field
    let maps = [("maze", &maze_map), ("open_field", &open_field)];

    for (map_name, map) in maps.into_iter() {
        group.bench_function(map_name, |b| {
            b.iter_batched(
                || {
                    let mut world = World::new();
                    world.insert_resource(clone_map(map));
                    world.insert_resource(Visibility::default());

                    // Single entity in center with large vision radius
                    world.spawn((Position(40, 40), VisionRadius(20)));

                    let mut schedule = Schedule::default();
                    schedule.add_systems(compute_visibility_system);
                    (world, schedule)
                },
                |(mut world, mut schedule)| {
                    schedule.run(black_box(&mut world));
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_los_visible,
    bench_compute_visibility_system,
    bench_fov_patterns
);
criterion_main!(benches);
