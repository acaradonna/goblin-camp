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
