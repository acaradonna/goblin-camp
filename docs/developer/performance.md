# ⚡ Performance Guide

> *Optimization strategies and performance best practices for Goblin Camp*

This guide provides comprehensive performance optimization techniques for the ECS-based simulation, covering profiling, memory management, and system-level optimizations.

## 🎯 Quick Navigation

- [Performance Philosophy](#performance-philosophy) - Our optimization approach
- [Profiling and Measurement](#profiling-and-measurement) - Tools and techniques
- [ECS Performance Patterns](#ecs-performance-patterns) - System optimization strategies
- [Memory Management](#memory-management) - Efficient resource usage
- [Common Bottlenecks](#common-bottlenecks) - Known performance issues
- [Optimization Checklist](#optimization-checklist) - Systematic improvement guide

---

## 🎯 Performance Philosophy

### Core Principles

**Measure First**: Never optimize without profiling data
**Determinism Preserved**: Optimizations must not break simulation determinism
**Readability Maintained**: Performance improvements should not sacrifice code clarity
**Incremental Approach**: Small, measured improvements over large rewrites

### Performance Goals

**Target Metrics**:
- 1000+ entities at 60fps
- Sub-millisecond system execution times
- Minimal memory allocation during gameplay
- Consistent frame times (low variance)

**Acceptable Trade-offs**:
- Memory usage for CPU performance
- Code complexity for critical path optimization
- Preprocessing cost for runtime efficiency

---

## 📊 Profiling and Measurement

### Profiling Tools

**Built-in Performance Monitoring**:

```rust
// src/performance.rs
use std::time::Instant;
use std::collections::HashMap;

pub struct PerformanceMonitor {
    system_times: HashMap<&'static str, Vec<f64>>,
    frame_start: Option<Instant>,
}

impl PerformanceMonitor {
    pub fn start_frame(&mut self) {
        self.frame_start = Some(Instant::now());
    }
    
    pub fn end_frame(&mut self) {
        if let Some(start) = self.frame_start.take() {
            let frame_time = start.elapsed().as_secs_f64() * 1000.0; // ms
            self.system_times.entry("total_frame")
                .or_insert_with(Vec::new)
                .push(frame_time);
        }
    }
    
    pub fn measure_system<F>(&mut self, name: &'static str, f: F) 
    where F: FnOnce()
    {
        let start = Instant::now();
        f();
        let duration = start.elapsed().as_secs_f64() * 1000.0; // ms
        
        self.system_times.entry(name)
            .or_insert_with(Vec::new)
            .push(duration);
    }
    
    pub fn report(&self) {
        for (system, times) in &self.system_times {
            let avg = times.iter().sum::<f64>() / times.len() as f64;
            let max = times.iter().fold(0.0f64, |a, &b| a.max(b));
            println!("{}: avg={:.3}ms, max={:.3}ms", system, avg, max);
        }
    }
}
```

**System-Level Monitoring**:

```rust
pub fn performance_monitoring_system(
    mut monitor: ResMut<PerformanceMonitor>,
    time: Res<Time>,
) {
    if time.ticks % 60 == 0 { // Report every 60 ticks
        monitor.report();
    }
}

// Usage in systems
pub fn monitored_job_assignment_system(
    mut monitor: ResMut<PerformanceMonitor>,
    // ... other parameters
) {
    monitor.measure_system("job_assignment", || {
        // Actual system logic here
        job_assignment_logic(/* ... */);
    });
}
```

### External Profiling

**Using `perf` (Linux)**:

```bash
# Profile the application
cargo build --release
perf record --call-graph=dwarf ./target/release/gc_cli

# Analyze results
perf report
perf annotate

# Generate flamegraph
cargo install flamegraph
cargo flamegraph --bin gc_cli
```

**Using `cargo-profiling`**:

```bash
# Install profiling tools
cargo install cargo-profiling

# Profile with different tools
cargo profiling callgrind --bin gc_cli
cargo profiling valgrind --bin gc_cli
```

### Memory Profiling

**Heap Analysis**:

```rust
// Add to Cargo.toml for debug builds
[dependencies]
dhat = { version = "0.3", optional = true }

// In main.rs
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    
    // Run simulation
    run_simulation();
}
```

**Memory Usage Tracking**:

```rust
pub fn memory_monitoring_system(
    time: Res<Time>,
    entities: Query<Entity>,
    job_board: Res<JobBoard>,
) {
    if time.ticks % 300 == 0 { // Every 5 seconds at 60fps
        let entity_count = entities.iter().count();
        let job_count = job_board.jobs.len();
        
        println!("Memory usage - Entities: {}, Jobs: {}", 
                entity_count, job_count);
        
        // Check for memory leaks
        if entity_count > 10000 {
            eprintln!("WARNING: High entity count detected");
        }
    }
}
```

---

## ⚙️ ECS Performance Patterns

### Query Optimization

**Use Specific Queries**:

```rust
// Bad: Over-broad query
fn slow_system(all_entities: Query<(Entity, &Position, Option<&Miner>)>) {
    for (entity, pos, miner) in all_entities.iter() {
        if miner.is_some() {
            // Process miner
        }
    }
}

// Good: Targeted query
fn fast_system(miners: Query<(Entity, &Position), With<Miner>>) {
    for (entity, pos) in miners.iter() {
        // Process miner directly
    }
}
```

**Filter Components Efficiently**:

```rust
// Combine filters for better performance
fn optimized_worker_system(
    // Only workers without jobs
    available_workers: Query<Entity, (With<Worker>, Without<AssignedJob>)>,
    
    // Only miners with jobs  
    active_miners: Query<&Position, (With<Miner>, With<AssignedJob>)>,
    
    // Items that can be carried and aren't being carried
    loose_items: Query<&Position, (With<Carriable>, Without<Inventory>)>,
) {
    // Efficient iteration over specific subsets
}
```

### System Organization

**Batch Similar Operations**:

```rust
// Good: Process all mining operations together
pub fn mining_batch_system(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    miners: Query<&Position, With<ActiveMiningJob>>,
) {
    let mut tiles_to_update = Vec::new();
    let mut items_to_spawn = Vec::new();
    
    // Collect all changes first
    for pos in miners.iter() {
        if map.get_tile(pos.0, pos.1) == TileKind::Wall {
            tiles_to_update.push(*pos);
            items_to_spawn.push(*pos);
        }
    }
    
    // Apply changes in batches
    for pos in tiles_to_update {
        map.set_tile(pos.0, pos.1, TileKind::Floor);
    }
    
    for pos in items_to_spawn {
        commands.spawn((Item::stone(), Position(pos.0, pos.1)));
    }
}
```

**Minimize Resource Conflicts**:

```rust
// Use ParamSet to avoid borrowing conflicts
pub fn complex_hauling_system(
    mut param_set: ParamSet<(
        Query<&mut Position, With<Carrier>>,
        Query<&mut Position, With<Item>>,
        Query<&Inventory, With<Carrier>>,
    )>,
) {
    // Phase 1: Update carrier positions
    let carrier_updates = {
        let carriers = param_set.p0();
        collect_carrier_movements(carriers)
    };
    
    // Phase 2: Update item positions  
    let item_updates = {
        let items = param_set.p1();
        collect_item_movements(items, &carrier_updates)
    };
    
    // Phase 3: Apply updates
    apply_position_updates(carrier_updates, item_updates);
}
```

### Cache-Friendly Data Access

**Spatial Locality**:

```rust
// Group entities by spatial regions for better cache performance
pub struct SpatialGrid {
    cells: HashMap<(i32, i32), Vec<Entity>>,
    cell_size: i32,
}

impl SpatialGrid {
    pub fn update(&mut self, positions: Query<(Entity, &Position)>) {
        self.cells.clear();
        
        for (entity, pos) in positions.iter() {
            let cell_x = pos.0 / self.cell_size;
            let cell_y = pos.1 / self.cell_size;
            
            self.cells.entry((cell_x, cell_y))
                .or_insert_with(Vec::new)
                .push(entity);
        }
    }
    
    pub fn entities_near(&self, pos: &Position, radius: i32) -> Vec<Entity> {
        let min_cell_x = (pos.0 - radius) / self.cell_size;
        let max_cell_x = (pos.0 + radius) / self.cell_size;
        let min_cell_y = (pos.1 - radius) / self.cell_size;
        let max_cell_y = (pos.1 + radius) / self.cell_size;
        
        let mut result = Vec::new();
        for x in min_cell_x..=max_cell_x {
            for y in min_cell_y..=max_cell_y {
                if let Some(entities) = self.cells.get(&(x, y)) {
                    result.extend(entities);
                }
            }
        }
        result
    }
}
```

---

## 🧠 Memory Management

### Reducing Allocations

**Pre-allocate Collections**:

```rust
pub struct JobBoard {
    jobs: HashMap<JobId, Job>,
    // Pre-allocated work vectors to avoid allocations
    assignment_candidates: Vec<Entity>,
    job_queue: VecDeque<JobId>,
}

impl JobBoard {
    pub fn assign_jobs(&mut self, workers: &Query<Entity, With<Worker>>) {
        // Reuse pre-allocated vector
        self.assignment_candidates.clear();
        self.assignment_candidates.extend(workers.iter());
        
        // Sort workers by priority without allocating
        self.assignment_candidates.sort_by_key(|&entity| {
            // Priority calculation
            0
        });
        
        // Process assignments using pre-allocated storage
        for &worker in &self.assignment_candidates {
            if let Some(job_id) = self.job_queue.pop_front() {
                self.assign_job(worker, job_id);
            }
        }
    }
}
```

**Object Pooling**:

```rust
pub struct EntityPool {
    available_items: Vec<Entity>,
    available_workers: Vec<Entity>,
}

impl EntityPool {
    pub fn spawn_item(&mut self, commands: &mut Commands, item_type: Item) -> Entity {
        if let Some(entity) = self.available_items.pop() {
            // Reuse existing entity
            commands.entity(entity).insert(item_type);
            entity
        } else {
            // Create new entity
            commands.spawn(item_type).id()
        }
    }
    
    pub fn despawn_item(&mut self, entity: Entity, commands: &mut Commands) {
        // Remove components but keep entity for reuse
        commands.entity(entity).remove::<Item>();
        self.available_items.push(entity);
    }
}
```

### Efficient Data Structures

**Compact Component Storage**:

```rust
// Use smaller data types where possible
#[derive(Component)]
pub struct CompactPosition {
    x: u16, // Instead of i32 if coordinates fit
    y: u16,
}

// Pack multiple flags into single byte
#[derive(Component)]
pub struct EntityFlags {
    flags: u8, // Can represent 8 boolean flags
}

impl EntityFlags {
    pub fn is_miner(&self) -> bool { self.flags & 0x01 != 0 }
    pub fn is_carrier(&self) -> bool { self.flags & 0x02 != 0 }
    pub fn is_idle(&self) -> bool { self.flags & 0x04 != 0 }
    
    pub fn set_miner(&mut self, value: bool) {
        if value { self.flags |= 0x01; } else { self.flags &= !0x01; }
    }
}
```

**Memory-Efficient Collections**:

```rust
// Use arrays for small, fixed-size collections
#[derive(Component)]
pub struct Inventory {
    items: [Option<Entity>; 4], // Instead of Vec for small inventories
    count: u8,
}

// Use bitsets for large boolean collections
use bit_set::BitSet;

pub struct VisibilityCache {
    visible_tiles: BitSet, // One bit per tile
    map_width: usize,
    map_height: usize,
}

impl VisibilityCache {
    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        let index = (y as usize * self.map_width) + x as usize;
        self.visible_tiles.contains(index)
    }
}
```

---

## 🐌 Common Bottlenecks

### Job System Performance Issues

**Problem: O(n²) Job Assignment**:

```rust
// Slow: Check every job against every worker
fn slow_job_assignment(
    jobs: &JobBoard,
    workers: Query<Entity, With<Worker>>,
) {
    for (job_id, job) in &jobs.jobs {
        for worker in workers.iter() {
            if can_assign_job(worker, job) {
                assign_job(worker, *job_id);
                break;
            }
        }
    }
}

// Fast: Use job queues by type
fn fast_job_assignment(
    jobs: &mut JobBoard,
    miners: Query<Entity, (With<Miner>, Without<AssignedJob>)>,
    carriers: Query<Entity, (With<Carrier>, Without<AssignedJob>)>,
) {
    // Assign mining jobs to miners
    for miner in miners.iter() {
        if let Some(job_id) = jobs.mining_queue.pop_front() {
            assign_job(miner, job_id);
        }
    }
    
    // Assign hauling jobs to carriers
    for carrier in carriers.iter() {
        if let Some(job_id) = jobs.hauling_queue.pop_front() {
            assign_job(carrier, job_id);
        }
    }
}
```

### Pathfinding Optimization

**A* Algorithm Improvements**:

```rust
// Use hierarchical pathfinding for long distances
pub struct HierarchicalPathfinder {
    high_level_graph: Graph<Region>,
    region_size: i32,
}

impl HierarchicalPathfinder {
    pub fn find_path(&self, start: Position, goal: Position) -> Option<Vec<Position>> {
        let start_region = self.get_region(start);
        let goal_region = self.get_region(goal);
        
        if start_region == goal_region {
            // Short path within same region
            return self.local_pathfind(start, goal);
        }
        
        // Long path across regions
        let region_path = self.high_level_pathfind(start_region, goal_region)?;
        self.refine_path(start, goal, region_path)
    }
}

// Cache pathfinding results
pub struct PathCache {
    cache: HashMap<(Position, Position), Vec<Position>>,
    max_entries: usize,
}

impl PathCache {
    pub fn get_or_compute<F>(&mut self, start: Position, goal: Position, compute_fn: F) -> Vec<Position>
    where F: FnOnce() -> Vec<Position>
    {
        if let Some(cached) = self.cache.get(&(start, goal)) {
            return cached.clone();
        }
        
        let path = compute_fn();
        
        if self.cache.len() >= self.max_entries {
            // Remove oldest entry (simple LRU)
            if let Some(key) = self.cache.keys().next().cloned() {
                self.cache.remove(&key);
            }
        }
        
        self.cache.insert((start, goal), path.clone());
        path
    }
}
```

### Map Update Performance

**Bulk Map Operations**:

```rust
pub struct GameMap {
    tiles: Vec<TileKind>,
    width: i32,
    height: i32,
    // Cache for expensive computations
    dirty_regions: BitSet,
    visibility_cache: HashMap<Position, BitSet>,
}

impl GameMap {
    // Batch multiple tile updates
    pub fn update_tiles(&mut self, updates: &[(i32, i32, TileKind)]) {
        for &(x, y, tile_kind) in updates {
            let index = self.coord_to_index(x, y);
            self.tiles[index] = tile_kind;
            
            // Mark region as dirty for cache invalidation
            let region_id = self.get_region_id(x, y);
            self.dirty_regions.insert(region_id);
        }
        
        // Invalidate affected caches
        self.invalidate_visibility_cache();
    }
    
    // Use inline functions for hot paths
    #[inline]
    pub fn coord_to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
    
    #[inline]
    pub fn get_tile_fast(&self, x: i32, y: i32) -> TileKind {
        // Skip bounds checking in release builds for performance
        debug_assert!(x >= 0 && x < self.width && y >= 0 && y < self.height);
        
        unsafe {
            *self.tiles.get_unchecked(self.coord_to_index(x, y))
        }
    }
}
```

---

## ✅ Optimization Checklist

### System-Level Optimizations

- [ ] **Query Specificity**: Use `With<>` and `Without<>` filters to minimize query scope
- [ ] **Resource Access**: Minimize mutable resource access conflicts
- [ ] **Batch Operations**: Group similar operations together
- [ ] **Early Returns**: Exit early when conditions aren't met
- [ ] **Component Size**: Keep components small and cache-friendly

### Data Structure Optimizations

- [ ] **Pre-allocation**: Use `Vec::with_capacity()` for known sizes
- [ ] **Object Pooling**: Reuse entities and components
- [ ] **Compact Types**: Use smaller integer types when possible
- [ ] **Bit Packing**: Pack boolean flags into single bytes
- [ ] **Array vs Vec**: Use arrays for small, fixed-size collections

### Algorithm Optimizations

- [ ] **Spatial Indexing**: Use spatial grids for position-based queries
- [ ] **Caching**: Cache expensive computations
- [ ] **Hierarchical Approaches**: Break large problems into smaller ones
- [ ] **Lazy Evaluation**: Compute values only when needed
- [ ] **Incremental Updates**: Update only changed data

### Memory Optimizations

- [ ] **Allocation Profiling**: Measure allocation hotspots
- [ ] **String Interning**: Reuse common strings
- [ ] **Memory Pools**: Pre-allocate memory for frequent operations
- [ ] **Cache Line Alignment**: Consider memory layout for hot data
- [ ] **Garbage Collection**: Remove unused entities and resources

### Performance Monitoring

- [ ] **Profiling Integration**: Built-in performance monitoring
- [ ] **Frame Time Tracking**: Monitor frame consistency
- [ ] **System Time Measurement**: Track individual system performance
- [ ] **Memory Usage Monitoring**: Watch for memory leaks
- [ ] **Regression Testing**: Automated performance tests

---

## 📈 Performance Testing

### Benchmark Framework

```rust
// benches/simulation.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_full_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("simulation");
    
    for entity_count in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("full_tick", entity_count),
            entity_count,
            |b, &entity_count| {
                let mut world = setup_world_with_entities(entity_count);
                let mut schedule = create_main_schedule();
                
                b.iter(|| {
                    schedule.run(black_box(&mut world));
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_full_simulation);
criterion_main!(benches);
```

### Load Testing

```rust
#[test]
#[ignore] // Run with `cargo test load_test -- --ignored`
fn load_test_10000_entities() {
    let mut world = setup_world_with_entities(10000);
    let mut schedule = create_main_schedule();
    
    let start = std::time::Instant::now();
    
    for tick in 0..1000 {
        schedule.run(&mut world);
        
        if tick % 100 == 0 {
            let elapsed = start.elapsed();
            let fps = (tick as f64) / elapsed.as_secs_f64();
            println!("Tick {}: {:.1} fps", tick, fps);
        }
    }
    
    let final_elapsed = start.elapsed();
    let final_fps = 1000.0 / final_elapsed.as_secs_f64();
    
    assert!(final_fps >= 30.0, "Performance below 30 fps: {:.1}", final_fps);
}
```

---

## 📚 Related Documentation

- [Developer Guide](../README.md) - Development workflow and tools
- [ECS Architecture](../ecs-guide.md) - Component system design
- [Testing Guide](../testing.md) - Testing strategies and patterns
- [Systems Reference](./api/systems.md) - System implementation details

---

*This guide provides comprehensive performance optimization strategies for Goblin Camp. Always profile before optimizing and maintain determinism in all performance improvements.*
