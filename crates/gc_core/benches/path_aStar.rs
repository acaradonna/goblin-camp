use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use gc_core::mapgen::MapGenerator;
use gc_core::path::{astar_path, PathRequest, PathService};
use gc_core::world::{GameMap, TileKind};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn create_random_map(width: u32, height: u32, seed: u32) -> GameMap {
    let generator = MapGenerator::new();
    generator.generate(width, height, seed)
}

fn create_mostly_floor_map(width: u32, height: u32, wall_density: f32, seed: u32) -> GameMap {
    let mut map = GameMap::new(width, height);
    let mut rng = StdRng::seed_from_u64(seed as u64);

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            if rng.gen::<f32>() < wall_density {
                map.set_tile(x, y, TileKind::Wall);
            }
            // else remains Floor (default)
        }
    }
    map
}

fn find_valid_positions(map: &GameMap, count: usize, seed: u32) -> Vec<(i32, i32)> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut positions = Vec::new();
    let mut attempts = 0;

    while positions.len() < count && attempts < count * 100 {
        let x = rng.gen_range(0..map.width as i32);
        let y = rng.gen_range(0..map.height as i32);

        if map.is_walkable(x, y) {
            positions.push((x, y));
        }
        attempts += 1;
    }

    positions
}

fn bench_astar_single_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("astar_single_path");

    // Test different map sizes
    for &size in &[20, 50, 100] {
        let map = create_random_map(size, size, 42);
        let positions = find_valid_positions(&map, 10, 123);

        if positions.len() >= 2 {
            let start = positions[0];
            let goal = positions[1];

            group.bench_with_input(
                BenchmarkId::new("random_map", format!("{}x{}", size, size)),
                &(map, start, goal),
                |b, (map, start, goal)| {
                    b.iter(|| {
                        black_box(astar_path(
                            black_box(map),
                            black_box(*start),
                            black_box(*goal),
                        ))
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_astar_different_densities(c: &mut Criterion) {
    let mut group = c.benchmark_group("astar_wall_density");

    let size = 50u32;
    for &density in &[0.1, 0.2, 0.3, 0.4] {
        let map = create_mostly_floor_map(size, size, density, 42);
        let positions = find_valid_positions(&map, 10, 456);

        if positions.len() >= 2 {
            let start = positions[0];
            let goal = positions[positions.len() - 1]; // Use distant goal

            group.bench_with_input(
                BenchmarkId::new("wall_density", format!("{:.1}", density)),
                &(map, start, goal),
                |b, (map, start, goal)| {
                    b.iter(|| {
                        black_box(astar_path(
                            black_box(map),
                            black_box(*start),
                            black_box(*goal),
                        ))
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_astar_path_lengths(c: &mut Criterion) {
    let mut group = c.benchmark_group("astar_path_length");

    let map = create_mostly_floor_map(100, 100, 0.2, 42);
    let positions = find_valid_positions(&map, 50, 789);

    if positions.len() >= 10 {
        // Close path (start near each other)
        let close_start = positions[0];
        let close_goal = positions[1];

        // Medium path
        let med_start = positions[0];
        let med_goal = positions[positions.len() / 2];

        // Far path (corner to corner style)
        let far_start = positions[0];
        let far_goal = positions[positions.len() - 1];

        for (name, start, goal) in [
            ("close", close_start, close_goal),
            ("medium", med_start, med_goal),
            ("far", far_start, far_goal),
        ] {
            group.bench_with_input(
                BenchmarkId::new("distance", name),
                &(map.clone(), start, goal),
                |b, (map, start, goal)| {
                    b.iter(|| {
                        black_box(astar_path(
                            black_box(map),
                            black_box(*start),
                            black_box(*goal),
                        ))
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_path_service_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_service_cache");

    let map = create_random_map(50, 50, 42);
    let positions = find_valid_positions(&map, 20, 999);

    if positions.len() >= 10 {
        // Create repeated requests that should benefit from caching
        let mut requests = Vec::new();
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                requests.push(PathRequest {
                    start: positions[i],
                    goal: positions[j],
                });
            }
        }

        // Duplicate some requests to test cache hits
        let original_len = requests.len();
        for i in 0..original_len.min(10) {
            requests.push(requests[i]);
        }

        group.bench_with_input(
            BenchmarkId::new("cached_batch", requests.len()),
            &(map.clone(), requests.clone()),
            |b, (map, requests)| {
                b.iter(|| {
                    let mut service = PathService::new(100);
                    black_box(service.batch(black_box(map), black_box(requests)))
                })
            },
        );

        // Benchmark without cache (direct astar calls)
        let direct_requests = &requests[..requests.len().min(20)]; // Limit for performance
        group.bench_with_input(
            BenchmarkId::new("direct_batch", direct_requests.len()),
            &(map, direct_requests),
            |b, (map, requests)| {
                b.iter(|| {
                    let results: Vec<_> = requests
                        .iter()
                        .map(|req| {
                            black_box(astar_path(
                                black_box(map),
                                black_box(req.start),
                                black_box(req.goal),
                            ))
                        })
                        .collect();
                    black_box(results)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_astar_single_path,
    bench_astar_different_densities,
    bench_astar_path_lengths,
    bench_path_service_cache
);
criterion_main!(benches);
