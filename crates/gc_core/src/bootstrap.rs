//! Bootstrap utilities for building a standard world and schedule
//! shared by both CLI demos and the TUI. Keeping this in `gc_core`
//! ensures a single source of truth for setup and determinism.

use bevy_ecs::prelude::*;
use rand::Rng;

use crate::designations;
use crate::jobs;
use crate::prelude::*;
use crate::stockpiles::StockpileBundle;
use crate::systems;

/// Options controlling what entities/resources to include when building a world.
#[derive(Debug, Clone, Copy)]
pub struct WorldOptions {
    /// If true, spawns a small demo scene with a miner, a carrier, and a stockpile.
    pub populate_demo_scene: bool,
    /// Initial time tick duration in ms (fixed-step). Defaults to 100.
    pub tick_ms: u64,
}

impl Default for WorldOptions {
    fn default() -> Self {
        Self {
            populate_demo_scene: false,
            tick_ms: 100,
        }
    }
}

/// Build a standard world with deterministic RNG, generated map, job systems,
/// and optional demo population. This is the canonical entry point for shells.
pub fn build_standard_world(width: u32, height: u32, seed: u64, opts: WorldOptions) -> World {
    let mut world = World::new();

    // Deterministic RNG first so any subsequent randomness draws from it
    world.insert_resource(systems::DeterministicRng::new(seed));

    // Map generation via centralized RNG
    let gen = MapGenerator::new();
    let mapgen_seed = {
        let mut rng = world.resource_mut::<systems::DeterministicRng>();
        rng.mapgen_rng.gen::<u32>()
    };
    let map = gen.generate(width, height, mapgen_seed);
    world.insert_resource(map);

    // Core resources
    world.insert_resource(JobBoard::default());
    world.insert_resource(jobs::ItemSpawnQueue::default());
    world.insert_resource(jobs::ActiveJobs::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    world.insert_resource(systems::Time::new(opts.tick_ms));
    // Default to stepwise movement to avoid teleporting agents/items in demos
    world.insert_resource(systems::MovementConfig::default());

    if opts.populate_demo_scene {
        // Miner
        world.spawn((
            Name("Grak".into()),
            Position(5, 5),
            Velocity(0, 0),
            Miner,
            AssignedJob::default(),
            VisionRadius(8),
        ));

        // Carrier
        world.spawn((
            Name("Urok".into()),
            Position(5, 5),
            Velocity(0, 0),
            Carrier,
            Inventory::default(),
            AssignedJob::default(),
            VisionRadius(8),
        ));

        // Stockpile zone centered around (10,10)
        world
            .spawn(StockpileBundle::new(9, 9, 11, 11))
            .insert(Name("Stockpile".into()));
    }

    world
}

/// Build the default simulation schedule used by shells for demos/play.
pub fn build_default_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        systems::movement,
        systems::confine_to_map,
        (
            designations::designation_dedup_system,
            designations::designation_to_jobs_system,
            jobs::job_assignment_system,
        )
            .chain(),
        (
            jobs::mine_job_execution_system,
            systems::hauling_execution_system,
            systems::auto_haul_system,
        ),
        systems::advance_time,
    ));
    schedule
}
