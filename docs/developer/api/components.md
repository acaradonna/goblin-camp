# 📋 Components API Reference

> *Complete reference for all ECS components in Goblin Camp*

This document provides detailed documentation for every component used in the Goblin Camp simulation, organized by functional area.

## 🎯 Quick Navigation

- [Entity Type Markers](#entity-type-markers) - Components that identify entity types
- [Spatial Components](#spatial-components) - Position and area definition
- [Job & Task Components](#job--task-components) - Work assignment and execution
- [Item & Inventory Components](#item--inventory-components) - Resource management
- [Capability Components](#capability-components) - Entity abilities and roles
- [Lifecycle Components](#lifecycle-components) - State management
- [Zone Components](#zone-components) - Area-based systems

---

## 🏷️ Entity Type Markers

### `Goblin`

```rust
#[derive(Component, Debug)]
pub struct Goblin;
```

**Purpose**: Marker component identifying goblin entities in the world
**Usage**: Core entity type for all worker agents
**Systems**: Used by all agent-related queries and systems

**Example**:
```rust
// Spawn a goblin worker
world.spawn((
    Goblin,
    Position(10, 15),
    Miner,
    AssignedJob(None),
));

// Query all goblins
fn goblin_system(goblins: Query<Entity, With<Goblin>>) {
    for goblin in goblins.iter() {
        // Process goblin entities
    }
}
```

---

## 📍 Spatial Components

### `Position`

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position(pub i32, pub i32);
```

**Purpose**: Defines the world coordinates of an entity
**Usage**: Required for all entities that exist in the spatial simulation
**Systems**: Movement, pathfinding, collision detection, FOV calculations

**Fields**:
- `0: i32` - X coordinate in world space
- `1: i32` - Y coordinate in world space

**Example**:
```rust
// Create entity at position (25, 30)
world.spawn((Position(25, 30), Goblin));

// Move entity to new position
fn movement_system(mut positions: Query<&mut Position, With<Goblin>>) {
    for mut pos in positions.iter_mut() {
        pos.0 += 1; // Move right
        pos.1 += 1; // Move down
    }
}

// Distance calculation
fn distance(pos1: &Position, pos2: &Position) -> f32 {
    let dx = (pos1.0 - pos2.0) as f32;
    let dy = (pos1.1 - pos2.1) as f32;
    (dx * dx + dy * dy).sqrt()
}
```

### `ViewRange`

```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct ViewRange(pub i32);
```

**Purpose**: Defines how far an entity can see for field-of-view calculations
**Usage**: Optional component for entities that need vision systems
**Systems**: FOV calculation, visibility determination

**Fields**:
- `0: i32` - Maximum view distance in tiles

**Example**:
```rust
// Goblin with extended vision
world.spawn((
    Goblin,
    Position(10, 10),
    ViewRange(8), // Can see 8 tiles away
));

// FOV system usage
fn fov_system(
    entities: Query<(Entity, &Position, Option<&ViewRange>)>
) {
    for (entity, pos, view_range) in entities.iter() {
        let range = view_range.map(|vr| vr.0).unwrap_or(5); // Default 5 tiles
        // Calculate visibility with range
    }
}
```

### `ZoneBounds`

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneBounds {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}
```

**Purpose**: Defines rectangular areas for zones like stockpiles
**Usage**: Area-based entities that span multiple tiles
**Systems**: Stockpile management, zone queries, area calculations

**Fields**:
- `min_x: i32` - Left boundary (inclusive)
- `min_y: i32` - Top boundary (inclusive)  
- `max_x: i32` - Right boundary (inclusive)
- `max_y: i32` - Bottom boundary (inclusive)

**Methods**:

```rust
impl ZoneBounds {
    /// Create new zone bounds from corner coordinates
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self;
    
    /// Check if a position is within the zone
    pub fn contains(&self, x: i32, y: i32) -> bool;
    
    /// Calculate the center point of the zone
    pub fn center(&self) -> (i32, i32);
    
    /// Calculate the area of the zone in tiles
    pub fn area(&self) -> i32;
}
```

**Example**:
```rust
// Create a 5x5 stockpile zone
let bounds = ZoneBounds::new(10, 10, 14, 14);
world.spawn((
    Stockpile::default(),
    Position(bounds.center().0, bounds.center().1),
    bounds,
));

// Check if item is in stockpile
fn check_stockpile_membership(
    items: Query<&Position, With<Item>>,
    stockpiles: Query<&ZoneBounds, With<Stockpile>>,
) {
    for item_pos in items.iter() {
        for bounds in stockpiles.iter() {
            if bounds.contains(item_pos.0, item_pos.1) {
                println!("Item is in stockpile");
            }
        }
    }
}
```

---

## 🔧 Job & Task Components

### `AssignedJob`

```rust
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AssignedJob(pub Option<JobId>);
```

**Purpose**: Tracks which job (if any) is currently assigned to an entity
**Usage**: All worker entities that can be assigned tasks
**Systems**: Job assignment, job execution, worker queries

**Fields**:
- `0: Option<JobId>` - Current job ID, or `None` if available for work

**Example**:
```rust
// Spawn worker available for jobs
world.spawn((
    Goblin,
    Miner,
    Position(10, 10),
    AssignedJob(None), // Available for assignment
));

// Find available workers
fn find_available_workers(
    workers: Query<Entity, (With<Miner>, Without<AssignedJob>)>
) {
    for worker in workers.iter() {
        // This worker can be assigned a job
    }
}

// Check if worker has a job
fn worker_status_system(
    workers: Query<(Entity, &AssignedJob)>
) {
    for (entity, assigned_job) in workers.iter() {
        match assigned_job.0 {
            Some(job_id) => println!("Worker {:?} has job {:?}", entity, job_id),
            None => println!("Worker {:?} is available", entity),
        }
    }
}
```

---

## 📦 Item & Inventory Components

### `Item`

```rust
#[derive(Component, Debug)]
pub struct Item {
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    Stone,
    // Future: Wood, Metal, Food, etc.
}
```

**Purpose**: Core component for all item entities in the world
**Usage**: Required for all resource and object entities
**Systems**: Item spawning, hauling, stockpile management

**Fields**:
- `item_type: ItemType` - The specific type of item

**Methods**:

```rust
impl Item {
    /// Create a stone item (primary resource from mining)
    pub fn stone() -> Self {
        Self { item_type: ItemType::Stone }
    }
}
```

**Example**:
```rust
// Spawn stone item from mining
commands.spawn((
    Item::stone(),
    Position(mining_location.0, mining_location.1),
    Carriable,
    Stone,
));

// Query items by type
fn stone_counting_system(
    items: Query<&Item>
) {
    let stone_count = items.iter()
        .filter(|item| item.item_type == ItemType::Stone)
        .count();
    println!("Stone items: {}", stone_count);
}
```

### `Carriable`

```rust
#[derive(Component, Debug)]
pub struct Carriable;
```

**Purpose**: Marker indicating an item can be picked up and transported
**Usage**: Items that can be hauled by worker entities
**Systems**: Hauling job creation, pickup/drop operations

**Example**:
```rust
// Make item carriable
world.spawn((
    Item::stone(),
    Position(15, 20),
    Carriable, // Can be picked up
));

// Find all carriable items
fn find_carriable_items(
    items: Query<(Entity, &Position), (With<Item>, With<Carriable>)>
) {
    for (entity, pos) in items.iter() {
        println!("Carriable item at ({}, {})", pos.0, pos.1);
    }
}
```

### `Stone`

```rust
#[derive(Component, Debug)]
pub struct Stone;
```

**Purpose**: Specific marker for stone items
**Usage**: Type-specific behavior and queries for stone resources
**Systems**: Stockpile filtering, crafting recipes, item identification

**Example**:
```rust
// Spawn stone with all necessary components
commands.spawn((
    Item::stone(),
    Position(x, y),
    Carriable,
    Stone, // Type-specific marker
));

// Query only stone items
fn stone_management_system(
    stones: Query<(Entity, &Position), With<Stone>>
) {
    for (entity, pos) in stones.iter() {
        // Handle stone-specific logic
    }
}
```

### `Inventory`

```rust
#[derive(Component, Debug, Default)]
pub struct Inventory(pub Option<Entity>);
```

**Purpose**: Allows entities to carry a single item (MVP implementation)
**Usage**: Worker entities that can pick up and transport items
**Systems**: Hauling execution, item pickup/drop, inventory management

**Fields**:
- `0: Option<Entity>` - Entity reference to carried item, or `None` if empty

**Example**:
```rust
// Spawn worker with inventory capability
world.spawn((
    Goblin,
    Carrier,
    Position(10, 10),
    Inventory(None), // Empty inventory
));

// Pickup item logic
fn pickup_system(
    mut carriers: Query<(&mut Inventory, &Position), With<Carrier>>,
    items: Query<(Entity, &Position), (With<Item>, With<Carriable>)>,
) {
    for (mut inventory, carrier_pos) in carriers.iter_mut() {
        if inventory.0.is_some() {
            continue; // Already carrying something
        }
        
        // Find item at same position
        for (item_entity, item_pos) in items.iter() {
            if carrier_pos.0 == item_pos.0 && carrier_pos.1 == item_pos.1 {
                inventory.0 = Some(item_entity);
                break;
            }
        }
    }
}

// Check what entity is carrying
fn inventory_status_system(
    carriers: Query<(Entity, &Inventory), With<Carrier>>
) {
    for (entity, inventory) in carriers.iter() {
        match inventory.0 {
            Some(item) => println!("Entity {:?} is carrying {:?}", entity, item),
            None => println!("Entity {:?} has empty inventory", entity),
        }
    }
}
```

---

## 🛠️ Capability Components

### `Miner`

```rust
#[derive(Component, Debug)]
pub struct Miner;
```

**Purpose**: Marks entities capable of executing mining jobs
**Usage**: Worker entities that can convert wall tiles to floor tiles
**Systems**: Job assignment (mining jobs), mining execution

**Example**:
```rust
// Spawn mining-capable goblin
world.spawn((
    Goblin,
    Miner, // Can mine walls
    Position(5, 5),
    AssignedJob(None),
));

// Find available miners
fn assign_mining_jobs(
    available_miners: Query<Entity, (With<Miner>, Without<AssignedJob>)>,
    mut job_board: ResMut<JobBoard>,
) {
    for miner in available_miners.iter() {
        // Assign mining job to this miner
    }
}
```

### `Carrier`

```rust
#[derive(Component, Debug)]
pub struct Carrier;
```

**Purpose**: Marks entities capable of hauling items between locations
**Usage**: Worker entities that can pick up, transport, and deliver items
**Systems**: Job assignment (hauling jobs), hauling execution, inventory operations

**Example**:
```rust
// Spawn hauling-capable goblin
world.spawn((
    Goblin,
    Carrier, // Can haul items
    Position(10, 10),
    AssignedJob(None),
    Inventory(None),
));

// Multi-capable worker
world.spawn((
    Goblin,
    Miner,   // Can mine AND haul
    Carrier,
    Position(15, 15),
    AssignedJob(None),
    Inventory(None),
));
```



## 🔄 Lifecycle Components

### `DesignationLifecycle`

```rust
#[derive(Component, Debug)]
pub struct DesignationLifecycle {
    pub state: DesignationState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignationState {
    Active,   // Ready to be processed into jobs
    Ignored,  // Duplicate designation that should be skipped
    Consumed, // Processed designation (for future use)
}
```

**Purpose**: Manages the processing lifecycle of player designations
**Usage**: All designation entities to prevent duplicate processing
**Systems**: Designation deduplication, designation-to-job conversion

**Fields**:
- `state: DesignationState` - Current processing state

**Example**:
```rust
// Create mining designation
world.spawn((
    MineDesignation,
    Position(20, 25),
    DesignationLifecycle {
        state: DesignationState::Active, // Ready to process
    },
));

// Process designations by state
fn process_designations(
    designations: Query<(Entity, &Position, &DesignationLifecycle), With<MineDesignation>>
) {
    for (entity, pos, lifecycle) in designations.iter() {
        match lifecycle.state {
            DesignationState::Active => {
                // Convert to job
            },
            DesignationState::Ignored => {
                // Skip processing
            },
            DesignationState::Consumed => {
                // Remove from world
            },
        }
    }
}
```

### `MineDesignation`

```rust
#[derive(Component, Debug)]
pub struct MineDesignation;
```

**Purpose**: Marks an entity as a mining designation from player input
**Usage**: Player-created designations for mining operations
**Systems**: Designation deduplication, job generation, designation processing

**Example**:
```rust
// Create mining designation at mouse click
world.spawn((
    MineDesignation,
    Position(mouse_x, mouse_y),
    DesignationLifecycle {
        state: DesignationState::Active,
    },
));

// Query only mining designations
fn mining_designation_system(
    designations: Query<(Entity, &Position), With<MineDesignation>>
) {
    for (entity, pos) in designations.iter() {
        // Process mining designations
    }
}
```

---

## 🏭 Zone Components

### `Stockpile`

```rust
#[derive(Component, Debug)]
pub struct Stockpile {
    pub accepts: Option<Vec<ItemType>>, // None = accept all items
}
```

**Purpose**: Defines a storage zone for organizing items
**Usage**: Area-based entities that accept and store items
**Systems**: Hauling job creation, item organization, stockpile queries

**Fields**:
- `accepts: Option<Vec<ItemType>>` - Item types accepted (None = all types)

**Methods**:

```rust
impl Stockpile {
    /// Create stockpile that accepts all item types
    pub fn new_universal() -> Self {
        Self { accepts: None }
    }
    
    /// Create stockpile that accepts specific item types
    pub fn new_filtered(item_types: Vec<ItemType>) -> Self {
        Self { accepts: Some(item_types) }
    }
    
    /// Check if stockpile accepts a specific item type
    pub fn accepts_item(&self, item_type: ItemType) -> bool {
        match &self.accepts {
            None => true, // Accept all
            Some(types) => types.contains(&item_type),
        }
    }
}
```

**Example**:
```rust
// Universal stockpile (accepts all items)
world.spawn((
    Stockpile::new_universal(),
    Position(30, 30),
    ZoneBounds::new(28, 28, 32, 32),
));

// Stone-only stockpile
world.spawn((
    Stockpile::new_filtered(vec![ItemType::Stone]),
    Position(40, 40),
    ZoneBounds::new(38, 38, 42, 42),
));

// Check stockpile compatibility
fn stockpile_compatibility_system(
    stockpiles: Query<&Stockpile>,
    items: Query<&Item>,
) {
    for stockpile in stockpiles.iter() {
        for item in items.iter() {
            if stockpile.accepts_item(item.item_type) {
                println!("Stockpile accepts this item type");
            }
        }
    }
}
```

---

## 🎯 Usage Patterns

### Entity Composition Examples

```rust
// Basic goblin worker
world.spawn((
    Goblin,              // Entity type
    Position(10, 10),    // Spatial
    AssignedJob(None),   // Job system
));

// Specialized miner
world.spawn((
    Goblin,
    Miner,               // Mining capability
    Position(15, 15),
    AssignedJob(None),
    ViewRange(6),        // Extended vision
));

// Multi-role worker
world.spawn((
    Goblin,
    Miner,               // Can mine
    Carrier,             // Can haul
    Position(20, 20),
    AssignedJob(None),
    Inventory(None),     // Can carry items
));

// Resource item
commands.spawn((
    Item::stone(),       // Item data
    Position(25, 25),    // World position
    Carriable,           // Can be picked up
    Stone,               // Type-specific marker
));

// Storage zone
world.spawn((
    Stockpile::new_universal(),  // Accepts all items
    Position(35, 35),            // Center position
    ZoneBounds::new(33, 33, 37, 37), // 5x5 area
));
```

### Common Query Patterns

```rust
// Find available workers
Query<Entity, (With<Miner>, Without<AssignedJob>)>

// Get all items at a position
Query<(Entity, &Item), (With<Position>, With<Carriable>)>

// Workers currently hauling
Query<(Entity, &Inventory), (With<Carrier>, With<AssignedJob>)>

// Active designations
Query<(Entity, &Position), (With<MineDesignation>, With<DesignationLifecycle>)>

// Stockpiles with bounds
Query<(&Stockpile, &ZoneBounds)>
```

## 📚 Related Documentation

- [ECS Architecture Guide](../ecs-guide.md) - Understanding ECS patterns
- [Job System Deep Dive](../job-system.md) - Task assignment and execution
- [Systems Reference](./systems.md) - Systems that use these components
- [Developer Guide](../README.md) - Getting started with development

---

*This reference covers all components in the current codebase. For system-specific usage patterns, see the [Systems Reference](./systems.md).*
