# Mining/Items/Stockpiles/Hauling Pipeline Design

This document defines the minimal viable pipeline for M2 Job Execution MVP, covering the flow from mining designations through item creation to stockpile hauling. This design establishes the events, components, and system interactions needed to implement the complete mining-to-storage workflow.

## Overview

The mining-items-stockpiles pipeline enables the following workflow:
1. **Mine Execution**: Mining jobs convert Wall tiles to Floor tiles and spawn items
2. **Item Management**: Items exist as entities with position and carriable properties
3. **Stockpile System**: Zones that accept items with spatial membership queries
4. **Hauling Execution**: Jobs that move items from ground to stockpiles

This pipeline integrates with the existing designation lifecycle and job assignment systems.

## Architecture Principles

- **Event-driven item creation**: Mining uses events to decouple tile mutation from item spawning
- **ECS entity items**: Items are full entities with components, not just data
- **Zone-based stockpiles**: Stockpiles are spatial regions that accept items
- **Deterministic simulation**: All systems respect fixed-step execution and seed determinism

## Events

### ItemSpawnEvent

```rust
#[derive(Debug, Clone)]
pub struct ItemSpawnEvent {
    pub item_type: ItemType,
    pub position: Position,
    pub source: ItemSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Stone,
    // Future: Wood, Metal, etc.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemSource {
    Mining,
    // Future: Chopping, Construction, etc.
}
```

**Purpose**: Decouples item creation from the system that triggers it (mining, construction, etc.)

**Usage**: Mining systems emit `ItemSpawnEvent` when tiles are mined; item spawning systems consume events to create item entities

### ItemPickupEvent

```rust
#[derive(Debug, Clone)]
pub struct ItemPickupEvent {
    pub item: Entity,
    pub agent: Entity,
}
```

**Purpose**: Tracks when agents pick up items for hauling jobs

### ItemDropEvent

```rust
#[derive(Debug, Clone)]
pub struct ItemDropEvent {
    pub item: Entity,
    pub agent: Entity,
    pub destination: Position,
}
```

**Purpose**: Tracks when agents drop items at stockpiles

## Components

### Item Components

```rust
#[derive(Component, Debug)]
pub struct Item {
    pub item_type: ItemType,
    pub weight: u32,
}

#[derive(Component, Debug)]
pub struct Carriable;

#[derive(Component, Debug)]
pub enum ItemState {
    OnGround,
    Carried { by: Entity },
    Stockpiled { stockpile: Entity },
}
```

**Item**: Core item data (type, properties)
**Carriable**: Marker component indicating item can be hauled
**ItemState**: Tracks current location/ownership state

### Inventory Components

```rust
#[derive(Component, Debug, Default)]
pub struct Inventory {
    pub held_item: Option<Entity>,
    pub capacity: u32,
}

#[derive(Component, Debug)]
pub struct CarryingItem(pub Entity);
```

**Inventory**: Agent component tracking carrying capacity and current item
**CarryingItem**: Marker component for agents currently carrying items

### Stockpile Components

```rust
#[derive(Component, Debug)]
pub struct Stockpile {
    pub bounds: StockpileBounds,
    pub accepts: Vec<ItemType>,
    pub current_items: Vec<Entity>,
    pub max_items: Option<u32>,
}

#[derive(Component, Debug)]
pub struct StockpileBounds {
    pub top_left: Position,
    pub bottom_right: Position,
}

#[derive(Component, Debug)]
pub struct StockpileCell {
    pub stockpile: Entity,
    pub occupied: bool,
}
```

**Stockpile**: Zone entity with bounds, item type filters, and capacity
**StockpileBounds**: Rectangular area definition
**StockpileCell**: Component on tile entities within stockpile bounds

## Systems and Pipeline Flow

### Phase 1: Mining Execution

```rust
pub fn mining_job_execution_system(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    mut events: EventWriter<ItemSpawnEvent>,
    query: Query<(Entity, &Position, &AssignedJob), With<Miner>>,
    job_board: Res<JobBoard>,
) {
    // For each mining agent with an assigned Mine job:
    // 1. Validate job target is Wall tile
    // 2. Convert Wall -> Floor in GameMap
    // 3. Emit ItemSpawnEvent for Stone
    // 4. Clear AssignedJob (job complete)
}
```

**Dependencies**: Must run after job assignment
**Outputs**: Modified map tiles, ItemSpawnEvent emissions

### Phase 2: Item Spawning

```rust
pub fn item_spawn_system(
    mut commands: Commands,
    mut events: EventReader<ItemSpawnEvent>,
) {
    // For each ItemSpawnEvent:
    // 1. Create item entity with Item, Position, Carriable, ItemState components
    // 2. Set ItemState::OnGround
}
```

**Dependencies**: Consumes ItemSpawnEvents from mining
**Outputs**: Item entities on the map

### Phase 3: Hauling Job Creation

```rust
pub fn hauling_job_generation_system(
    mut job_board: ResMut<JobBoard>,
    items: Query<(Entity, &Position, &ItemState), (With<Item>, With<Carriable>)>,
    stockpiles: Query<(Entity, &Stockpile, &StockpileBounds)>,
) {
    // For each item in ItemState::OnGround:
    // 1. Find nearest stockpile that accepts item type
    // 2. Find available cell in stockpile bounds
    // 3. Create JobKind::Haul job if space available
}
```

**Dependencies**: Items exist, stockpiles defined
**Outputs**: Haul jobs on JobBoard

### Phase 4: Hauling Execution

```rust
pub fn hauling_job_execution_system(
    mut commands: Commands,
    mut events: EventWriter<ItemPickupEvent>,
    mut events_drop: EventWriter<ItemDropEvent>,
    mut query: Query<(Entity, &mut Position, &mut Inventory, &AssignedJob), With<Carrier>>,
    items: Query<&mut ItemState, With<Item>>,
    job_board: Res<JobBoard>,
) {
    // For each hauling agent with assigned Haul job:
    // 1. Move toward item position if not carrying
    // 2. Pick up item when adjacent (emit ItemPickupEvent)
    // 3. Move toward stockpile destination if carrying
    // 4. Drop item when at destination (emit ItemDropEvent)
    // 5. Clear AssignedJob when complete
}
```

**Dependencies**: Path system, job assignment
**Outputs**: Agent movement, item state changes, pickup/drop events

### Phase 5: Stockpile Management

```rust
pub fn stockpile_management_system(
    mut stockpiles: Query<&mut Stockpile>,
    mut events_pickup: EventReader<ItemPickupEvent>,
    mut events_drop: EventReader<ItemDropEvent>,
    mut items: Query<&mut ItemState, With<Item>>,
) {
    // Handle ItemPickupEvent: Remove item from ground
    // Handle ItemDropEvent: Add item to stockpile, update ItemState::Stockpiled
}
```

**Dependencies**: Pickup/drop events
**Outputs**: Updated stockpile inventories, item states

## System Ordering Requirements

```rust
schedule.add_systems((
    // Phase 1: Job execution (mining creates items)
    mining_job_execution_system,
    
    // Phase 2: Event processing (spawn items from events)
    item_spawn_system.after(mining_job_execution_system),
    
    // Phase 3: Job generation (create haul jobs for new items)
    hauling_job_generation_system.after(item_spawn_system),
    
    // Phase 4: Movement and hauling
    (
        hauling_job_execution_system,
        stockpile_management_system,
    ).after(hauling_job_generation_system),
));
```

**Critical Dependencies**:
- Mining must complete before items spawn
- Items must exist before hauling jobs are created
- Job assignment happens before job execution
- Event processing maintains proper ordering

## Job Types Extension

```rust
#[derive(Debug, Clone)]
pub enum JobKind {
    Mine { x: i32, y: i32 },
    Haul { item: Entity, destination: Position },
    // Future: Build, Chop, etc.
}
```

**Haul Job**: Specifies specific item entity and target position within stockpile bounds

## Query Helpers

```rust
pub fn find_nearest_stockpile(
    item_pos: Position,
    item_type: ItemType,
    stockpiles: &Query<(Entity, &Stockpile, &StockpileBounds)>,
) -> Option<(Entity, Position)> {
    // Return nearest stockpile that accepts item_type with available space
}

pub fn find_available_stockpile_cell(
    stockpile_bounds: &StockpileBounds,
    map: &GameMap,
) -> Option<Position> {
    // Return first available position within stockpile bounds
}

pub fn stockpile_contains_position(
    bounds: &StockpileBounds,
    pos: Position,
) -> bool {
    // Check if position is within stockpile rectangular bounds
}
```

## Acceptance Criteria

### AC1: Mining Creates Items
- **Given**: Miner agent with Mine job at Wall tile
- **When**: mining_job_execution_system executes
- **Then**: Wall becomes Floor, ItemSpawnEvent emitted, Stone item created
- **Verification**: Integration test checking map state and item entities

### AC2: Items Can Be Hauled
- **Given**: Stone item on ground, stockpile with available space
- **When**: hauling systems execute
- **Then**: Haul job created, agent picks up and transports item
- **Verification**: Test item movement from ground to stockpile

### AC3: Stockpile Inventory Management
- **Given**: Agent dropping item at stockpile
- **When**: stockpile_management_system processes ItemDropEvent
- **Then**: Item state becomes Stockpiled, stockpile inventory updated
- **Verification**: Test stockpile item counts and item states

### AC4: End-to-End Pipeline
- **Given**: Mine designation at Wall tile, empty stockpile
- **When**: Full pipeline executes through multiple ticks
- **Then**: Wall mined, Stone created, transported to stockpile
- **Verification**: Integration test of complete workflow

### AC5: Multiple Items and Stockpiles
- **Given**: Multiple mine jobs, multiple stockpiles
- **When**: Systems execute with pathfinding
- **Then**: Items distributed to nearest available stockpiles
- **Verification**: Test spatial distribution and pathfinding integration

## Future Extensions

### Additional Item Types
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Stone,
    Wood,
    IronOre,
    Coal,
    // etc.
}
```

### Stockpile Filters
```rust
#[derive(Component, Debug)]
pub struct StockpileFilter {
    pub accepts: HashSet<ItemType>,
    pub rejects: HashSet<ItemType>,
    pub priority: u8,
}
```

### Item Quality and Properties
```rust
#[derive(Component, Debug)]
pub struct ItemQuality(pub u8); // 0-10 quality scale

#[derive(Component, Debug)]
pub struct ItemProperties {
    pub durability: u32,
    pub material: MaterialType,
    pub crafted_by: Option<Entity>,
}
```

### Multi-Item Inventory
```rust
#[derive(Component, Debug)]
pub struct Inventory {
    pub items: Vec<Entity>,
    pub weight_capacity: u32,
    pub volume_capacity: u32,
}
```

## Performance Considerations

### Spatial Indexing
- Consider spatial hash or grid for stockpile queries when many stockpiles exist
- Cache nearest-stockpile results for repeated item types

### Event Batching
- Batch ItemSpawnEvents to reduce per-event processing overhead
- Process pickup/drop events in batches for stockpile updates

### Job Generation Throttling
- Limit hauling job creation per tick to prevent job board overflow
- Prioritize high-priority stockpiles or item types

## Testing Strategy

### Unit Tests
- Individual system behavior with controlled entity setups
- Event emission and consumption verification
- Component state transitions

### Integration Tests
- Full pipeline tests in `crates/gc_core/tests/mining_hauling_tests.rs`
- Multi-agent scenarios with contention
- Pathfinding integration with hauling

### Demo Validation
- Extend jobs demo to show mining and hauling counts
- Visual verification of item movement in ASCII maps
- Performance metrics for large-scale mining operations

## Related Documentation

- [Designation Lifecycle](designation_lifecycle.md) - Prerequisite designation system
- [AI Jobs](ai_jobs.md) - Job board and assignment system
- [Pathfinding](pathfinding.md) - Movement system used by hauling
- [MASTER_PLAN.md](../plan/MASTER_PLAN.md) - M2 roadmap and acceptance criteria

## Change History

- **v1.0** - Initial design for M2 Job Execution MVP pipeline