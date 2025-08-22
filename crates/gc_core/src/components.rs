use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug)]
pub struct Goblin;

#[derive(Component, Debug)]
pub struct JobQueue;

#[derive(Component, Debug)]
pub struct Carrier;

#[derive(Component, Debug)]
pub struct Miner;

#[derive(Component, Debug, Default)]
pub struct AssignedJob(pub Option<crate::jobs::JobId>);

#[derive(Component, Debug)]
pub struct VisionRadius(pub i32);

/// Represents the lifecycle state of a designation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum DesignationState {
    /// Active designation ready to be processed
    #[default]
    Active,
    /// Duplicate designation that should be ignored
    Ignored,
    /// Designation that has been consumed/processed (for future use)
    Consumed,
}

/// Component to track the lifecycle state of designations
#[derive(Component, Debug, Default)]
pub struct DesignationLifecycle(pub DesignationState);

/// Types of items that can exist in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    Stone,
}

/// Component representing an item entity that can be spawned, carried, and placed
#[derive(Component, Debug)]
pub struct Item {
    pub item_type: ItemType,
}

impl Item {
    pub fn stone() -> Self {
        Self {
            item_type: ItemType::Stone,
        }
    }
}

/// Marker component indicating that an item can be carried/hauled by agents
#[derive(Component, Debug)]
pub struct Carriable;

/// Component representing a stone item
#[derive(Component, Debug)]
pub struct Stone;

/// Inventory component for agents to carry a single item (MVP)
/// Holds an optional entity reference to the carried item
#[derive(Component, Debug, Default)]
pub struct Inventory(pub Option<Entity>);

/// Defines rectangular bounds for a zone
#[derive(Component, Debug, Clone)]
pub struct ZoneBounds {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl ZoneBounds {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// Check if a position is within the zone bounds (inclusive)
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    /// Get the center point of the zone
    pub fn center(&self) -> (i32, i32) {
        ((self.min_x + self.max_x) / 2, (self.min_y + self.max_y) / 2)
    }
}

/// Component marking a stockpile zone that can accept items
#[derive(Component, Debug)]
pub struct Stockpile {
    /// Items accepted by this stockpile (None = accepts all)
    pub accepts: Option<Vec<ItemType>>,
}
