use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Core ECS Components for Goblin Camp Simulation
///
/// This module defines all the Entity-Component-System (ECS) components used
/// throughout the simulation. Components are pure data structures that define
/// the properties and capabilities of game entities.
/// Marker component for goblin entities
/// Used to identify goblin agents in the world for queries and systems
#[derive(Component, Debug)]
pub struct Goblin;

/// Component for entities that have job queues
/// Currently unused but reserved for future job scheduling features
#[derive(Component, Debug)]
pub struct JobQueue;

/// Component marking an entity as capable of carrying/hauling items
/// Carriers can pick up items and transport them to stockpiles
#[derive(Component, Debug)]
pub struct Carrier;

/// Component marking an entity as capable of mining operations
/// Miners can execute mining jobs to convert wall tiles to floor tiles
#[derive(Component, Debug)]
pub struct Miner;

/// Component marking an entity as capable of construction operations
/// Builders can execute build jobs to construct walls, floors, and doors
#[derive(Component, Debug)]
pub struct Builder;

/// Component tracking which job (if any) is currently assigned to an entity
/// Contains an optional JobId that references a job in the JobBoard
/// When None, the entity is available for new job assignments
#[derive(Component, Debug, Default)]
pub struct AssignedJob(pub Option<crate::jobs::JobId>);

/// Component defining how far an entity can see for line-of-sight calculations
/// Used by the FOV (Field of View) system to determine visibility ranges
#[derive(Component, Debug)]
pub struct VisionRadius(pub i32);

/// Represents the lifecycle state of a designation
/// Designations go through states to prevent duplicate processing and
/// enable proper cleanup of completed or invalid designations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum DesignationState {
    /// Active designation ready to be processed
    /// This is the initial state when a designation is created
    #[default]
    Active,
    /// Duplicate designation that should be ignored
    /// Used when the same designation would create duplicate jobs
    Ignored,
    /// Designation that has been consumed/processed (for future use)
    /// Reserved for tracking completed designations
    Consumed,
}

/// Component to track the lifecycle state of designations
/// Attached to designation entities to manage their processing lifecycle
/// and prevent duplicate job creation from the same designation
#[derive(Component, Debug, Default)]
pub struct DesignationLifecycle(pub DesignationState);

/// Types of items that can exist in the world
/// This enum defines all possible item types that can be created,
/// carried, and stored in stockpiles. Currently only Stone is implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ItemType {
    /// Stone items created from mining operations
    /// These are the primary resource produced by mining wall tiles
    Stone,
    /// Wood planks used for construction
    WoodPlank,
    /// Stone blocks used for construction
    StoneBlock,
}

/// Material types used in construction recipes
/// Defines what materials can be used to build structures
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum MaterialType {
    /// Any type of stone material
    Stone,
    /// Any type of wood material
    Wood,
}

impl MaterialType {
    /// Check if an item type satisfies this material requirement
    pub fn accepts_item(&self, item_type: ItemType) -> bool {
        match (self, item_type) {
            (MaterialType::Stone, ItemType::Stone | ItemType::StoneBlock) => true,
            (MaterialType::Wood, ItemType::WoodPlank) => true,
            _ => false,
        }
    }

    /// Get all item types that satisfy this material requirement
    pub fn accepted_items(&self) -> Vec<ItemType> {
        match self {
            MaterialType::Stone => vec![ItemType::Stone, ItemType::StoneBlock],
            MaterialType::Wood => vec![ItemType::WoodPlank],
        }
    }
}

/// Component representing an item entity that can be spawned, carried, and placed
/// Items are full ECS entities with position and other properties,
/// making them part of the spatial simulation rather than just data
#[derive(Component, Debug)]
pub struct Item {
    /// The specific type of this item (Stone, Wood, etc.)
    pub item_type: ItemType,
}

impl Item {
    /// Creates a new stone item component
    /// This is the primary item type created by mining operations
    pub fn stone() -> Self {
        Self {
            item_type: ItemType::Stone,
        }
    }

    /// Creates a new wood plank item component
    /// Used for construction of floors and doors
    pub fn wood_plank() -> Self {
        Self {
            item_type: ItemType::WoodPlank,
        }
    }

    /// Creates a new stone block item component
    /// Used for construction of walls
    pub fn stone_block() -> Self {
        Self {
            item_type: ItemType::StoneBlock,
        }
    }
}

/// Marker component indicating that an item can be carried/hauled by agents
/// Items with this component can be picked up by Carrier entities
/// and transported to stockpiles or other locations
#[derive(Component, Debug)]
pub struct Carriable;

/// Component representing a stone item
/// This is a specific marker for stone items, used in conjunction
/// with the more generic Item component for type-specific behavior
#[derive(Component, Debug)]
pub struct Stone;

/// Component representing a wood plank item
/// Used for construction of floors and doors
#[derive(Component, Debug)]
pub struct WoodPlank;

/// Component representing a stone block item
/// Used for construction of walls
#[derive(Component, Debug)]
pub struct StoneBlock;

/// Inventory component for agents to carry a single item (MVP)
/// Holds an optional entity reference to the carried item
/// Currently supports only one item at a time for simplicity
/// When Some(entity), the entity is the item being carried
/// When None, the inventory is empty and can accept a new item
#[derive(Component, Debug, Default)]
pub struct Inventory(pub Option<Entity>);

/// Defines rectangular bounds for a zone
/// Used by stockpiles and other area-based game features
/// Coordinates are inclusive on all sides
#[derive(Component, Debug, Clone)]
pub struct ZoneBounds {
    /// Minimum X coordinate (inclusive)
    pub min_x: i32,
    /// Minimum Y coordinate (inclusive)
    pub min_y: i32,
    /// Maximum X coordinate (inclusive)  
    pub max_x: i32,
    /// Maximum Y coordinate (inclusive)
    pub max_y: i32,
}

impl ZoneBounds {
    /// Create a new zone bounds with the specified coordinates
    /// All coordinates are inclusive
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// Check if a position is within the zone bounds (inclusive)
    /// Returns true if the point (x, y) is inside or on the boundary
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    /// Get the center point of the zone
    /// Returns the center coordinates, rounded down for odd dimensions
    pub fn center(&self) -> (i32, i32) {
        ((self.min_x + self.max_x) / 2, (self.min_y + self.max_y) / 2)
    }
}

/// Component marking a stockpile zone that can accept items
/// Stockpiles are storage areas where items can be hauled and organized
/// They use ZoneBounds to define their spatial area
#[derive(Component, Debug)]
pub struct Stockpile {
    /// Items accepted by this stockpile (None = accepts all)
    /// When Some(vec), only items matching the specified types are accepted
    /// When None, all item types are accepted (current MVP behavior)
    pub accepts: Option<Vec<ItemType>>,
}

// ============================================================================
// Combat MVP Components
// ============================================================================

/// Faction types for combat and social interactions
/// Determines hostility and targeting behavior between entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactionKind {
    /// Player-controlled goblins and allies
    Goblins,
    /// Hostile invaders and enemies
    Invaders,
    /// Neutral entities that don't participate in combat
    Neutral,
}

/// Component defining an entity's faction allegiance
/// Used to determine hostility and targeting in combat systems
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    /// The faction this entity belongs to
    pub kind: FactionKind,
}

impl Faction {
    /// Create a new faction component
    pub fn new(kind: FactionKind) -> Self {
        Self { kind }
    }

    /// Check if this faction is hostile to another faction
    pub fn is_hostile_to(&self, other: &Faction) -> bool {
        matches!(
            (self.kind, other.kind),
            (FactionKind::Goblins, FactionKind::Invaders)
                | (FactionKind::Invaders, FactionKind::Goblins)
        )
    }
}

/// Component representing an entity's health and vitality
/// Tracks current and maximum hit points for combat and survival
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    /// Current hit points (0 = dead)
    pub hp: i32,
    /// Maximum hit points this entity can have
    pub max_hp: i32,
}

impl Health {
    /// Create a new health component with specified values
    /// Automatically clamps hp to valid range [0, max_hp]
    pub fn new(hp: i32, max_hp: i32) -> Self {
        let max_hp = max_hp.max(0);
        let hp = hp.clamp(0, max_hp);
        Self { hp, max_hp }
    }

    /// Create a new health component with full health
    pub fn full(max_hp: i32) -> Self {
        Self::new(max_hp, max_hp)
    }

    /// Check if the entity is alive (hp > 0)
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Check if the entity is dead (hp <= 0)
    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    /// Apply damage to the entity, clamping to 0
    /// Returns the actual damage dealt
    pub fn take_damage(&mut self, damage: i32) -> i32 {
        let old_hp = self.hp;
        self.hp = (self.hp - damage).clamp(0, self.max_hp);
        old_hp - self.hp
    }

    /// Heal the entity, clamping to max_hp
    /// Returns the actual healing applied
    pub fn heal(&mut self, amount: i32) -> i32 {
        let old_hp = self.hp;
        self.hp = (self.hp + amount).clamp(0, self.max_hp);
        self.hp - old_hp
    }

    /// Get the percentage of health remaining (0.0 to 1.0)
    pub fn health_percentage(&self) -> f32 {
        if self.max_hp == 0 {
            0.0
        } else {
            self.hp as f32 / self.max_hp as f32
        }
    }
}

/// Component defining an entity's combat capabilities and statistics
/// Used for attack resolution, damage calculation, and combat mechanics
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CombatStats {
    /// Accuracy bonus for attack rolls (higher = more likely to hit)
    pub accuracy: i32,
    /// Evasion bonus for defense rolls (higher = harder to hit)
    pub evasion: i32,
    /// Attack power for damage calculation
    pub attack: i32,
    /// Defense value that reduces incoming damage
    pub defense: i32,
    /// Minimum damage dealt on successful hit
    pub dmg_min: i32,
    /// Maximum damage dealt on successful hit
    pub dmg_max: i32,
}

impl CombatStats {
    /// Create new combat stats with validation
    /// Ensures dmg_min <= dmg_max and all stats are non-negative
    pub fn new(
        accuracy: i32,
        evasion: i32,
        attack: i32,
        defense: i32,
        dmg_min: i32,
        dmg_max: i32,
    ) -> Self {
        let (dmg_min, dmg_max) = if dmg_min <= dmg_max {
            (dmg_min, dmg_max)
        } else {
            (dmg_max, dmg_min)
        };

        Self {
            accuracy: accuracy.max(0),
            evasion: evasion.max(0),
            attack: attack.max(0),
            defense: defense.max(0),
            dmg_min: dmg_min.max(0),
            dmg_max: dmg_max.max(0),
        }
    }

    /// Calculate the base hit chance percentage (0-100)
    /// Higher accuracy vs evasion increases hit chance
    pub fn hit_chance(&self) -> i32 {
        let base = 50 + (self.accuracy - self.evasion) * 5;
        base.clamp(5, 95)
    }
}

/// Component tracking attack cooldown timing
/// Prevents entities from attacking too frequently
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct AttackCooldown {
    /// Tick number when the entity can attack again
    /// When current tick >= until_tick, attack is allowed
    pub until_tick: u64,
}

impl AttackCooldown {
    /// Create a new cooldown that expires at the specified tick
    pub fn new(until_tick: u64) -> Self {
        Self { until_tick }
    }

    /// Check if the cooldown has expired at the current tick
    pub fn is_ready(&self, current_tick: u64) -> bool {
        current_tick >= self.until_tick
    }

    /// Set the cooldown to expire after the specified number of ticks
    pub fn set_duration(&mut self, current_tick: u64, duration_ticks: u64) {
        self.until_tick = current_tick + duration_ticks;
    }
}

/// Marker component indicating an entity can participate in combat
/// Entities with this component can attack, be attacked, and use combat systems
#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Combatant;

/// Marker component indicating an entity is dead
/// Dead entities should not participate in combat, movement, or jobs
#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Dead;

/// Component for targeting other entities in combat
/// Lightweight pointer to the target entity for combat systems
#[derive(Component, Debug, Clone)]
pub struct Target {
    /// The entity being targeted
    pub entity: Entity,
}

impl Target {
    /// Create a new target component
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

// ============================================================================
// Construction MVP Components
// ============================================================================

/// Types of buildable structures that can be constructed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildableKind {
    /// Wall structure that blocks movement
    Wall,
    /// Floor tile that allows movement
    Floor,
    /// Door that can be opened and closed
    Door,
}

impl BuildableKind {
    /// Get the material cost for building this structure
    /// Returns a vector of (MaterialType, quantity) pairs
    pub fn material_cost(&self) -> Vec<(MaterialType, u32)> {
        match self {
            BuildableKind::Wall => vec![(MaterialType::Stone, 2)],
            BuildableKind::Floor => vec![(MaterialType::Wood, 1)],
            BuildableKind::Door => vec![(MaterialType::Wood, 2)],
        }
    }

    /// Get the tile kind this buildable becomes when completed
    pub fn resulting_tile(&self) -> crate::world::TileKind {
        match self {
            BuildableKind::Wall => crate::world::TileKind::Wall,
            BuildableKind::Floor => crate::world::TileKind::Floor,
            BuildableKind::Door => crate::world::TileKind::Floor, // Doors are entities on floor
        }
    }
}

/// Orientation for directional structures like walls and doors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    /// North facing (up)
    North,
    /// East facing (right)
    East,
    /// South facing (down)
    South,
    /// West facing (left)
    West,
}

impl Orientation {
    /// Get the opposite orientation
    pub fn opposite(&self) -> Self {
        match self {
            Orientation::North => Orientation::South,
            Orientation::East => Orientation::West,
            Orientation::South => Orientation::North,
            Orientation::West => Orientation::East,
        }
    }

    /// Get the direction vector for this orientation
    pub fn direction(&self) -> (i32, i32) {
        match self {
            Orientation::North => (0, -1),
            Orientation::East => (1, 0),
            Orientation::South => (0, 1),
            Orientation::West => (-1, 0),
        }
    }
}

/// Component representing a construction designation
/// Marks a location where something should be built
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionDesignation {
    /// What type of structure to build
    pub buildable: BuildableKind,
    /// Position where the structure should be built
    pub position: (i32, i32),
    /// Orientation for directional structures (None for omnidirectional)
    pub orientation: Option<Orientation>,
}

impl ConstructionDesignation {
    /// Create a new construction designation
    pub fn new(buildable: BuildableKind, position: (i32, i32)) -> Self {
        Self {
            buildable,
            position,
            orientation: None,
        }
    }

    /// Create a new construction designation with orientation
    pub fn with_orientation(
        buildable: BuildableKind,
        position: (i32, i32),
        orientation: Orientation,
    ) -> Self {
        Self {
            buildable,
            position,
            orientation: Some(orientation),
        }
    }

    /// Get the cells this designation occupies (for multi-cell placements)
    /// Currently all structures are single-cell, but this allows future expansion
    pub fn occupied_cells(&self) -> Vec<(i32, i32)> {
        vec![self.position]
    }
}

/// Component tracking materials reserved for a construction job
/// Prevents double-allocation of materials across multiple jobs
/// Note: Does not implement Serialize/Deserialize because it contains Entity references
/// that should be rebuilt on load rather than serialized
#[derive(Component, Debug, Clone)]
pub struct MaterialReservation {
    /// List of reserved item entities and their types
    pub reserved_items: Vec<(Entity, ItemType)>,
}

impl MaterialReservation {
    /// Create a new empty material reservation
    pub fn new() -> Self {
        Self {
            reserved_items: Vec::new(),
        }
    }

    /// Add a reserved item to this reservation
    pub fn add_item(&mut self, entity: Entity, item_type: ItemType) {
        self.reserved_items.push((entity, item_type));
    }

    /// Check if this reservation satisfies the required materials
    pub fn satisfies_requirements(&self, required: &[(MaterialType, u32)]) -> bool {
        for (material_type, required_count) in required {
            let available_count = self
                .reserved_items
                .iter()
                .filter(|(_, item_type)| material_type.accepts_item(*item_type))
                .count() as u32;

            if available_count < *required_count {
                return false;
            }
        }
        true
    }
}

impl Default for MaterialReservation {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker component for doors that can be opened/closed
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Door {
    /// Whether the door is currently open (allows passage)
    pub is_open: bool,
    /// Orientation of the door
    pub orientation: Orientation,
}

impl Door {
    /// Create a new closed door with the specified orientation
    pub fn new(orientation: Orientation) -> Self {
        Self {
            is_open: false,
            orientation,
        }
    }

    /// Toggle the door's open/closed state
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    /// Check if entities can pass through this door
    pub fn is_passable(&self) -> bool {
        self.is_open
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::entity::Entity;

    #[test]
    fn faction_hostility_rules() {
        let goblins = Faction::new(FactionKind::Goblins);
        let invaders = Faction::new(FactionKind::Invaders);
        let neutral = Faction::new(FactionKind::Neutral);

        // Goblins and Invaders are hostile to each other
        assert!(goblins.is_hostile_to(&invaders));
        assert!(invaders.is_hostile_to(&goblins));

        // Neutral entities are not hostile to anyone
        assert!(!neutral.is_hostile_to(&goblins));
        assert!(!neutral.is_hostile_to(&invaders));
        assert!(!neutral.is_hostile_to(&neutral));

        // Goblins are not hostile to themselves
        assert!(!goblins.is_hostile_to(&goblins));
    }

    #[test]
    fn health_creation_and_validation() {
        // Test normal creation
        let health = Health::new(75, 100);
        assert_eq!(health.hp, 75);
        assert_eq!(health.max_hp, 100);

        // Test clamping to max_hp
        let health = Health::new(150, 100);
        assert_eq!(health.hp, 100);
        assert_eq!(health.max_hp, 100);

        // Test clamping to 0
        let health = Health::new(-10, 100);
        assert_eq!(health.hp, 0);
        assert_eq!(health.max_hp, 100);

        // Test full health constructor
        let health = Health::full(50);
        assert_eq!(health.hp, 50);
        assert_eq!(health.max_hp, 50);
    }

    #[test]
    fn health_life_death_checks() {
        let mut health = Health::new(50, 100);

        // Should be alive
        assert!(health.is_alive());
        assert!(!health.is_dead());

        // Take damage but stay alive
        let damage_dealt = health.take_damage(30);
        assert_eq!(damage_dealt, 30);
        assert_eq!(health.hp, 20);
        assert!(health.is_alive());

        // Take lethal damage
        let damage_dealt = health.take_damage(50);
        assert_eq!(damage_dealt, 20);
        assert_eq!(health.hp, 0);
        assert!(health.is_dead());
        assert!(!health.is_alive());
    }

    #[test]
    fn health_healing() {
        let mut health = Health::new(20, 100);

        // Heal normally
        let healing_applied = health.heal(30);
        assert_eq!(healing_applied, 30);
        assert_eq!(health.hp, 50);

        // Heal beyond max_hp (should clamp)
        let healing_applied = health.heal(100);
        assert_eq!(healing_applied, 50);
        assert_eq!(health.hp, 100);
    }

    #[test]
    fn health_percentage() {
        let health = Health::new(75, 100);
        assert_eq!(health.health_percentage(), 0.75);

        let health = Health::new(0, 100);
        assert_eq!(health.health_percentage(), 0.0);

        let health = Health::new(100, 100);
        assert_eq!(health.health_percentage(), 1.0);

        // Edge case: max_hp = 0
        let health = Health::new(0, 0);
        assert_eq!(health.health_percentage(), 0.0);
    }

    #[test]
    fn combat_stats_validation() {
        // Test normal creation
        let stats = CombatStats::new(10, 5, 15, 8, 20, 30);
        assert_eq!(stats.accuracy, 10);
        assert_eq!(stats.evasion, 5);
        assert_eq!(stats.attack, 15);
        assert_eq!(stats.defense, 8);
        assert_eq!(stats.dmg_min, 20);
        assert_eq!(stats.dmg_max, 30);

        // Test negative stat clamping
        let stats = CombatStats::new(-5, -3, -10, -2, 15, 25);
        assert_eq!(stats.accuracy, 0);
        assert_eq!(stats.evasion, 0);
        assert_eq!(stats.attack, 0);
        assert_eq!(stats.defense, 0);
        assert_eq!(stats.dmg_min, 15);
        assert_eq!(stats.dmg_max, 25);

        // Test damage range validation
        let stats = CombatStats::new(5, 3, 10, 5, 30, 20);
        assert_eq!(stats.dmg_min, 20);
        assert_eq!(stats.dmg_max, 30);
    }

    #[test]
    fn combat_stats_hit_chance() {
        let stats = CombatStats::new(10, 5, 15, 8, 20, 30);

        // Base hit chance should be 50 + (10 - 5) * 5 = 75
        assert_eq!(stats.hit_chance(), 75);

        // High accuracy vs low evasion
        let stats = CombatStats::new(20, 0, 15, 8, 20, 30);
        assert_eq!(stats.hit_chance(), 95); // Clamped to max

        // Low accuracy vs high evasion
        let stats = CombatStats::new(0, 20, 15, 8, 20, 30);
        assert_eq!(stats.hit_chance(), 5); // Clamped to min
    }

    #[test]
    fn attack_cooldown_timing() {
        let mut cooldown = AttackCooldown::new(100);

        // Should not be ready before the tick
        assert!(!cooldown.is_ready(50));
        assert!(!cooldown.is_ready(99));

        // Should be ready at and after the tick
        assert!(cooldown.is_ready(100));
        assert!(cooldown.is_ready(150));

        // Test setting duration
        cooldown.set_duration(200, 50);
        assert_eq!(cooldown.until_tick, 250);
        assert!(!cooldown.is_ready(200));
        assert!(cooldown.is_ready(250));
    }

    #[test]
    fn target_creation() {
        let entity = Entity::from_raw(42);
        let target = Target::new(entity);
        assert_eq!(target.entity, entity);
    }

    // ========================================================================
    // Construction MVP Tests
    // ========================================================================

    #[test]
    fn material_type_accepts_item() {
        // Stone material accepts stone items
        assert!(MaterialType::Stone.accepts_item(ItemType::Stone));
        assert!(MaterialType::Stone.accepts_item(ItemType::StoneBlock));
        assert!(!MaterialType::Stone.accepts_item(ItemType::WoodPlank));

        // Wood material accepts wood items
        assert!(MaterialType::Wood.accepts_item(ItemType::WoodPlank));
        assert!(!MaterialType::Wood.accepts_item(ItemType::Stone));
        assert!(!MaterialType::Wood.accepts_item(ItemType::StoneBlock));
    }

    #[test]
    fn material_type_accepted_items() {
        let stone_items = MaterialType::Stone.accepted_items();
        assert!(stone_items.contains(&ItemType::Stone));
        assert!(stone_items.contains(&ItemType::StoneBlock));

        let wood_items = MaterialType::Wood.accepted_items();
        assert!(wood_items.contains(&ItemType::WoodPlank));
    }

    #[test]
    fn buildable_material_costs() {
        // Wall requires 2 stone
        let wall_cost = BuildableKind::Wall.material_cost();
        assert_eq!(wall_cost, vec![(MaterialType::Stone, 2)]);

        // Floor requires 1 wood
        let floor_cost = BuildableKind::Floor.material_cost();
        assert_eq!(floor_cost, vec![(MaterialType::Wood, 1)]);

        // Door requires 2 wood
        let door_cost = BuildableKind::Door.material_cost();
        assert_eq!(door_cost, vec![(MaterialType::Wood, 2)]);
    }

    #[test]
    fn buildable_resulting_tiles() {
        use crate::world::TileKind;

        assert_eq!(BuildableKind::Wall.resulting_tile(), TileKind::Wall);
        assert_eq!(BuildableKind::Floor.resulting_tile(), TileKind::Floor);
        assert_eq!(BuildableKind::Door.resulting_tile(), TileKind::Floor);
    }

    #[test]
    fn orientation_direction() {
        assert_eq!(Orientation::North.direction(), (0, -1));
        assert_eq!(Orientation::East.direction(), (1, 0));
        assert_eq!(Orientation::South.direction(), (0, 1));
        assert_eq!(Orientation::West.direction(), (-1, 0));
    }

    #[test]
    fn orientation_opposite() {
        assert_eq!(Orientation::North.opposite(), Orientation::South);
        assert_eq!(Orientation::East.opposite(), Orientation::West);
        assert_eq!(Orientation::South.opposite(), Orientation::North);
        assert_eq!(Orientation::West.opposite(), Orientation::East);
    }

    #[test]
    fn construction_designation_creation() {
        let designation = ConstructionDesignation::new(BuildableKind::Wall, (5, 10));
        assert_eq!(designation.buildable, BuildableKind::Wall);
        assert_eq!(designation.position, (5, 10));
        assert_eq!(designation.orientation, None);

        let designation_with_orient = ConstructionDesignation::with_orientation(
            BuildableKind::Door,
            (3, 7),
            Orientation::North,
        );
        assert_eq!(designation_with_orient.buildable, BuildableKind::Door);
        assert_eq!(designation_with_orient.position, (3, 7));
        assert_eq!(
            designation_with_orient.orientation,
            Some(Orientation::North)
        );
    }

    #[test]
    fn construction_designation_occupied_cells() {
        let designation = ConstructionDesignation::new(BuildableKind::Wall, (5, 10));
        let cells = designation.occupied_cells();
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0], (5, 10));
    }

    #[test]
    fn material_reservation() {
        let mut reservation = MaterialReservation::new();
        assert!(reservation.reserved_items.is_empty());

        let entity1 = Entity::from_raw(1);
        let entity2 = Entity::from_raw(2);

        reservation.add_item(entity1, ItemType::Stone);
        reservation.add_item(entity2, ItemType::Stone);

        assert_eq!(reservation.reserved_items.len(), 2);
    }

    #[test]
    fn material_reservation_satisfies_requirements() {
        let mut reservation = MaterialReservation::new();

        // Add 2 stone items
        reservation.add_item(Entity::from_raw(1), ItemType::Stone);
        reservation.add_item(Entity::from_raw(2), ItemType::Stone);

        // Should satisfy requirement for 2 stone
        assert!(reservation.satisfies_requirements(&[(MaterialType::Stone, 2)]));

        // Should not satisfy requirement for 3 stone
        assert!(!reservation.satisfies_requirements(&[(MaterialType::Stone, 3)]));

        // Should not satisfy requirement for wood
        assert!(!reservation.satisfies_requirements(&[(MaterialType::Wood, 1)]));
    }

    #[test]
    fn door_state() {
        let mut door = Door::new(Orientation::North);
        assert_eq!(door.orientation, Orientation::North);
        assert!(!door.is_open);
        assert!(!door.is_passable());

        door.toggle();
        assert!(door.is_open);
        assert!(door.is_passable());

        door.toggle();
        assert!(!door.is_open);
        assert!(!door.is_passable());
    }
}
