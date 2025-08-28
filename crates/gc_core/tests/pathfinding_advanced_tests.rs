// Comprehensive pathfinding tests to improve coverage

use gc_core::prelude::*;

fn create_test_map(width: u32, height: u32) -> GameMap {
    // Fill with floors by default (already done by GameMap::new)
    GameMap::new(width, height)
}

fn create_map_with_walls() -> GameMap {
    let mut map = create_test_map(10, 10);
    // Create some walls to test obstacle avoidance
    for y in 2..8 {
        map.set_tile(5, y, TileKind::Wall); // Vertical wall
    }
    map
}

#[test]
fn path_service_new_creates_with_capacity() {
    let service = PathService::new(100);
    let (hits, misses) = service.stats();
    assert_eq!(hits, 0);
    assert_eq!(misses, 0);
}

#[test]
fn path_service_new_with_zero_capacity_still_works() {
    let service = PathService::new(0);
    let (hits, misses) = service.stats();
    assert_eq!(hits, 0);
    assert_eq!(misses, 0);
}

#[test]
fn path_service_cache_hits_work() {
    let mut service = PathService::new(10);
    let map = create_test_map(10, 10);

    // First request - should be a miss
    let path1 = service.get(&map, (0, 0), (2, 2));
    assert!(path1.is_some());
    let (hits, misses) = service.stats();
    assert_eq!(hits, 0);
    assert_eq!(misses, 1);

    // Same request - should be a hit
    let path2 = service.get(&map, (0, 0), (2, 2));
    assert!(path2.is_some());
    let (hits, misses) = service.stats();
    assert_eq!(hits, 1);
    assert_eq!(misses, 1);

    // Paths should be identical
    assert_eq!(path1, path2);
}

#[test]
fn path_service_batch_processing() {
    let mut service = PathService::new(10);
    let map = create_test_map(10, 10);

    let requests = vec![
        PathRequest {
            start: (0, 0),
            goal: (1, 1),
        },
        PathRequest {
            start: (2, 2),
            goal: (3, 3),
        },
        PathRequest {
            start: (0, 0),
            goal: (1, 1),
        }, // Duplicate for cache hit
    ];

    let results = service.batch(&map, &requests);
    assert_eq!(results.len(), 3);

    // All should have found paths
    for result in &results {
        assert!(result.is_some());
    }

    // Should have 1 hit and 2 misses (third request is cached)
    let (hits, misses) = service.stats();
    assert_eq!(hits, 1);
    assert_eq!(misses, 2);
}

#[test]
fn path_service_reset_stats() {
    let mut service = PathService::new(10);
    let map = create_test_map(10, 10);

    // Generate some stats
    service.get(&map, (0, 0), (1, 1));
    service.get(&map, (0, 0), (1, 1)); // Cache hit

    let (hits, misses) = service.stats();
    assert_eq!(hits, 1);
    assert_eq!(misses, 1);

    // Reset and verify
    service.reset_stats();
    let (hits, misses) = service.stats();
    assert_eq!(hits, 0);
    assert_eq!(misses, 0);
}

#[test]
fn astar_path_no_path_available() {
    let mut map = create_test_map(5, 5);
    // Create an impossible path by walling off the goal
    for x in 0..5 {
        for y in 0..5 {
            if x > 0 || y > 0 {
                map.set_tile(x, y, TileKind::Wall);
            }
        }
    }

    let result = astar_path(&map, (0, 0), (4, 4));
    assert!(result.is_none());
}

#[test]
fn astar_path_same_start_and_goal() {
    let map = create_test_map(5, 5);

    let result = astar_path(&map, (2, 2), (2, 2));
    assert!(result.is_some());

    let (path, cost) = result.unwrap();
    assert_eq!(path.len(), 1);
    assert_eq!(path[0], (2, 2));
    assert_eq!(cost, 0);
}

#[test]
fn astar_path_around_obstacles() {
    let map = create_map_with_walls();

    // Path from left side to right side, should go around wall
    let result = astar_path(&map, (0, 5), (9, 5));
    assert!(result.is_some());

    let (path, _cost) = result.unwrap();

    // Path should not go through the wall at x=5
    for &(x, y) in &path {
        if x == 5 && (2..8).contains(&y) {
            panic!("Path goes through wall at ({}, {})", x, y);
        }
    }

    // Should start and end at correct positions
    assert_eq!(path[0], (0, 5));
    assert_eq!(path[path.len() - 1], (9, 5));
}

#[test]
fn astar_path_adjacent_positions() {
    let map = create_test_map(5, 5);

    let result = astar_path(&map, (1, 1), (1, 2));
    assert!(result.is_some());

    let (path, cost) = result.unwrap();
    assert_eq!(path.len(), 2);
    assert_eq!(path[0], (1, 1));
    assert_eq!(path[1], (1, 2));
    assert_eq!(cost, 1);
}

#[test]
fn path_request_equality() {
    let req1 = PathRequest {
        start: (1, 2),
        goal: (3, 4),
    };
    let req2 = PathRequest {
        start: (1, 2),
        goal: (3, 4),
    };
    let req3 = PathRequest {
        start: (1, 2),
        goal: (3, 5),
    };

    assert_eq!(req1, req2);
    assert_ne!(req1, req3);
}

#[test]
fn path_request_debug_and_clone() {
    let req = PathRequest {
        start: (1, 2),
        goal: (3, 4),
    };
    let cloned = req;

    assert_eq!(req, cloned);

    // Test debug formatting works
    let debug_str = format!("{:?}", req);
    assert!(debug_str.contains("PathRequest"));
    assert!(debug_str.contains("start"));
    assert!(debug_str.contains("goal"));
}

#[test]
fn path_to_unreachable_area() {
    let mut map = create_test_map(10, 10);

    // Create an island at (8,8) by surrounding it with walls
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx != 0 || dy != 0 {
                map.set_tile(8 + dx, 8 + dy, TileKind::Wall);
            }
        }
    }

    let result = astar_path(&map, (0, 0), (8, 8));
    assert!(result.is_none());
}

#[test]
fn path_service_cache_eviction() {
    let mut service = PathService::new(2); // Very small cache
    let map = create_test_map(10, 10);

    // Fill cache beyond capacity
    service.get(&map, (0, 0), (1, 1)); // Entry 1
    service.get(&map, (2, 2), (3, 3)); // Entry 2
    service.get(&map, (4, 4), (5, 5)); // Entry 3 - should evict Entry 1

    // First entry should be evicted (LRU), so this should be a miss
    service.get(&map, (0, 0), (1, 1));

    let (_hits, misses) = service.stats();
    // Should have had cache hits for entries 2 and 3, but miss for entry 1 retrieval
    assert_eq!(misses, 4); // 3 initial + 1 re-fetch of evicted entry
}
