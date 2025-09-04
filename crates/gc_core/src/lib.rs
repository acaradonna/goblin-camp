//! # Goblin Camp Core Simulation Engine
//!
//! `gc_core` is the heart of the Goblin Camp simulation, providing:
//!
//! - **ECS Architecture**: Built on Bevy ECS for high-performance, data-oriented design
//! - **Deterministic Simulation**: Fixed-step timing and seeded RNG for reproducible results  
//! - **Spatial Systems**: 2D tile-based world with pathfinding and field-of-view
//! - **Job System**: Hierarchical task assignment for mining, hauling, and construction
//! - **Item Management**: Full entity-based items with spatial simulation
//! - **Save/Load**: JSON serialization with versioning support
//!
//! ## Architecture Overview
//!
//! The simulation follows Entity-Component-System (ECS) principles:
//!
//! - **Entities**: Goblins, items, designations, and world features
//! - **Components**: Pure data (Position, Inventory, Job assignments)  
//! - **Systems**: Logic that operates on component data (movement, mining, hauling)
//! - **Resources**: Global state (GameMap, JobBoard, Time)
//!
//! ## Module Organization
//!
//! - [`components`]: All ECS components for entities and spatial data
//! - [`systems`]: Core simulation systems and deterministic time management
//! - [`jobs`]: Job board, assignment, and execution systems
//! - [`world`]: Spatial representation, tiles, and map management
//! - [`designations`]: Player input system for marking mining/construction areas
//! - [`stockpiles`]: Storage zones and item organization systems
//! - [`path`]: A* pathfinding with caching and obstacle avoidance
//! - [`fov`]: Field-of-view and line-of-sight calculations
//! - [`mapgen`]: Procedural terrain generation
//! - [`save`]: World serialization and persistence
//! - [`inventory`]: Item carrying and storage systems
//!
//! ## Usage Example
//!
//! ```rust
//! use bevy_ecs::prelude::*;
//! use gc_core::prelude::*;
//! use gc_core::systems;
//!
//! let mut world = World::new();
//!
//! // Initialize core resources
//! world.insert_resource(GameMap::new(50, 50));
//! world.insert_resource(JobBoard::default());
//! world.insert_resource(ActiveJobs::default());
//! world.insert_resource(systems::Time::new(16)); // 16ms per tick
//! world.insert_resource(systems::DeterministicRng::new(42));
//!
//! // Spawn a goblin miner
//! world.spawn((
//!     Name("Grok".into()),
//!     Position(10, 10),
//!     Goblin,
//!     Miner,
//!     AssignedJob::default(),
//! ));
//!
//! // Create and run simulation schedule
//! let mut schedule = Schedule::default();
//! schedule.add_systems((
//!     systems::movement,
//!     systems::mining_execution_system,
//!     systems::advance_time,
//! ));
//!
//! // Run simulation steps
//! for _ in 0..100 {
//!     schedule.run(&mut world);
//! }
//! ```

/// Action logging for lifecycle events and debugging
/// Provides a centralized log for tracking significant simulation events
/// such as job assignments, mining operations, and item movements
#[derive(bevy_ecs::prelude::Resource, Default, Debug)]
pub struct ActionLog {
    /// Chronological list of logged events
    pub events: Vec<String>,
}

impl ActionLog {
    /// Add a new event to the log with automatic timestamping
    /// Events are stored in chronological order
    pub fn log(&mut self, event: String) {
        self.events.push(event);
    }

    /// Clear all logged events
    /// Useful for resetting between simulation runs or tests
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

/// Convenience prelude module that re-exports commonly used types
/// Import this module to get access to the most frequently used
/// components, systems, and resources in a single use statement
///
/// ```rust
/// use gc_core::prelude::*;
/// // Now you have access to Position, GameMap, JobBoard, etc.
/// ```
pub mod prelude {
    pub use crate::bootstrap::*;
    pub use crate::components::*;
    pub use crate::designations::*;
    pub use crate::fov::*;
    pub use crate::inventory::*;
    pub use crate::jobs::*;
    pub use crate::mapgen::*;
    pub use crate::path::*;
    pub use crate::recipes::*;
    pub use crate::save::*;
    pub use crate::stockpiles::*;
    pub use crate::systems::*;
    pub use crate::world::*;
    pub use crate::ActionLog;
}

// Public module declarations
// Each module contains related functionality for specific simulation aspects

/// ECS components for entities, spatial data, and game state
pub mod components;
/// Player designation system for marking areas for mining, construction, etc.
pub mod designations;
/// Field-of-view and line-of-sight calculations
pub mod fov;
/// Item carrying and inventory management systems
pub mod inventory;
/// Job board, assignment, and execution systems  
pub mod jobs;
/// Procedural terrain and world generation
pub mod mapgen;
/// A* pathfinding with caching and optimization
pub mod path;
/// Recipe registry and crafting system for workshops
pub mod recipes;
/// World serialization and save/load functionality
pub mod save;
/// Storage zones and item organization systems
pub mod stockpiles;
/// Core simulation systems and time management
pub mod systems;
/// Spatial world representation and tile management
pub mod world;

/// Bootstrap helpers for building standard worlds and schedules shared by CLI/TUI
pub mod bootstrap;

// Removed empty internal tests module; tests live in `tests/` integration folder.
