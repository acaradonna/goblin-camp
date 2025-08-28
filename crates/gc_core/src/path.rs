use crate::world::GameMap;
use lru::LruCache;
use pathfinding::prelude::astar;
use std::num::NonZeroUsize;

// A* Pathfinding System with LRU Caching
//
// This module provides efficient pathfinding using the A* algorithm with
// Manhattan distance heuristic. Includes caching to avoid redundant calculations
// for frequently requested paths.
//
// Features:
// - 4-directional movement (no diagonals)
// - LRU cache to improve performance for repeated path requests
// - Batch processing for multiple path calculations
// - Statistics tracking for cache hit/miss analysis

// Type aliases and structures for pathfinding

/// Result type for pathfinding operations
///
/// Returns `Some((path_coords, total_cost))` on success, `None` if no path exists
type PathResult = Option<(Vec<(i32, i32)>, i32)>;
/// Cache key combining start and goal coordinates: (start_x, start_y, goal_x, goal_y)
type CacheKey = (i32, i32, i32, i32);
/// LRU cache storing pathfinding results
type PathCache = LruCache<CacheKey, PathResult>;

/// Generate neighbors for A* pathfinding with 4-directional movement
/// Only returns walkable neighboring tiles based on the game map
/// Each neighbor has a movement cost of 1 (uniform cost grid)
fn neighbors(map: &GameMap, x: i32, y: i32) -> Vec<((i32, i32), i32)> {
    let mut n = Vec::with_capacity(4);
    // 4-directional movement: right, left, down, up
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for (dx, dy) in dirs {
        let nx = x + dx;
        let ny = y + dy;
        if map.is_walkable(nx, ny) {
            n.push(((nx, ny), 1));
        }
    }
    n
}

/// Find shortest path using A* algorithm with Manhattan distance heuristic
/// Returns None if no path exists, otherwise returns (path, total_cost)
/// The path includes both start and goal positions
pub fn astar_path(map: &GameMap, start: (i32, i32), goal: (i32, i32)) -> PathResult {
    astar(
        &start,
        |&(x, y)| neighbors(map, x, y),
        |&(x, y)| (x - goal.0).abs() + (y - goal.1).abs(), // Manhattan distance heuristic
        |&p| p == goal,
    )
}

/// Request structure for batch pathfinding operations
/// Encapsulates start and goal coordinates for a single pathfinding request
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathRequest {
    /// Starting position for the path
    pub start: (i32, i32),
    /// Goal position for the path
    pub goal: (i32, i32),
}

/// Pathfinding service with LRU caching for performance optimization
/// Caches computed paths to avoid redundant calculations for frequently requested routes
/// Maintains statistics for cache performance analysis
#[derive(Debug)]
pub struct PathService {
    /// LRU cache storing path results
    cache: PathCache,
    /// Number of cache hits (requests served from cache)
    hits: usize,
    /// Number of cache misses (requests requiring computation)
    misses: usize,
}

impl PathService {
    /// Create a new PathService with specified cache capacity
    /// Larger capacity means more paths cached but higher memory usage
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity.max(1)).unwrap();
        Self {
            cache: LruCache::new(cap),
            hits: 0,
            misses: 0,
        }
    }

    /// Get path from start to goal, using cache if available
    /// Automatically updates cache with new calculations
    /// Returns None if no path exists
    pub fn get(&mut self, map: &GameMap, start: (i32, i32), goal: (i32, i32)) -> PathResult {
        let key = (start.0, start.1, goal.0, goal.1);
        if let Some(v) = self.cache.get(&key) {
            self.hits += 1;
            return v.clone();
        }
        self.misses += 1;
        let v = astar_path(map, start, goal);
        self.cache.put(key, v.clone());
        v
    }

    /// Process multiple pathfinding requests in batch
    /// More efficient than individual calls for multiple paths
    /// Each request is still cached independently
    pub fn batch(&mut self, map: &GameMap, reqs: &[PathRequest]) -> Vec<PathResult> {
        let mut out = Vec::with_capacity(reqs.len());
        for r in reqs {
            out.push(self.get(map, r.start, r.goal));
        }
        out
    }

    /// Get cache performance statistics (hits, misses)
    /// Useful for optimizing cache size and analyzing path request patterns
    pub fn stats(&self) -> (usize, usize) {
        (self.hits, self.misses)
    }

    /// Reset performance statistics to zero
    /// Useful for benchmarking or periodic analysis
    pub fn reset_stats(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}
