# 🏗️ ECS Architecture Guide

> *Understanding Entity-Component-System design in Goblin Camp*

This guide explains how Goblin Camp uses the Entity-Component-System (ECS) architecture pattern for high-performance, data-oriented game simulation.

## 📖 ECS Fundamentals

### Core Concepts

**Entities** are unique identifiers for game objects:
```rust
// Entities are just IDs - no data
let goblin_entity = world.spawn().id();
let stone_entity = world.spawn().id();
```

**Components** are pure data structures:
```rust
#[derive(Component, Debug)]
pub struct Position(pub i32, pub i32);

#[derive(Component, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
```

**Systems** contain logic that operates on component data:
```rust
fn movement_system(mut positions: Query<&mut Position, With<Goblin>>) {
    for mut pos in positions.iter_mut() {
        // Move goblin logic
        pos.0 += 1;
    }
}
```

## 🎯 Goblin Camp ECS Architecture

### Entity Types

| Entity Type | Core Components | Purpose |
|-------------|----------------|---------|
| **Goblin** | `Goblin`, `Position`, `AssignedJob` | Worker agents |
| **Item** | `Item`, `Position`, `Carriable` | Resources and objects |
| **Designation** | `MineDesignation`, `Position`, `DesignationLifecycle` | Player commands |
| **Stockpile** | `Stockpile`, `Position`, `ZoneBounds` | Storage areas |

### Component Categories

#### 🏷️ Marker Components
Pure tags with no data - used for entity classification:

```rust
#[derive(Component, Debug)]
pub struct Goblin;        // Marks goblin entities

#[derive(Component, Debug)]
pub struct Miner;         // Marks mining-capable entities

#[derive(Component, Debug)]
pub struct Carrier;       // Marks hauling-capable entities
```

#### 📍 Spatial Components
Position and area definition:

```rust
#[derive(Component, Debug)]
pub struct Position(pub i32, pub i32);  // World coordinates

#[derive(Component, Debug)]
pub struct ZoneBounds {                  // Rectangular areas
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

#[derive(Component, Debug)]
pub struct ViewRange(pub i32);           // FOV distance
```

#### 🛠️ Behavioral Components
State and capability data:

```rust
#[derive(Component, Debug)]
pub struct AssignedJob(pub Option<JobId>);  // Current task

#[derive(Component, Debug)]
pub struct Inventory(pub Option<Entity>);   // Carried item

#[derive(Component, Debug)]
pub struct DesignationLifecycle {           // Processing state
    pub state: DesignationState,
}
```

#### 🎯 Item Components
Resource and object properties:

```rust
#[derive(Component, Debug)]
pub struct Item {                           // Item type and properties
    pub item_type: ItemType,
}

#[derive(Component, Debug)]
pub struct Carriable;                       // Can be picked up

#[derive(Component, Debug)]
pub struct Stone;                          // Specific item marker
```

## 🔄 System Organization

### System Categories

#### ⏰ Core Simulation Systems
```rust
// Time advancement - runs every tick
pub fn advance_time(mut time: ResMut<Time>)

// Movement and spatial updates
pub fn movement_system(mut positions: Query<&mut Position>)
```

#### 🏗️ Job Processing Systems
```rust
// Convert designations to jobs
pub fn designation_to_jobs_system(/* ... */)

// Assign jobs to available workers
pub fn job_assignment_system(/* ... */)

// Execute mining jobs
pub fn mining_execution_system(/* ... */)

// Execute hauling jobs  
pub fn hauling_execution_system(/* ... */)
```

#### 🎯 Lifecycle Management Systems
```rust
// Prevent duplicate designations
pub fn designation_dedup_system(/* ... */)

// Create haul jobs for new items
pub fn auto_haul_system(/* ... */)
```

### System Execution Order

Critical ordering ensures deterministic behavior:

```rust
// Proper system scheduling
schedule.add_systems((
    // Phase 1: Input processing
    designation_dedup_system,
    
    // Phase 2: Job management
    designation_to_jobs_system.after(designation_dedup_system),
    job_assignment_system.after(designation_to_jobs_system),
    
    // Phase 3: Job execution
    mining_execution_system.after(job_assignment_system),
    auto_haul_system.after(mining_execution_system),
    hauling_execution_system.after(auto_haul_system),
    
    // Phase 4: Cleanup
    advance_time.after(hauling_execution_system),
));
```

## 🎨 Component Design Patterns

### 1. Composition over Inheritance

Instead of inheritance hierarchies, combine components:

```rust
// ❌ Traditional OOP approach
class MinerGoblin extends Goblin {
    mine() { /* ... */ }
}

// ✅ ECS composition approach
world.spawn((
    Goblin,        // Entity type
    Miner,         // Capability
    Position(x, y), // Spatial data
    AssignedJob(None), // State
));
```

### 2. Data-Oriented Design

Components store only data, systems contain logic:

```rust
// ❌ Methods on components
#[derive(Component)]
pub struct Position {
    x: i32, y: i32,
    fn move_to(&mut self, new_x: i32, new_y: i32) { /* ... */ }
}

// ✅ Pure data components
#[derive(Component)]
pub struct Position(pub i32, pub i32);

// Logic in systems
fn movement_system(mut positions: Query<&mut Position>) {
    // Movement logic here
}
```

### 3. Optional vs Required Components

Use `Option<&Component>` for optional components:

```rust
// System handles entities with or without ViewRange
fn visibility_system(
    entities: Query<(Entity, &Position, Option<&ViewRange>)>
) {
    for (entity, pos, view_range) in entities.iter() {
        let range = view_range.map(|vr| vr.0).unwrap_or(5); // Default range
        // Calculate visibility with range
    }
}
```

## 🚀 Query Patterns

### Basic Queries

```rust
// All entities with Position
Query<&Position>

// Mutable position access
Query<&mut Position>

// Multiple components
Query<(&Position, &Health)>

// Entity ID + components
Query<(Entity, &Position)>
```

### Filtered Queries

```rust
// Only goblins
Query<&Position, With<Goblin>>

// Goblins but not miners
Query<&Position, (With<Goblin>, Without<Miner>)>

// Entities with jobs assigned
Query<&AssignedJob, Changed<AssignedJob>>

// New items (added this tick)
Query<&Position, (With<Item>, Added<Item>)>
```

### Complex Queries with ParamSet

When you need multiple mutable queries:

```rust
fn complex_system(
    mut param_set: ParamSet<(
        Query<&mut Position, With<Goblin>>,
        Query<&mut Position, With<Item>>,
    )>
) {
    // First query: update goblin positions
    {
        let mut goblins = param_set.p0();
        for mut pos in goblins.iter_mut() {
            // Update goblin position
        }
    }
    
    // Second query: update item positions  
    {
        let mut items = param_set.p1();
        for mut pos in items.iter_mut() {
            // Update item position
        }
    }
}
```

## 🎯 Best Practices

### Component Design

1. **Keep components simple** - Prefer multiple small components over complex ones
2. **Use clear naming** - Component names should be self-documenting
3. **Minimize dependencies** - Components shouldn't reference other components
4. **Use markers effectively** - Marker components enable powerful filtering

### System Design

1. **Single responsibility** - Each system should have one clear purpose
2. **Minimize state** - Prefer stateless systems that operate on component data
3. **Use proper ordering** - Chain systems with `.after()` and `.before()`
4. **Handle edge cases** - Check for None values and empty queries

### Performance Optimization

1. **Use filtered queries** - Narrow queries to only needed entities
2. **Batch operations** - Process multiple entities efficiently
3. **Avoid unnecessary allocations** - Reuse collections when possible
4. **Profile before optimizing** - Measure actual performance impacts

## 🧪 Testing ECS Systems

### Unit Testing Components

```rust
#[test]
fn test_position_component() {
    let mut world = World::new();
    let entity = world.spawn(Position(10, 20)).id();
    
    let position = world.get::<Position>(entity).unwrap();
    assert_eq!(position.0, 10);
    assert_eq!(position.1, 20);
}
```

### Integration Testing Systems

```rust
#[test]
fn test_movement_system() {
    let mut world = World::new();
    let mut schedule = Schedule::default();
    schedule.add_systems(movement_system);
    
    // Setup entities
    world.spawn((Position(0, 0), Goblin));
    
    // Run system
    schedule.run(&mut world);
    
    // Verify results
    let position = world.query::<&Position>().single(&world);
    assert_eq!(position.0, 1); // Moved one step
}
```

## 🔍 Debugging ECS

### Query Inspection

```rust
fn debug_system(
    goblins: Query<(Entity, &Position, &AssignedJob), With<Goblin>>
) {
    println!("=== Goblin Status ===");
    for (entity, pos, job) in goblins.iter() {
        println!("Entity {:?}: pos=({}, {}), job={:?}", 
                entity, pos.0, pos.1, job.0);
    }
}
```

### Component Validation

```rust
fn validate_world_state(world: &World) {
    // Check for orphaned components
    let positions = world.query::<(Entity, &Position)>();
    for (entity, _) in positions.iter(world) {
        // Verify entity has required components
        assert!(world.get::<Goblin>(entity).is_some() || 
                world.get::<Item>(entity).is_some(),
                "Entity {:?} has Position but no type marker", entity);
    }
}
```

## 📚 Further Reading

- [Bevy ECS Documentation](https://docs.rs/bevy_ecs/latest/bevy_ecs/)
- [ECS Architecture Benefits](https://en.wikipedia.org/wiki/Entity_component_system)
- [Data-Oriented Design](https://www.dataorienteddesign.com/dodbook/)
- [Systems Reference](./api/systems.md)
- [Performance Guide](./performance.md)

---

*This guide covers the foundational ECS patterns used throughout Goblin Camp. For specific system implementations, see the [Systems Reference](./api/systems.md).*
