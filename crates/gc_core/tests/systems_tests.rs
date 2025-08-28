// Additional systems tests to improve coverage

use bevy_ecs::prelude::*;
use gc_core::prelude::*;
use gc_core::{designations, jobs, systems};
use rand::Rng;

#[test]
fn time_resource_creation() {
    let time = systems::Time::new(16);
    assert_eq!(time.ticks, 0);
    assert_eq!(time.tick_ms, 16);
}

#[test]
fn time_resource_with_different_tick_rates() {
    let time_fast = systems::Time::new(8);
    let time_slow = systems::Time::new(33);

    assert_eq!(time_fast.tick_ms, 8);
    assert_eq!(time_slow.tick_ms, 33);
}

#[test]
fn deterministic_rng_creation() {
    let rng = systems::DeterministicRng::new(42);
    assert_eq!(rng.master_seed, 42);
}

#[test]
fn deterministic_rng_different_seeds() {
    let rng1 = systems::DeterministicRng::new(42);
    let rng2 = systems::DeterministicRng::new(123);

    assert_ne!(rng1.master_seed, rng2.master_seed);
}

#[test]
fn advance_time_system() {
    let mut world = World::new();
    world.insert_resource(systems::Time::new(16));

    let mut schedule = Schedule::default();
    schedule.add_systems(systems::advance_time);

    // Initial time
    {
        let time = world.get_resource::<systems::Time>().unwrap();
        assert_eq!(time.ticks, 0);
    }

    // Advance time
    schedule.run(&mut world);

    {
        let time = world.get_resource::<systems::Time>().unwrap();
        assert_eq!(time.ticks, 1);
    }

    // Advance again
    schedule.run(&mut world);

    {
        let time = world.get_resource::<systems::Time>().unwrap();
        assert_eq!(time.ticks, 2);
    }
}

#[test]
fn movement_system_basic() {
    let mut world = World::new();

    // Create an entity with position and velocity
    let entity = world.spawn((Position(5, 5), Velocity(1, 1), Goblin)).id();

    let mut schedule = Schedule::default();
    schedule.add_systems(systems::movement);

    // Run movement system
    schedule.run(&mut world);

    // Check that position moved by velocity
    let pos = world.get::<Position>(entity).unwrap();
    assert_eq!(*pos, Position(6, 6)); // Should have moved by (1, 1)
}

#[test]
fn movement_system_no_velocity() {
    let mut world = World::new();

    // Create an entity without velocity
    let entity = world.spawn((Position(5, 5), Goblin)).id();

    let mut schedule = Schedule::default();
    schedule.add_systems(systems::movement);

    // Run movement system
    schedule.run(&mut world);

    // Position should remain unchanged (no velocity component)
    let pos = world.get::<Position>(entity).unwrap();
    assert_eq!(*pos, Position(5, 5));
}

#[test]
fn confine_to_map_system() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(10, 10));

    // Create an entity outside map bounds
    let entity = world
        .spawn((
            Position(-1, 15), // Outside bounds
            Goblin,
        ))
        .id();

    let mut schedule = Schedule::default();
    schedule.add_systems(systems::confine_to_map);

    // Run system
    schedule.run(&mut world);

    // Position should be confined within map bounds
    let pos = world.get::<Position>(entity).unwrap();
    assert!(pos.0 >= 0 && pos.0 < 10);
    assert!(pos.1 >= 0 && pos.1 < 10);
}

#[test]
fn designation_to_jobs_system_basic() {
    let mut world = World::new();
    world.insert_resource(JobBoard::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    world.insert_resource(systems::DeterministicRng::new(42));

    // Create a mine designation
    world.spawn((
        designations::MineDesignation,
        Position(10, 15),
        DesignationLifecycle::default(),
    ));

    let mut schedule = Schedule::default();
    schedule.add_systems(designations::designation_to_jobs_system);

    // Run system
    schedule.run(&mut world);

    // Check that a job was created on the job board
    let job_board = world.get_resource::<JobBoard>().unwrap();
    assert!(!job_board.0.is_empty());
}

#[test]
fn job_assignment_system_basic() {
    let mut world = World::new();
    world.insert_resource(JobBoard::default());
    world.insert_resource(systems::DeterministicRng::new(42));
    world.insert_resource(jobs::ActiveJobs::default());

    // Create a job manually
    {
        let mut job_board = world.get_resource_mut::<JobBoard>().unwrap();
        let mut rng = systems::DeterministicRng::new(42);
        let mut bytes = [0u8; 16];
        rng.job_rng.fill(&mut bytes);
        let job_id = JobId(uuid::Uuid::from_bytes(bytes));
        job_board.0.push(Job {
            id: job_id,
            kind: JobKind::Mine { x: 10, y: 10 },
        });
    }

    // Create a miner
    let miner = world
        .spawn((Miner, Position(9, 9), AssignedJob::default(), Goblin))
        .id();

    let mut schedule = Schedule::default();
    schedule.add_systems(jobs::job_assignment_system);

    // Run system
    schedule.run(&mut world);

    // Check that miner got assigned
    let assigned_job = world.get::<AssignedJob>(miner).unwrap();
    assert!(assigned_job.0.is_some());
}

#[test]
fn mining_execution_system_basic() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(20, 20));
    world.insert_resource(jobs::ActiveJobs::default());
    world.insert_resource(jobs::ItemSpawnQueue::default());

    // Set up a wall tile to mine
    {
        let mut map = world.get_resource_mut::<GameMap>().unwrap();
        map.set_tile(10, 10, TileKind::Wall);
    }

    // Create a miner with an active job
    let mut rng = systems::DeterministicRng::new(42);
    let mut bytes = [0u8; 16];
    rng.job_rng.fill(&mut bytes);
    let job_id = JobId(uuid::Uuid::from_bytes(bytes));

    let _miner = world
        .spawn((Miner, Position(10, 10), AssignedJob(Some(job_id)), Goblin))
        .id();

    // Add the job to active jobs
    {
        let mut active_jobs = world.get_resource_mut::<jobs::ActiveJobs>().unwrap();
        active_jobs.jobs.insert(
            job_id,
            Job {
                id: job_id,
                kind: JobKind::Mine { x: 10, y: 10 },
            },
        );
    }

    let mut schedule = Schedule::default();
    schedule.add_systems(systems::mining_execution_system);

    // Run system
    schedule.run(&mut world);

    // Check that wall was mined (turned to floor)
    let map = world.get_resource::<GameMap>().unwrap();
    assert_eq!(map.get_tile(10, 10), Some(TileKind::Floor));
}

#[test]
fn hauling_execution_system_basic() {
    let mut world = World::new();
    world.insert_resource(jobs::ActiveJobs::default());

    // Create an item on the ground
    let _item = world
        .spawn((
            Item {
                item_type: ItemType::Stone,
            },
            Position(5, 5),
        ))
        .id();

    // Create a carrier
    let mut rng = systems::DeterministicRng::new(42);
    let mut bytes = [0u8; 16];
    rng.job_rng.fill(&mut bytes);
    let job_id = JobId(uuid::Uuid::from_bytes(bytes));

    let _carrier = world
        .spawn((
            Carrier,
            Position(5, 5), // Same position as item
            AssignedJob(Some(job_id)),
            Inventory::default(),
            Goblin,
        ))
        .id();

    // Add hauling job
    {
        let mut active_jobs = world.get_resource_mut::<jobs::ActiveJobs>().unwrap();
        active_jobs.jobs.insert(
            job_id,
            Job {
                id: job_id,
                kind: JobKind::Haul {
                    from: (5, 5),
                    to: (10, 10),
                },
            },
        );
    }

    let mut schedule = Schedule::default();
    schedule.add_systems(systems::hauling_execution_system);

    // Run system
    schedule.run(&mut world);

    // The hauling system ran without errors
    // Note: Actual hauling behavior depends on complex logic
    // We're mainly testing that the system can run
}

#[test]
fn deterministic_rng_stream_independence() {
    let mut rng1 = systems::DeterministicRng::new(42);
    let mut rng2 = systems::DeterministicRng::new(42);

    // Generate some numbers from mapgen stream on first RNG
    use rand::Rng;
    let _val1 = rng1.mapgen_rng.gen::<u32>();
    let _val2 = rng1.mapgen_rng.gen::<u32>();

    // Job stream should still be identical between the two RNGs
    let job_val1 = rng1.job_rng.gen::<u32>();
    let job_val2 = rng2.job_rng.gen::<u32>();

    assert_eq!(job_val1, job_val2);
}

#[test]
fn mining_job_assignment_system() {
    let mut world = World::new();
    world.insert_resource(JobBoard::default());
    world.insert_resource(jobs::ActiveJobs::default());
    world.insert_resource(systems::DeterministicRng::new(42));

    // Create a mining job
    {
        let mut job_board = world.get_resource_mut::<JobBoard>().unwrap();
        let mut rng = systems::DeterministicRng::new(42);
        let mut bytes = [0u8; 16];
        rng.job_rng.fill(&mut bytes);
        let job_id = JobId(uuid::Uuid::from_bytes(bytes));
        job_board.0.push(Job {
            id: job_id,
            kind: JobKind::Mine { x: 5, y: 5 },
        });
    }

    // Create a miner
    let _miner = world
        .spawn((Miner, Position(5, 5), AssignedJob::default(), Goblin))
        .id();

    let mut schedule = Schedule::default();
    schedule.add_systems(jobs::mining_job_assignment_system);

    // Run system (should assign the job)
    schedule.run(&mut world);

    // Verify active jobs were created
    let active_jobs = world.get_resource::<jobs::ActiveJobs>().unwrap();
    assert!(!active_jobs.jobs.is_empty());
}
