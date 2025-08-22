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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
