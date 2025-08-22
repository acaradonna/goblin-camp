use bevy_ecs::prelude::*;
use gc_core::prelude::*;
use gc_core::{designations, jobs, world::TileKind};

#[test]
fn mine_job_converts_wall_to_floor() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(10, 10));
    world.insert_resource(jobs::JobBoard::default());
    world.insert_resource(jobs::ItemSpawnQueue::default());
    world.insert_resource(jobs::ActiveJobs::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });

    // Set up a wall at position (5, 5)
    {
        let mut map = world.resource_mut::<GameMap>();
        map.set_tile(5, 5, TileKind::Wall);
    }

    // Create a miner at the wall position
    world.spawn((
        Name("TestMiner".into()),
        Position(5, 5),
        Miner,
        AssignedJob::default(),
    ));

    // Create a mine designation which will auto-spawn a job
    world.spawn((
        designations::MineDesignation,
        Position(5, 5),
        DesignationLifecycle::default(),
    ));

    // Verify initial state
    assert_eq!(
        world.resource::<GameMap>().get_tile(5, 5),
        Some(TileKind::Wall)
    );
    assert_eq!(world.resource::<jobs::ItemSpawnQueue>().requests.len(), 0);

    // Run the systems
    let mut schedule = Schedule::default();
    schedule.add_systems((
        (
            designations::designation_dedup_system,
            designations::designation_to_jobs_system,
        )
            .chain(),
        jobs::mine_job_assignment_system,
        jobs::mine_job_execution_system,
    ));

    // Run multiple steps like the CLI demo
    for _ in 0..5 {
        schedule.run(&mut world);
    }

    // Verify the wall was converted to floor
    assert_eq!(
        world.resource::<GameMap>().get_tile(5, 5),
        Some(TileKind::Floor)
    );

    // Verify a stone item was spawned
    let item_queue = world.resource::<jobs::ItemSpawnQueue>();
    assert_eq!(item_queue.requests.len(), 1);
    assert_eq!(item_queue.requests[0].item_type, jobs::ItemType::Stone);
    assert_eq!(item_queue.requests[0].position, (5, 5));
}

#[test]
fn mine_job_does_not_affect_non_wall_tiles() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(10, 10));
    world.insert_resource(jobs::JobBoard::default());
    world.insert_resource(jobs::ItemSpawnQueue::default());
    world.insert_resource(jobs::ActiveJobs::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });

    // Set up a floor at position (5, 5) - not a wall
    {
        let mut map = world.resource_mut::<GameMap>();
        map.set_tile(5, 5, TileKind::Floor);
    }

    // Create a miner at the position
    world.spawn((
        Name("TestMiner".into()),
        Position(5, 5),
        Miner,
        AssignedJob::default(),
    ));

    // Create a mine designation which will auto-spawn a job
    world.spawn((
        designations::MineDesignation,
        Position(5, 5),
        DesignationLifecycle::default(),
    ));

    // Verify initial state
    assert_eq!(
        world.resource::<GameMap>().get_tile(5, 5),
        Some(TileKind::Floor)
    );
    assert_eq!(world.resource::<jobs::ItemSpawnQueue>().requests.len(), 0);

    // Run the systems
    let mut schedule = Schedule::default();
    schedule.add_systems((
        (
            designations::designation_dedup_system,
            designations::designation_to_jobs_system,
        )
            .chain(),
        jobs::mine_job_assignment_system,
        jobs::mine_job_execution_system,
    ));

    // Run multiple steps like the CLI demo
    for _ in 0..5 {
        schedule.run(&mut world);
    }

    // Verify the floor tile is unchanged
    assert_eq!(
        world.resource::<GameMap>().get_tile(5, 5),
        Some(TileKind::Floor)
    );

    // Verify no items were spawned since there was no wall to mine
    let item_queue = world.resource::<jobs::ItemSpawnQueue>();
    assert_eq!(item_queue.requests.len(), 0);
}

#[test]
fn miner_gets_assigned_mine_jobs() {
    let mut world = World::new();
    world.insert_resource(GameMap::new(10, 10));
    world.insert_resource(jobs::JobBoard::default());
    world.insert_resource(jobs::ItemSpawnQueue::default());
    world.insert_resource(jobs::ActiveJobs::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });

    // Create a miner
    let miner_entity = world
        .spawn((
            Name("TestMiner".into()),
            Position(5, 5),
            Miner,
            AssignedJob::default(),
        ))
        .id();

    // Create a mine designation which will auto-spawn a job
    world.spawn((
        designations::MineDesignation,
        Position(5, 5),
        DesignationLifecycle::default(),
    ));

    // Verify miner starts with no job
    let assigned = world.get::<AssignedJob>(miner_entity).unwrap();
    assert!(assigned.0.is_none());

    // Run job creation and assignment systems
    let mut schedule = Schedule::default();
    schedule.add_systems((
        (
            designations::designation_dedup_system,
            designations::designation_to_jobs_system,
        )
            .chain(),
        jobs::mine_job_assignment_system,
    ));

    // Run multiple steps like the CLI demo
    for _ in 0..5 {
        schedule.run(&mut world);
    }

    // Verify miner got assigned a job
    let assigned = world.get::<AssignedJob>(miner_entity).unwrap();
    assert!(assigned.0.is_some());
}
