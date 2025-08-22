//! gc_core: Core simulation engine for Goblin Camp
//! - ECS world, systems and schedules
//! - Data-driven definitions (entities, items, biomes)
//! - Job system and AI behaviors
//! - Pathfinding and map representation

/// Action logging for lifecycle events
#[derive(bevy_ecs::prelude::Resource, Default, Debug)]
pub struct ActionLog {
    pub events: Vec<String>,
}

impl ActionLog {
    pub fn log(&mut self, event: String) {
        self.events.push(event);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

pub mod prelude {
    pub use crate::components::*;
    pub use crate::designations::*;
    pub use crate::fov::*;
    pub use crate::jobs::*;
    pub use crate::mapgen::*;
    pub use crate::path::*;
    pub use crate::save::*;
    pub use crate::systems::*;
    pub use crate::world::*;
    pub use crate::ActionLog;
}

pub mod components;
pub mod designations;
pub mod fov;
pub mod jobs;
pub mod mapgen;
pub mod path;
pub mod save;
pub mod systems;
pub mod world;

// Removed empty internal tests module; tests live in `tests/` integration folder.
