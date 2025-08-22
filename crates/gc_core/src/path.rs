use crate::world::GameMap;
use lru::LruCache;
use pathfinding::prelude::astar;
use std::num::NonZeroUsize;

fn neighbors(map: &GameMap, x: i32, y: i32) -> Vec<((i32,i32), i32)> {
    let mut n = Vec::with_capacity(4);
    let dirs = [(1,0),(-1,0),(0,1),(0,-1)];
    for (dx,dy) in dirs { let nx = x+dx; let ny = y+dy; if map.is_walkable(nx, ny) { n.push(((nx,ny), 1)); } }
    n
}

pub fn astar_path(map: &GameMap, start: (i32,i32), goal: (i32,i32)) -> Option<(Vec<(i32,i32)>, i32)> {
    astar(&start, |&(x,y)| neighbors(map,x,y), |&(x,y)| (x-goal.0).abs()+(y-goal.1).abs(), |&p| p==goal)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathRequest { pub start: (i32,i32), pub goal: (i32,i32) }

// A small service to batch and cache pathfinding with an LRU cache.
#[derive(Debug)]
pub struct PathService {
    cache: LruCache<(i32,i32,i32,i32), Option<(Vec<(i32,i32)>, i32)>>,
    hits: usize,
    misses: usize,
}

impl PathService {
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity.max(1)).unwrap();
        Self { cache: LruCache::new(cap), hits: 0, misses: 0 }
    }

    pub fn get(&mut self, map: &GameMap, start: (i32,i32), goal: (i32,i32)) -> Option<(Vec<(i32,i32)>, i32)> {
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

    pub fn batch(&mut self, map: &GameMap, reqs: &[PathRequest]) -> Vec<Option<(Vec<(i32,i32)>, i32)>> {
        let mut out = Vec::with_capacity(reqs.len());
        for r in reqs {
            out.push(self.get(map, r.start, r.goal));
        }
        out
    }

    pub fn stats(&self) -> (usize, usize) { (self.hits, self.misses) }
    pub fn reset_stats(&mut self) { self.hits = 0; self.misses = 0; }
}
