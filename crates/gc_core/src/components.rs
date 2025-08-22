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
