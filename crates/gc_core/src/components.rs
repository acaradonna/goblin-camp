use bevy_ecs::prelude::*;

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
