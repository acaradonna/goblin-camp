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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    /// Stone items created from mining operations
    /// These are the primary resource produced by mining wall tiles
    Stone,
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
}
